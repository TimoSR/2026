use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

const MAGIC: &[u8; 8] = b"MLBIN001";
const DTYPE_F32: u8 = 1;

#[derive(Debug, Clone)]
struct Tensor {
    dtype: u8,
    shape: Vec<u64>,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
struct TensorMeta {
    name: String,
    dtype: u8,
    shape: Vec<u64>,
    offset: u64,
    len: u64,
    checksum: u64,
}

#[derive(Debug, Clone)]
struct Model {
    name: String,
    model_type: String,
    tensors: HashMap<String, Tensor>,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("create-demo") => {
            let output = args.get(2).map(String::as_str).unwrap_or("demo.mlbin");
            let model = create_demo_model();
            write_model(output, &model)?;
            println!("created {output}");
        }
        Some("inspect") => {
            let file = required_arg(&args, 2, "file")?;
            let model = read_model(file)?;
            inspect(&model);
        }
        Some("infer") => {
            let file = required_arg(&args, 2, "file")?;
            let input = parse_f32_args(&args[3..])?;
            let model = read_model(file)?;
            let output = infer_dense_relu(&model, &input)?;
            println!("{output:?}");
        }
        _ => {
            eprintln!(
                "Usage:\n  mlbin create-demo [output.mlbin]\n  mlbin inspect <file.mlbin>\n  mlbin infer <file.mlbin> <f32> <f32> ..."
            );
        }
    }

    Ok(())
}

fn create_demo_model() -> Model {
    let mut tensors = HashMap::new();

    tensors.insert(
        "dense.weight".to_string(),
        Tensor {
            dtype: DTYPE_F32,
            shape: vec![3, 2],
            bytes: f32s_to_le_bytes(&[0.42, 1.31, -0.18, 0.77, 0.90, -0.35]),
        },
    );

    tensors.insert(
        "dense.bias".to_string(),
        Tensor {
            dtype: DTYPE_F32,
            shape: vec![3],
            bytes: f32s_to_le_bytes(&[0.10, -0.20, 0.05]),
        },
    );

    Model {
        name: "demo_dense_relu".to_string(),
        model_type: "dense_relu".to_string(),
        tensors,
    }
}

fn write_model<P: AsRef<Path>>(path: P, model: &Model) -> Result<(), String> {
    let mut tensor_items: Vec<(&String, &Tensor)> = model.tensors.iter().collect();
    tensor_items.sort_by(|a, b| a.0.cmp(b.0));

    let mut metas = Vec::new();
    let mut blob = Vec::new();

    for (name, tensor) in tensor_items {
        validate_tensor(tensor)?;

        let offset = blob.len() as u64;
        blob.extend_from_slice(&tensor.bytes);

        metas.push(TensorMeta {
            name: name.clone(),
            dtype: tensor.dtype,
            shape: tensor.shape.clone(),
            offset,
            len: tensor.bytes.len() as u64,
            checksum: fnv1a64(&tensor.bytes),
        });
    }

    let mut meta_bytes = Vec::new();
    write_string(&mut meta_bytes, &model.name)?;
    write_string(&mut meta_bytes, &model.model_type)?;
    write_u32(&mut meta_bytes, metas.len() as u32)?;

    for meta in &metas {
        write_string(&mut meta_bytes, &meta.name)?;
        write_u8(&mut meta_bytes, meta.dtype)?;
        write_u32(&mut meta_bytes, meta.shape.len() as u32)?;
        for dim in &meta.shape {
            write_u64(&mut meta_bytes, *dim)?;
        }
        write_u64(&mut meta_bytes, meta.offset)?;
        write_u64(&mut meta_bytes, meta.len)?;
        write_u64(&mut meta_bytes, meta.checksum)?;
    }

    let mut file = File::create(path).map_err(io_err)?;
    file.write_all(MAGIC).map_err(io_err)?;
    file.write_all(&1u32.to_le_bytes()).map_err(io_err)?;
    file.write_all(&(meta_bytes.len() as u64).to_le_bytes()).map_err(io_err)?;
    file.write_all(&(blob.len() as u64).to_le_bytes()).map_err(io_err)?;
    file.write_all(&meta_bytes).map_err(io_err)?;
    file.write_all(&blob).map_err(io_err)?;

    Ok(())
}

fn read_model<P: AsRef<Path>>(path: P) -> Result<Model, String> {
    let mut file = File::open(path).map_err(io_err)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(io_err)?;

    let mut cursor = Cursor::new(&bytes);

    let magic = cursor.read_exact(8)?;
    if magic != MAGIC {
        return Err("invalid file magic".to_string());
    }

    let version = cursor.read_u32()?;
    if version != 1 {
        return Err(format!("unsupported format version {version}"));
    }

    let meta_len = cursor.read_u64()? as usize;
    let blob_len = cursor.read_u64()? as usize;

    let meta_start = cursor.pos;
    let meta_end = checked_add(meta_start, meta_len)?;
    let blob_start = meta_end;
    let blob_end = checked_add(blob_start, blob_len)?;

    if blob_end != bytes.len() {
        return Err("file length does not match header".to_string());
    }

    let mut meta_cursor = Cursor::new(&bytes[meta_start..meta_end]);

    let name = meta_cursor.read_string()?;
    let model_type = meta_cursor.read_string()?;
    let tensor_count = meta_cursor.read_u32()? as usize;

    let blob = &bytes[blob_start..blob_end];
    let mut tensors = HashMap::new();

    for _ in 0..tensor_count {
        let tensor_name = meta_cursor.read_string()?;
        let dtype = meta_cursor.read_u8()?;
        let rank = meta_cursor.read_u32()? as usize;

        let mut shape = Vec::with_capacity(rank);
        for _ in 0..rank {
            shape.push(meta_cursor.read_u64()?);
        }

        let offset = meta_cursor.read_u64()? as usize;
        let len = meta_cursor.read_u64()? as usize;
        let checksum = meta_cursor.read_u64()?;

        let end = checked_add(offset, len)?;
        if end > blob.len() {
            return Err(format!("tensor {tensor_name} points outside tensor blob"));
        }

        let tensor_bytes = blob[offset..end].to_vec();

        if fnv1a64(&tensor_bytes) != checksum {
            return Err(format!("checksum mismatch for tensor {tensor_name}"));
        }

        let tensor = Tensor {
            dtype,
            shape,
            bytes: tensor_bytes,
        };

        validate_tensor(&tensor)?;
        tensors.insert(tensor_name, tensor);
    }

    if meta_cursor.remaining() != 0 {
        return Err("trailing bytes in metadata section".to_string());
    }

    Ok(Model {
        name,
        model_type,
        tensors,
    })
}

fn inspect(model: &Model) {
    println!("name: {}", model.name);
    println!("model_type: {}", model.model_type);
    println!("tensors:");

    let mut entries: Vec<_> = model.tensors.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    for (name, tensor) in entries {
        println!(
            "  - name: {name}, dtype: {}, shape: {:?}, bytes: {}",
            dtype_name(tensor.dtype),
            tensor.shape,
            tensor.bytes.len()
        );
    }
}

fn infer_dense_relu(model: &Model, input: &[f32]) -> Result<Vec<f32>, String> {
    if model.model_type != "dense_relu" {
        return Err(format!("unsupported model_type {}", model.model_type));
    }

    let weight = model.tensors.get("dense.weight").ok_or("missing dense.weight")?;
    let bias = model.tensors.get("dense.bias").ok_or("missing dense.bias")?;

    if weight.dtype != DTYPE_F32 || bias.dtype != DTYPE_F32 {
        return Err("dense tensors must be f32".to_string());
    }

    if weight.shape.len() != 2 {
        return Err("dense.weight must be rank 2".to_string());
    }

    let out_dim = weight.shape[0] as usize;
    let in_dim = weight.shape[1] as usize;

    if input.len() != in_dim {
        return Err(format!("expected input length {in_dim}, got {}", input.len()));
    }

    if bias.shape != vec![out_dim as u64] {
        return Err(format!("dense.bias must have shape [{out_dim}]"));
    }

    let weights = le_bytes_to_f32s(&weight.bytes)?;
    let biases = le_bytes_to_f32s(&bias.bytes)?;

    let mut output = vec![0.0f32; out_dim];

    for row in 0..out_dim {
        let mut acc = biases[row];

        for col in 0..in_dim {
            acc += weights[row * in_dim + col] * input[col];
        }

        output[row] = acc.max(0.0);
    }

    Ok(output)
}

fn validate_tensor(tensor: &Tensor) -> Result<(), String> {
    if tensor.dtype != DTYPE_F32 {
        return Err(format!("unsupported dtype {}", tensor.dtype));
    }

    if tensor.bytes.len() % 4 != 0 {
        return Err("f32 tensor byte length must be divisible by 4".to_string());
    }

    let mut elements = 1u64;
    for dim in &tensor.shape {
        elements = elements.checked_mul(*dim).ok_or("tensor element count overflow")?;
    }

    let expected_bytes = elements.checked_mul(4).ok_or("tensor byte length overflow")?;

    if expected_bytes as usize != tensor.bytes.len() {
        return Err(format!(
            "shape {:?} expects {} bytes but got {}",
            tensor.shape,
            expected_bytes,
            tensor.bytes.len()
        ));
    }

    Ok(())
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.pos)
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8], String> {
        let end = checked_add(self.pos, len)?;
        if end > self.bytes.len() {
            return Err("unexpected end of file".to_string());
        }

        let out = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        Ok(self.read_exact(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let b = self.read_exact(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        let b = self.read_exact(8)?;
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_exact(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "invalid utf-8 string".to_string())
    }
}

fn write_u8(out: &mut Vec<u8>, value: u8) -> Result<(), String> {
    out.push(value);
    Ok(())
}

fn write_u32(out: &mut Vec<u8>, value: u32) -> Result<(), String> {
    out.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn write_u64(out: &mut Vec<u8>, value: u64) -> Result<(), String> {
    out.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn write_string(out: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let len: u32 = value.len().try_into().map_err(|_| "string too large".to_string())?;
    write_u32(out, len)?;
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

fn f32s_to_le_bytes(values: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 4);
    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

fn le_bytes_to_f32s(bytes: &[u8]) -> Result<Vec<f32>, String> {
    if bytes.len() % 4 != 0 {
        return Err("invalid f32 byte length".to_string());
    }

    let mut values = Vec::with_capacity(bytes.len() / 4);

    for chunk in bytes.chunks_exact(4) {
        values.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    }

    Ok(values)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;

    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

fn checked_add(a: usize, b: usize) -> Result<usize, String> {
    a.checked_add(b).ok_or("integer overflow".to_string())
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> Result<&'a str, String> {
    args.get(index).map(String::as_str).ok_or_else(|| format!("missing required argument: {name}"))
}

fn parse_f32_args(args: &[String]) -> Result<Vec<f32>, String> {
    let mut values = Vec::with_capacity(args.len());

    for arg in args {
        values.push(arg.parse::<f32>().map_err(|_| format!("invalid f32 argument: {arg}"))?);
    }

    Ok(values)
}

fn dtype_name(dtype: u8) -> &'static str {
    match dtype {
        DTYPE_F32 => "f32",
        _ => "unknown",
    }
}

fn io_err(err: io::Error) -> String {
    err.to_string()
}
