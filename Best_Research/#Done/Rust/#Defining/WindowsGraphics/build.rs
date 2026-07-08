#![allow(clippy::needless_return)]

use std::{env, error::Error, fs, path::PathBuf};

const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION_2: u32 = 2;
const GLB_JSON_CHUNK_TYPE: u32 = 0x4E4F_534A;
const GLB_BINARY_CHUNK_TYPE: u32 = 0x004E_4942;

fn main() -> Result<(), Box<dyn Error>>
{
    let output_directory = PathBuf::from(env::var("OUT_DIR")?);
    let output_path = output_directory.join("example_cube.glb");
    let glb_file = create_example_glb();

    fs::write(output_path, glb_file)?;

    return Ok(());
}

fn create_example_glb() -> Vec<u8>
{
    let json = br#"{
        "asset":{"version":"2.0","generator":"WindowsGraphics GLB loader example"},
        "buffers":[{"byteLength":168}],
        "bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":96},{"buffer":0,"byteOffset":96,"byteLength":72}],
        "accessors":[{"bufferView":0,"componentType":5126,"count":8,"type":"VEC3"},{"bufferView":1,"componentType":5123,"count":36,"type":"SCALAR"}],
        "materials":[{"pbrMetallicRoughness":{"baseColorFactor":[0.20,0.85,0.95,1.0]}}],
        "meshes":[{"primitives":[{"attributes":{"POSITION":0},"indices":1,"material":0}]}],
        "nodes":[{"mesh":0,"translation":[0.0,-1.6,5.0],"scale":[0.55,0.55,0.55]}]
    }"#;
    let mut padded_json = json.to_vec();

    while !padded_json.len().is_multiple_of(4)
    {
        padded_json.push(b' ');
    }

    let mut binary_data = Vec::new();

    for value in [
        -1.0_f32, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, -1.0, -1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
    ]
    {
        binary_data.extend_from_slice(&value.to_le_bytes());
    }

    for index in [
        0_u16, 1, 2, 0, 2, 3,
        4, 6, 5, 4, 7, 6,
        0, 4, 5, 0, 5, 1,
        3, 2, 6, 3, 6, 7,
        1, 5, 6, 1, 6, 2,
        4, 0, 3, 4, 3, 7,
    ]
    {
        binary_data.extend_from_slice(&index.to_le_bytes());
    }

    let file_length = 12 + 8 + padded_json.len() + 8 + binary_data.len();
    let mut file = Vec::with_capacity(file_length);
    file.extend_from_slice(&GLB_MAGIC.to_le_bytes());
    file.extend_from_slice(&GLB_VERSION_2.to_le_bytes());
    file.extend_from_slice(&(file_length as u32).to_le_bytes());
    file.extend_from_slice(&(padded_json.len() as u32).to_le_bytes());
    file.extend_from_slice(&GLB_JSON_CHUNK_TYPE.to_le_bytes());
    file.extend_from_slice(&padded_json);
    file.extend_from_slice(&(binary_data.len() as u32).to_le_bytes());
    file.extend_from_slice(&GLB_BINARY_CHUNK_TYPE.to_le_bytes());
    file.extend_from_slice(&binary_data);

    return file;
}
