use anyhow::{anyhow, bail, Context, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, Subcommand};
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

const MAGIC: &[u8; 8] = b"MLBIN001";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub format_version: u32,
    pub model_type: String,
    pub tensors: Vec<TensorMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorMetadata {
    pub name: String,
    pub dtype: DType,
    pub shape: Vec<u64>,
    pub byte_offset: u64,
    pub byte_len: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DType {
    F32,
}

#[derive(Debug, Clone)]
pub struct Tensor {
    pub dtype: DType,
    pub shape: Vec<u64>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ModelFile {
    pub metadata: ModelMetadata,
    pub tensors: HashMap<String, Tensor>,
}

#[derive(Parser)]
#[command(name = "mlbin")]
#[command(about = "Minimal Rust binary container for ML model tensors")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    CreateDemo {
        #[arg(short, long, default_value = "demo.mlbin")]
        output: PathBuf,
    },
    Inspect {
        file: PathBuf,
    },
    Infer {
        file: PathBuf,
        #[arg(required = true)]
        input: Vec<f32>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::CreateDemo { output } => {
            let model = create_demo_model()?;
            write_model(&output, &model)?;
            println!("created {}", output.display());
        }
        Command::Inspect { file } => {
            let model = read_model(&file)?;
            println!("{}", serde_json::to_string_pretty(&model.metadata)?);
        }
        Command::Infer { file, input } => {
            let model = read_model(&file)?;
            let output = infer_dense_relu(&model, &input)?;
            println!("{output:?}");
        }
    }

    Ok(())
}

fn create_demo_model() -> Result<ModelFile> {
    let w: Vec<f32> = vec![0.42, 1.31, -0.18, 0.77, 0.90, -0.35];
    let b: Vec<f32> = vec![0.10, -0.20, 0.05];

    let mut tensors = HashMap::new();

    tensors.insert(
        "dense.weight".to_string(),
        Tensor {
            dtype: DType::F32,
            shape: vec![3, 2],
            bytes: f32s_to_le_bytes(&w),
        },
    );

    tensors.insert(
        "dense.bias".to_string(),
        Tensor {
            dtype: DType::F32,
            shape: vec![3],
            bytes: f32s_to_le_bytes(&b),
        },
    );

    let metadata = ModelMetadata {
        name: "demo_dense_relu".to_string(),
        format_version: 1,
        model_type: "dense_relu".to_string(),
        tensors: Vec::new(),
    };

    Ok(ModelFile { metadata, tensors })
}

fn write_model(path: &PathBuf, model: &ModelFile) -> Result<()> {
    let mut tensor_entries: Vec<(&String, &Tensor)> = model.tensors.iter().collect();
    tensor_entries.sort_by(|a, b| a.0.cmp(b.0));

    let mut metadata = model.metadata.clone();
    metadata.tensors.clear();

    let mut tensor_blob = Vec::new();

    for (name, tensor) in tensor_entries {
        validate_tensor(tensor).with_context(|| format!("invalid tensor {name}"))?;

        let offset = tensor_blob.len() as u64;
        tensor_blob.extend_from_slice(&tensor.bytes);

        metadata.tensors.push(TensorMetadata {
            name: name.clone(),
            dtype: tensor.dtype,
            shape: tensor.shape.clone(),
            byte_offset: offset,
            byte_len: tensor.bytes.len() as u64,
            sha256: hex_sha256(&tensor.bytes),
        });
    }

    let metadata_json = serde_json::to_vec_pretty(&metadata)?;
    let mut file = File::create(path)?;

    file.write_all(MAGIC)?;
    file.write_u64::<LittleEndian>(metadata_json.len() as u64)?;
    file.write_u64::<LittleEndian>(tensor_blob.len() as u64)?;
    file.write_all(&metadata_json)?;
    file.write_all(&tensor_blob)?;

    Ok(())
}

fn read_model(path: &PathBuf) -> Result<ModelFile> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut cursor = Cursor::new(&data);

    let mut magic = [0u8; 8];
    cursor.read_exact(&mut magic)?;

    if &magic != MAGIC {
        bail!("not an MLBIN001 file");
    }

    let metadata_len = cursor.read_u64::<LittleEndian>()? as usize;
    let tensor_blob_len = cursor.read_u64::<LittleEndian>()? as usize;

    let header_len = 8 + 8 + 8;
    let expected_len = header_len
        .checked_add(metadata_len)
        .and_then(|n| n.checked_add(tensor_blob_len))
        .ok_or_else(|| anyhow!("file length overflow"))?;

    if data.len() != expected_len {
        bail!("corrupt file length: expected {}, got {}", expected_len, data.len());
    }

    let metadata_start = header_len;
    let metadata_end = metadata_start + metadata_len;
    let tensor_blob_start = metadata_end;
    let tensor_blob_end = tensor_blob_start + tensor_blob_len;

    let metadata: ModelMetadata = serde_json::from_slice(&data[metadata_start..metadata_end])?;

    if metadata.format_version != 1 {
        bail!("unsupported format version {}", metadata.format_version);
    }

    let tensor_blob = &data[tensor_blob_start..tensor_blob_end];
    let mut tensors = HashMap::new();

    for entry in &metadata.tensors {
        let start = entry.byte_offset as usize;
        let end = start
            .checked_add(entry.byte_len as usize)
            .ok_or_else(|| anyhow!("tensor byte range overflow: {}", entry.name))?;

        if end > tensor_blob.len() {
            bail!("tensor {} points outside tensor blob", entry.name);
        }

        let bytes = tensor_blob[start..end].to_vec();

        if hex_sha256(&bytes) != entry.sha256 {
            bail!("sha256 mismatch for tensor {}", entry.name);
        }

        let tensor = Tensor {
            dtype: entry.dtype,
            shape: entry.shape.clone(),
            bytes,
        };

        validate_tensor(&tensor)?;
        tensors.insert(entry.name.clone(), tensor);
    }

    Ok(ModelFile { metadata, tensors })
}

fn infer_dense_relu(model: &ModelFile, input: &[f32]) -> Result<Vec<f32>> {
    if model.metadata.model_type != "dense_relu" {
        bail!("unsupported model type {}", model.metadata.model_type);
    }

    let weight = model.tensors.get("dense.weight").ok_or_else(|| anyhow!("missing dense.weight"))?;
    let bias = model.tensors.get("dense.bias").ok_or_else(|| anyhow!("missing dense.bias"))?;

    let weight_shape = &weight.shape;

    if weight_shape.len() != 2 {
        bail!("dense.weight must be rank-2");
    }

    let out_dim = weight_shape[0] as usize;
    let in_dim = weight_shape[1] as usize;

    if input.len() != in_dim {
        bail!("expected input length {}, got {}", in_dim, input.len());
    }

    if bias.shape != vec![out_dim as u64] {
        bail!("dense.bias shape must be [{out_dim}]");
    }

    let w_values = le_bytes_to_f32s(&weight.bytes)?;
    let b_values = le_bytes_to_f32s(&bias.bytes)?;

    let w = Array2::from_shape_vec((out_dim, in_dim), w_values)?;
    let b = Array1::from_vec(b_values);
    let x = Array1::from_vec(input.to_vec());

    let mut y = w.dot(&x) + b;

    for v in y.iter_mut() {
        *v = v.max(0.0);
    }

    Ok(y.to_vec())
}

fn validate_tensor(tensor: &Tensor) -> Result<()> {
    match tensor.dtype {
        DType::F32 => {
            if tensor.bytes.len() % 4 != 0 {
                bail!("f32 tensor byte length must be divisible by 4");
            }

            let expected_elements = tensor
                .shape
                .iter()
                .try_fold(1u64, |acc, dim| acc.checked_mul(*dim))
                .ok_or_else(|| anyhow!("tensor shape element count overflow"))?;

            let expected_bytes = expected_elements
                .checked_mul(4)
                .ok_or_else(|| anyhow!("tensor byte length overflow"))?;

            if expected_bytes as usize != tensor.bytes.len() {
                bail!("tensor shape expects {} bytes but got {}", expected_bytes, tensor.bytes.len());
            }
        }
    }

    Ok(())
}

fn f32s_to_le_bytes(values: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 4);

    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }

    out
}

fn le_bytes_to_f32s(bytes: &[u8]) -> Result<Vec<f32>> {
    if bytes.len() % 4 != 0 {
        bail!("f32 byte buffer length must be divisible by 4");
    }

    Ok(bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect())
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}
