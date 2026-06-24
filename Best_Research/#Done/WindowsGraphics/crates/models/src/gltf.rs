use std::{fs, path::Path};
use windows::{
    core::{Error, HSTRING, Result},
    Data::Json::{JsonArray, JsonObject},
    Win32::Foundation::E_FAIL,
};

use graphics::{GraphicsObject, GraphicsVertex};

// data structures
/// A static mesh instance loaded from a glTF or GLB container.
pub struct GltfObject
{
    object_identifier: u64,
    mesh_identifier: u64,
    material_identifier: u64,
    vertices: Vec<GraphicsVertex>,
    indices: Vec<u16>,
    position: [f32; 3],
    rotation_radians_per_second: [f32; 3],
    bounding_radius: f32,
}

struct GltfBufferView
{
    buffer_index: usize,
    byte_offset: usize,
    byte_length: usize,
    byte_stride: Option<usize>,
}

struct GltfAccessor
{
    buffer_view_index: usize,
    byte_offset: usize,
    component_type: u32,
    component_count: usize,
    element_count: usize,
}

struct GltfLoadingData<'document>
{
    meshes: &'document JsonArray,
    materials: &'document Option<JsonArray>,
    accessors: &'document [GltfAccessor],
    buffer_views: &'document [GltfBufferView],
    buffers: &'document [Vec<u8>],
}
// data structures

// domain constants
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_VERSION_2: u32 = 2;
const GLB_JSON_CHUNK_TYPE: u32 = 0x4E4F_534A;
const GLB_BINARY_CHUNK_TYPE: u32 = 0x004E_4942;
const GLTF_TRIANGLES_MODE: u32 = 4;
const COMPONENT_TYPE_UNSIGNED_BYTE: u32 = 5121;
const COMPONENT_TYPE_UNSIGNED_SHORT: u32 = 5123;
const COMPONENT_TYPE_UNSIGNED_INT: u32 = 5125;
const COMPONENT_TYPE_FLOAT: u32 = 5126;
const DEFAULT_VERTEX_COLOUR: [f32; 3] = [0.80, 0.80, 0.80];
const CUBE_TEXTURE_SIZE: [u32; 2] = [3, 2];
const CUBE_TEXTURE_PIXELS: [u8; 24] = [
    220, 60, 60, 255,
    60, 190, 220, 255,
    70, 90, 220, 255,
    230, 140, 50, 255,
    80, 210, 90, 255,
    180, 70, 210, 255,
];
// domain constants

/// Loads every mesh-bearing node from a `.gltf` or `.glb` file.
///
/// Object identifiers begin at `first_object_identifier` and increase by one per loaded object.
pub fn load_objects(file_path: &Path, first_object_identifier: u64) -> Result<Vec<GltfObject>>
{
    let (json_text, glb_binary_buffer) = read_gltf_container(file_path)?;
    let document = JsonObject::Parse(&HSTRING::from(json_text))?;
    let buffers = read_buffers(&document, file_path, glb_binary_buffer)?;
    let buffer_views = read_buffer_views(&document)?;
    let accessors = read_accessors(&document)?;
    let meshes = document.GetNamedArray(&HSTRING::from("meshes"))?;
    let materials = named_array_if_present(&document, "materials")?;
    let nodes = named_array_if_present(&document, "nodes")?;
    let loading_data = GltfLoadingData {
        meshes: &meshes,
        materials: &materials,
        accessors: &accessors,
        buffer_views: &buffer_views,
        buffers: &buffers,
    };
    let mut objects = Vec::new();
    let mut next_object_identifier = first_object_identifier;

    if let Some(nodes) = nodes
    {
        let mut node_index = 0;

        while node_index < nodes.Size()? as usize
        {
            let node = nodes.GetObjectAt(node_index as u32)?;

            if node.HasKey(&HSTRING::from("mesh"))?
            {
                let mesh_index = named_usize(&node, "mesh")?;
                let mut transform = node_transform(&node)?;
                let position = translation_from_transform(transform);
                transform[12] = 0.0;
                transform[13] = 0.0;
                transform[14] = 0.0;
                append_mesh_objects(
                    &mut objects,
                    &mut next_object_identifier,
                    mesh_index,
                    position,
                    transform,
                    &loading_data,
                )?;
            }

            node_index += 1;
        }
    }
    else
    {
        let mut mesh_index = 0;

        while mesh_index < meshes.Size()? as usize
        {
            append_mesh_objects(
                &mut objects,
                &mut next_object_identifier,
                mesh_index,
                [0.0, 0.0, 0.0],
                identity_matrix(),
                &loading_data,
            )?;
            mesh_index += 1;
        }
    }

    if objects.is_empty()
    {
        return Err(Error::new(E_FAIL, "The glTF file contains no mesh nodes."));
    }

    return Ok(objects);
}

impl GraphicsObject for GltfObject
{
    fn identifier(&self) -> u64
    {
        return self.object_identifier;
    }

    fn mesh_identifier(&self) -> u64
    {
        return self.mesh_identifier;
    }

    fn material_identifier(&self) -> u64
    {
        return self.material_identifier;
    }

    fn vertices(&self) -> &[GraphicsVertex]
    {
        return &self.vertices;
    }

    fn indices(&self) -> &[u16]
    {
        return &self.indices;
    }

    fn position(&self) -> [f32; 3]
    {
        return self.position;
    }

    fn rotation_radians(&self, elapsed_seconds: f32) -> [f32; 3]
    {
        return [
            elapsed_seconds * self.rotation_radians_per_second[0],
            elapsed_seconds * self.rotation_radians_per_second[1],
            elapsed_seconds * self.rotation_radians_per_second[2],
        ];
    }

    fn bounding_radius(&self) -> f32
    {
        return self.bounding_radius;
    }

    fn texture_size(&self) -> Option<[u32; 2]>
    {
        return Some(CUBE_TEXTURE_SIZE);
    }

    fn texture_pixels(&self) -> Option<&[u8]>
    {
        return Some(&CUBE_TEXTURE_PIXELS);
    }
}

impl GltfObject
{
    /// Sets the object's Euler rotation rate in radians per second.
    pub fn set_rotation_radians_per_second(&mut self, rotation_radians_per_second: [f32; 3])
    {
        self.rotation_radians_per_second = rotation_radians_per_second;
    }
}

fn append_mesh_objects(
    objects: &mut Vec<GltfObject>,
    next_object_identifier: &mut u64,
    mesh_index: usize,
    position: [f32; 3],
    transform: [f32; 16],
    loading_data: &GltfLoadingData,
) -> Result<()>
{
    let mesh = loading_data.meshes.GetObjectAt(mesh_index as u32)?;
    let primitives = mesh.GetNamedArray(&HSTRING::from("primitives"))?;
    let mut primitive_index = 0;

    while primitive_index < primitives.Size()? as usize
    {
        let primitive = primitives.GetObjectAt(primitive_index as u32)?;
        let mode = named_usize_or_default(&primitive, "mode", GLTF_TRIANGLES_MODE as usize)?;

        if mode != GLTF_TRIANGLES_MODE as usize
        {
            return Err(Error::new(E_FAIL, "Only glTF triangle primitives are supported."));
        }

        let attributes = primitive.GetNamedObject(&HSTRING::from("attributes"))?;
        let position_accessor_index = named_usize(&attributes, "POSITION")?;
        let colour_accessor_index = if attributes.HasKey(&HSTRING::from("COLOR_0"))?
        {
            Some(named_usize(&attributes, "COLOR_0")?)
        }
        else
        {
            None
        };
        let material_colour = primitive_colour(&primitive, loading_data.materials)?;
        let vertices = read_vertices(
            position_accessor_index,
            colour_accessor_index,
            material_colour,
            transform,
            loading_data.accessors,
            loading_data.buffer_views,
            loading_data.buffers,
        )?;
        let indices = read_indices(
            &primitive,
            vertices.len(),
            loading_data.accessors,
            loading_data.buffer_views,
            loading_data.buffers,
        )?;
        let bounding_radius = bounding_radius(&vertices);

        objects.push(GltfObject {
            object_identifier: *next_object_identifier,
            mesh_identifier: *next_object_identifier,
            material_identifier: *next_object_identifier,
            vertices,
            indices,
            position,
            rotation_radians_per_second: [0.0, 0.0, 0.0],
            bounding_radius,
        });
        *next_object_identifier += 1;
        primitive_index += 1;
    }

    return Ok(());
}

fn read_vertices(
    position_accessor_index: usize,
    colour_accessor_index: Option<usize>,
    material_colour: [f32; 3],
    transform: [f32; 16],
    accessors: &[GltfAccessor],
    buffer_views: &[GltfBufferView],
    buffers: &[Vec<u8>],
) -> Result<Vec<GraphicsVertex>>
{
    let positions = accessor_at(accessors, position_accessor_index)?;

    if positions.component_type != COMPONENT_TYPE_FLOAT || positions.component_count != 3
    {
        return Err(Error::new(E_FAIL, "glTF POSITION must use floating-point VEC3 values."));
    }

    let colours = match colour_accessor_index
    {
        Some(colour_accessor_index) => Some(accessor_at(accessors, colour_accessor_index)?),
        None => None,
    };
    let mut vertices = Vec::with_capacity(positions.element_count);
    let mut vertex_index = 0;

    while vertex_index < positions.element_count
    {
        let position = read_vector3(positions, vertex_index, buffer_views, buffers)?;
        let colour = match colours
        {
            Some(colours) => multiply_colours(
                read_colour(colours, vertex_index, buffer_views, buffers)?,
                material_colour,
            ),
            None => material_colour,
        };

        vertices.push(GraphicsVertex {
            position: transform_position(position, transform),
            color: colour,
        });
        vertex_index += 1;
    }

    return Ok(vertices);
}

fn read_indices(
    primitive: &JsonObject,
    vertex_count: usize,
    accessors: &[GltfAccessor],
    buffer_views: &[GltfBufferView],
    buffers: &[Vec<u8>],
) -> Result<Vec<u16>>
{
    if !primitive.HasKey(&HSTRING::from("indices"))?
    {
        let mut indices = Vec::with_capacity(vertex_count);
        let mut vertex_index = 0;

        while vertex_index < vertex_count
        {
            indices.push(u16::try_from(vertex_index).map_err(|_| Error::new(E_FAIL, "The glTF primitive has more than 65,535 vertices."))?);
            vertex_index += 1;
        }

        return Ok(indices);
    }

    let accessor_index = named_usize(primitive, "indices")?;
    let accessor = accessor_at(accessors, accessor_index)?;

    if accessor.component_count != 1
    {
        return Err(Error::new(E_FAIL, "glTF indices must be scalar values."));
    }

    let mut indices = Vec::with_capacity(accessor.element_count);
    let mut index = 0;

    while index < accessor.element_count
    {
        let index_value = read_unsigned_component(accessor, index, 0, buffer_views, buffers)?;
        indices.push(u16::try_from(index_value).map_err(|_| Error::new(E_FAIL, "The glTF primitive uses an index above 65,535."))?);
        index += 1;
    }

    return Ok(indices);
}

fn primitive_colour(primitive: &JsonObject, materials: &Option<JsonArray>) -> Result<[f32; 3]>
{
    let material_index = if primitive.HasKey(&HSTRING::from("material"))?
    {
        Some(named_usize(primitive, "material")?)
    }
    else
    {
        None
    };
    let materials = match materials
    {
        Some(materials) => materials,
        None => return Ok(DEFAULT_VERTEX_COLOUR),
    };
    let material_index = match material_index
    {
        Some(material_index) => material_index,
        None => return Ok(DEFAULT_VERTEX_COLOUR),
    };
    let material = materials.GetObjectAt(material_index as u32)?;

    if !material.HasKey(&HSTRING::from("pbrMetallicRoughness"))?
    {
        return Ok(DEFAULT_VERTEX_COLOUR);
    }

    let pbr = material.GetNamedObject(&HSTRING::from("pbrMetallicRoughness"))?;

    if !pbr.HasKey(&HSTRING::from("baseColorFactor"))?
    {
        return Ok(DEFAULT_VERTEX_COLOUR);
    }

    let colour = pbr.GetNamedArray(&HSTRING::from("baseColorFactor"))?;

    return Ok([
        colour.GetNumberAt(0)? as f32,
        colour.GetNumberAt(1)? as f32,
        colour.GetNumberAt(2)? as f32,
    ]);
}

fn read_buffers(document: &JsonObject, file_path: &Path, glb_binary_buffer: Option<Vec<u8>>) -> Result<Vec<Vec<u8>>>
{
    let json_buffers = document.GetNamedArray(&HSTRING::from("buffers"))?;
    let mut buffers = Vec::with_capacity(json_buffers.Size()? as usize);
    let mut buffer_index = 0;

    while buffer_index < json_buffers.Size()? as usize
    {
        let json_buffer = json_buffers.GetObjectAt(buffer_index as u32)?;
        let required_byte_length = named_usize(&json_buffer, "byteLength")?;
        let buffer = if json_buffer.HasKey(&HSTRING::from("uri"))?
        {
            let uri = json_buffer.GetNamedString(&HSTRING::from("uri"))?.to_string_lossy();
            read_buffer_uri(file_path, &uri)?
        }
        else
        {
            match &glb_binary_buffer
            {
                Some(glb_binary_buffer) => glb_binary_buffer.clone(),
                None => return Err(Error::new(E_FAIL, "The glTF buffer has no URI or GLB binary chunk.")),
            }
        };

        if buffer.len() < required_byte_length
        {
            return Err(Error::new(E_FAIL, "The glTF buffer is shorter than its declared byteLength."));
        }

        buffers.push(buffer);
        buffer_index += 1;
    }

    return Ok(buffers);
}

fn read_buffer_views(document: &JsonObject) -> Result<Vec<GltfBufferView>>
{
    let json_buffer_views = document.GetNamedArray(&HSTRING::from("bufferViews"))?;
    let mut buffer_views = Vec::with_capacity(json_buffer_views.Size()? as usize);
    let mut buffer_view_index = 0;

    while buffer_view_index < json_buffer_views.Size()? as usize
    {
        let buffer_view = json_buffer_views.GetObjectAt(buffer_view_index as u32)?;
        let byte_stride = if buffer_view.HasKey(&HSTRING::from("byteStride"))?
        {
            Some(named_usize(&buffer_view, "byteStride")?)
        }
        else
        {
            None
        };
        buffer_views.push(GltfBufferView {
            buffer_index: named_usize(&buffer_view, "buffer")?,
            byte_offset: named_usize_or_default(&buffer_view, "byteOffset", 0)?,
            byte_length: named_usize(&buffer_view, "byteLength")?,
            byte_stride,
        });
        buffer_view_index += 1;
    }

    return Ok(buffer_views);
}

fn read_accessors(document: &JsonObject) -> Result<Vec<GltfAccessor>>
{
    let json_accessors = document.GetNamedArray(&HSTRING::from("accessors"))?;
    let mut accessors = Vec::with_capacity(json_accessors.Size()? as usize);
    let mut accessor_index = 0;

    while accessor_index < json_accessors.Size()? as usize
    {
        let accessor = json_accessors.GetObjectAt(accessor_index as u32)?;

        if !accessor.HasKey(&HSTRING::from("bufferView"))?
        {
            return Err(Error::new(E_FAIL, "Sparse glTF accessors are not supported."));
        }

        let accessor_type = accessor.GetNamedString(&HSTRING::from("type"))?.to_string_lossy();
        accessors.push(GltfAccessor {
            buffer_view_index: named_usize(&accessor, "bufferView")?,
            byte_offset: named_usize_or_default(&accessor, "byteOffset", 0)?,
            component_type: named_usize(&accessor, "componentType")? as u32,
            component_count: accessor_component_count(&accessor_type)?,
            element_count: named_usize(&accessor, "count")?,
        });
        accessor_index += 1;
    }

    return Ok(accessors);
}

fn read_vector3(accessor: &GltfAccessor, element_index: usize, buffer_views: &[GltfBufferView], buffers: &[Vec<u8>]) -> Result<[f32; 3]>
{
    return Ok([
        read_float_component(accessor, element_index, 0, buffer_views, buffers)?,
        read_float_component(accessor, element_index, 1, buffer_views, buffers)?,
        read_float_component(accessor, element_index, 2, buffer_views, buffers)?,
    ]);
}

fn read_colour(accessor: &GltfAccessor, element_index: usize, buffer_views: &[GltfBufferView], buffers: &[Vec<u8>]) -> Result<[f32; 3]>
{
    if accessor.component_count != 3 && accessor.component_count != 4
    {
        return Err(Error::new(E_FAIL, "glTF COLOR_0 must use VEC3 or VEC4 values."));
    }

    return Ok([
        read_normalized_component(accessor, element_index, 0, buffer_views, buffers)?,
        read_normalized_component(accessor, element_index, 1, buffer_views, buffers)?,
        read_normalized_component(accessor, element_index, 2, buffer_views, buffers)?,
    ]);
}

fn read_float_component(accessor: &GltfAccessor, element_index: usize, component_index: usize, buffer_views: &[GltfBufferView], buffers: &[Vec<u8>]) -> Result<f32>
{
    if accessor.component_type != COMPONENT_TYPE_FLOAT
    {
        return Err(Error::new(E_FAIL, "This glTF value must use floating-point components."));
    }

    let offset = accessor_component_offset(accessor, element_index, component_index, buffer_views)?;
    let buffer = accessor_buffer(accessor, buffer_views, buffers)?;
    return Ok(f32::from_le_bytes(read_bytes::<4>(buffer, offset)?));
}

fn read_normalized_component(accessor: &GltfAccessor, element_index: usize, component_index: usize, buffer_views: &[GltfBufferView], buffers: &[Vec<u8>]) -> Result<f32>
{
    if accessor.component_type == COMPONENT_TYPE_FLOAT
    {
        return read_float_component(accessor, element_index, component_index, buffer_views, buffers);
    }

    let value = read_unsigned_component(accessor, element_index, component_index, buffer_views, buffers)?;

    return match accessor.component_type
    {
        COMPONENT_TYPE_UNSIGNED_BYTE => Ok(value as f32 / u8::MAX as f32),
        COMPONENT_TYPE_UNSIGNED_SHORT => Ok(value as f32 / u16::MAX as f32),
        _ => Err(Error::new(E_FAIL, "Unsupported glTF vertex colour component type.")),
    };
}

fn read_unsigned_component(accessor: &GltfAccessor, element_index: usize, component_index: usize, buffer_views: &[GltfBufferView], buffers: &[Vec<u8>]) -> Result<u32>
{
    let offset = accessor_component_offset(accessor, element_index, component_index, buffer_views)?;
    let buffer = accessor_buffer(accessor, buffer_views, buffers)?;

    return match accessor.component_type
    {
        COMPONENT_TYPE_UNSIGNED_BYTE => Ok(buffer_at(buffer, offset)? as u32),
        COMPONENT_TYPE_UNSIGNED_SHORT => Ok(u16::from_le_bytes(read_bytes::<2>(buffer, offset)?) as u32),
        COMPONENT_TYPE_UNSIGNED_INT => Ok(u32::from_le_bytes(read_bytes::<4>(buffer, offset)?)),
        _ => Err(Error::new(E_FAIL, "Unsupported glTF unsigned component type.")),
    };
}

fn accessor_component_offset(accessor: &GltfAccessor, element_index: usize, component_index: usize, buffer_views: &[GltfBufferView]) -> Result<usize>
{
    if element_index >= accessor.element_count || component_index >= accessor.component_count
    {
        return Err(Error::new(E_FAIL, "The glTF accessor component is out of range."));
    }

    let buffer_view = buffer_view_at(buffer_views, accessor.buffer_view_index)?;
    let component_size = component_size(accessor.component_type)?;
    let element_size = component_size * accessor.component_count;
    let stride = buffer_view.byte_stride.unwrap_or(element_size);

    if stride < element_size
    {
        return Err(Error::new(E_FAIL, "The glTF buffer view byteStride is too small."));
    }

    let offset = buffer_view.byte_offset
        .checked_add(accessor.byte_offset)
        .and_then(|offset| offset.checked_add(element_index * stride))
        .and_then(|offset| offset.checked_add(component_index * component_size))
        .ok_or_else(|| Error::new(E_FAIL, "The glTF accessor offset overflows."))?;
    let final_offset = offset.checked_add(component_size).ok_or_else(|| Error::new(E_FAIL, "The glTF accessor offset overflows."))?;

    if final_offset > buffer_view.byte_offset + buffer_view.byte_length
    {
        return Err(Error::new(E_FAIL, "The glTF accessor exceeds its buffer view."));
    }

    return Ok(offset);
}

fn accessor_buffer<'buffer>(accessor: &GltfAccessor, buffer_views: &[GltfBufferView], buffers: &'buffer [Vec<u8>]) -> Result<&'buffer [u8]>
{
    let buffer_view = buffer_view_at(buffer_views, accessor.buffer_view_index)?;
    let buffer = buffers.get(buffer_view.buffer_index).ok_or_else(|| Error::new(E_FAIL, "The glTF buffer view references a missing buffer."))?;
    return Ok(buffer);
}

fn buffer_view_at(buffer_views: &[GltfBufferView], buffer_view_index: usize) -> Result<&GltfBufferView>
{
    return buffer_views.get(buffer_view_index).ok_or_else(|| Error::new(E_FAIL, "The glTF accessor references a missing buffer view."));
}

fn accessor_at(accessors: &[GltfAccessor], accessor_index: usize) -> Result<&GltfAccessor>
{
    return accessors.get(accessor_index).ok_or_else(|| Error::new(E_FAIL, "The glTF primitive references a missing accessor."));
}

fn read_gltf_container(file_path: &Path) -> Result<(String, Option<Vec<u8>>)> 
{
    let file_bytes = read_file(file_path)?;

    if file_bytes.len() >= 4 && u32::from_le_bytes([file_bytes[0], file_bytes[1], file_bytes[2], file_bytes[3]]) == GLB_MAGIC
    {
        return read_glb(&file_bytes);
    }

    let json_text = String::from_utf8(file_bytes).map_err(|_| Error::new(E_FAIL, "The .gltf file is not valid UTF-8 JSON."))?;
    return Ok((json_text, None));
}

fn read_glb(file_bytes: &[u8]) -> Result<(String, Option<Vec<u8>>)> 
{
    if file_bytes.len() < 12
    {
        return Err(Error::new(E_FAIL, "The .glb header is incomplete."));
    }

    let version = read_u32(file_bytes, 4)?;

    if version != GLB_VERSION_2
    {
        return Err(Error::new(E_FAIL, "Only glTF binary version 2 is supported."));
    }

    let declared_length = read_u32(file_bytes, 8)? as usize;

    if declared_length != file_bytes.len()
    {
        return Err(Error::new(E_FAIL, "The .glb file length does not match its header."));
    }

    let mut offset = 12;
    let mut json_text = None;
    let mut binary_buffer = None;

    while offset < file_bytes.len()
    {
        let chunk_length = read_u32(file_bytes, offset)? as usize;
        let chunk_type = read_u32(file_bytes, offset + 4)?;
        let chunk_start = offset + 8;
        let chunk_end = chunk_start.checked_add(chunk_length).ok_or_else(|| Error::new(E_FAIL, "The .glb chunk length overflows."))?;

        if chunk_end > file_bytes.len()
        {
            return Err(Error::new(E_FAIL, "The .glb chunk exceeds the file length."));
        }

        if chunk_type == GLB_JSON_CHUNK_TYPE
        {
            json_text = Some(String::from_utf8(file_bytes[chunk_start..chunk_end].to_vec()).map_err(|_| Error::new(E_FAIL, "The .glb JSON chunk is not UTF-8."))?);
        }
        else if chunk_type == GLB_BINARY_CHUNK_TYPE
        {
            binary_buffer = Some(file_bytes[chunk_start..chunk_end].to_vec());
        }

        offset = chunk_end;
    }

    return Ok((json_text.ok_or_else(|| Error::new(E_FAIL, "The .glb file has no JSON chunk."))?, binary_buffer));
}

fn read_buffer_uri(file_path: &Path, uri: &str) -> Result<Vec<u8>>
{
    if uri.starts_with("data:")
    {
        return decode_data_uri(uri);
    }

    let parent_directory = file_path.parent().ok_or_else(|| Error::new(E_FAIL, "The glTF path has no parent directory."))?;
    return read_file(&parent_directory.join(uri));
}

fn decode_data_uri(uri: &str) -> Result<Vec<u8>>
{
    let (_, encoded_data) = uri.split_once(',').ok_or_else(|| Error::new(E_FAIL, "The glTF data URI has no comma separator."))?;

    if !uri.contains(";base64,")
    {
        return Err(Error::new(E_FAIL, "Only base64 glTF data URIs are supported."));
    }

    let mut decoded_data = Vec::with_capacity(encoded_data.len() * 3 / 4);
    let mut accumulated_value = 0_u32;
    let mut accumulated_bit_count = 0;

    for character in encoded_data.bytes()
    {
        if character == b'='
        {
            break;
        }

        let value = base64_value(character)? as u32;
        accumulated_value = (accumulated_value << 6) | value;
        accumulated_bit_count += 6;

        while accumulated_bit_count >= 8
        {
            accumulated_bit_count -= 8;
            decoded_data.push((accumulated_value >> accumulated_bit_count) as u8);
        }
    }

    return Ok(decoded_data);
}

fn base64_value(character: u8) -> Result<u8>
{
    return match character
    {
        b'A'..=b'Z' => Ok(character - b'A'),
        b'a'..=b'z' => Ok(character - b'a' + 26),
        b'0'..=b'9' => Ok(character - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Error::new(E_FAIL, "The glTF data URI contains invalid base64.")),
    };
}

fn node_transform(node: &JsonObject) -> Result<[f32; 16]>
{
    if node.HasKey(&HSTRING::from("matrix"))?
    {
        let matrix = node.GetNamedArray(&HSTRING::from("matrix"))?;

        if matrix.Size()? != 16
        {
            return Err(Error::new(E_FAIL, "A glTF node matrix must have 16 elements."));
        }

        let mut values = [0.0; 16];
        let mut value_index = 0;

        while value_index < values.len()
        {
            values[value_index] = matrix.GetNumberAt(value_index as u32)? as f32;
            value_index += 1;
        }

        return Ok(values);
    }

    let translation = named_vector3_or_default(node, "translation", [0.0, 0.0, 0.0])?;
    let scale = named_vector3_or_default(node, "scale", [1.0, 1.0, 1.0])?;
    let rotation = named_vector4_or_default(node, "rotation", [0.0, 0.0, 0.0, 1.0])?;

    return Ok(transform_from_translation_rotation_scale(translation, rotation, scale));
}

fn transform_from_translation_rotation_scale(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> [f32; 16]
{
    let x = rotation[0];
    let y = rotation[1];
    let z = rotation[2];
    let w = rotation[3];
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;

    return [
        (1.0 - 2.0 * (yy + zz)) * scale[0], (2.0 * (xy + wz)) * scale[0], (2.0 * (xz - wy)) * scale[0], 0.0,
        (2.0 * (xy - wz)) * scale[1], (1.0 - 2.0 * (xx + zz)) * scale[1], (2.0 * (yz + wx)) * scale[1], 0.0,
        (2.0 * (xz + wy)) * scale[2], (2.0 * (yz - wx)) * scale[2], (1.0 - 2.0 * (xx + yy)) * scale[2], 0.0,
        translation[0], translation[1], translation[2], 1.0,
    ];
}

fn transform_position(position: [f32; 3], transform: [f32; 16]) -> [f32; 3]
{
    return [
        transform[0] * position[0] + transform[4] * position[1] + transform[8] * position[2] + transform[12],
        transform[1] * position[0] + transform[5] * position[1] + transform[9] * position[2] + transform[13],
        transform[2] * position[0] + transform[6] * position[1] + transform[10] * position[2] + transform[14],
    ];
}

fn translation_from_transform(transform: [f32; 16]) -> [f32; 3]
{
    return [transform[12], transform[13], transform[14]];
}

fn identity_matrix() -> [f32; 16]
{
    return [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
}

fn bounding_radius(vertices: &[GraphicsVertex]) -> f32
{
    let mut radius_squared = 0.0_f32;

    for vertex in vertices
    {
        let radius_squared_for_vertex = vertex.position[0] * vertex.position[0]
            + vertex.position[1] * vertex.position[1]
            + vertex.position[2] * vertex.position[2];
        radius_squared = radius_squared.max(radius_squared_for_vertex);
    }

    return radius_squared.sqrt();
}

fn multiply_colours(left: [f32; 3], right: [f32; 3]) -> [f32; 3]
{
    return [left[0] * right[0], left[1] * right[1], left[2] * right[2]];
}

fn named_array_if_present(object: &JsonObject, name: &str) -> Result<Option<JsonArray>>
{
    if object.HasKey(&HSTRING::from(name))?
    {
        return Ok(Some(object.GetNamedArray(&HSTRING::from(name))?));
    }

    return Ok(None);
}

fn named_usize(object: &JsonObject, name: &str) -> Result<usize>
{
    if !object.HasKey(&HSTRING::from(name))?
    {
        return Err(Error::new(E_FAIL, "The glTF JSON field is missing."));
    }

    return named_usize_or_default(object, name, 0);
}

fn named_usize_or_default(object: &JsonObject, name: &str, default_value: usize) -> Result<usize>
{
    if !object.HasKey(&HSTRING::from(name))?
    {
        return Ok(default_value);
    }

    let value = object.GetNamedNumber(&HSTRING::from(name))?;

    if value < 0.0 || value.fract() != 0.0 || value > usize::MAX as f64
    {
        return Err(Error::new(E_FAIL, "The glTF JSON number is not a valid unsigned integer."));
    }

    return Ok(value as usize);
}

fn named_vector3_or_default(object: &JsonObject, name: &str, default_value: [f32; 3]) -> Result<[f32; 3]>
{
    if !object.HasKey(&HSTRING::from(name))?
    {
        return Ok(default_value);
    }

    let values = object.GetNamedArray(&HSTRING::from(name))?;

    if values.Size()? != 3
    {
        return Err(Error::new(E_FAIL, "The glTF vector must have three elements."));
    }

    return Ok([values.GetNumberAt(0)? as f32, values.GetNumberAt(1)? as f32, values.GetNumberAt(2)? as f32]);
}

fn named_vector4_or_default(object: &JsonObject, name: &str, default_value: [f32; 4]) -> Result<[f32; 4]>
{
    if !object.HasKey(&HSTRING::from(name))?
    {
        return Ok(default_value);
    }

    let values = object.GetNamedArray(&HSTRING::from(name))?;

    if values.Size()? != 4
    {
        return Err(Error::new(E_FAIL, "The glTF vector must have four elements."));
    }

    return Ok([values.GetNumberAt(0)? as f32, values.GetNumberAt(1)? as f32, values.GetNumberAt(2)? as f32, values.GetNumberAt(3)? as f32]);
}

fn accessor_component_count(accessor_type: &str) -> Result<usize>
{
    return match accessor_type
    {
        "SCALAR" => Ok(1),
        "VEC2" => Ok(2),
        "VEC3" => Ok(3),
        "VEC4" => Ok(4),
        _ => Err(Error::new(E_FAIL, "Unsupported glTF accessor type.")),
    };
}

fn component_size(component_type: u32) -> Result<usize>
{
    return match component_type
    {
        COMPONENT_TYPE_UNSIGNED_BYTE => Ok(1),
        COMPONENT_TYPE_UNSIGNED_SHORT => Ok(2),
        COMPONENT_TYPE_UNSIGNED_INT | COMPONENT_TYPE_FLOAT => Ok(4),
        _ => Err(Error::new(E_FAIL, "Unsupported glTF component type.")),
    };
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32>
{
    return Ok(u32::from_le_bytes(read_bytes::<4>(bytes, offset)?));
}

fn read_bytes<const BYTE_COUNT: usize>(bytes: &[u8], offset: usize) -> Result<[u8; BYTE_COUNT]>
{
    let end = offset.checked_add(BYTE_COUNT).ok_or_else(|| Error::new(E_FAIL, "The glTF binary offset overflows."))?;
    let source = bytes.get(offset..end).ok_or_else(|| Error::new(E_FAIL, "The glTF binary data is out of range."))?;
    let mut values = [0; BYTE_COUNT];
    values.copy_from_slice(source);
    return Ok(values);
}

fn buffer_at(buffer: &[u8], offset: usize) -> Result<u8>
{
    return buffer.get(offset).copied().ok_or_else(|| Error::new(E_FAIL, "The glTF binary data is out of range."));
}

fn read_file(file_path: &Path) -> Result<Vec<u8>>
{
    return fs::read(file_path).map_err(|_| Error::new(E_FAIL, "The glTF file could not be read."));
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn glb_loader_reads_a_triangle_mesh_and_node_translation()
    {
        let json = r#"{
            "asset":{"version":"2.0"},
            "buffers":[{"byteLength":36}],
            "bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":36}],
            "accessors":[{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3"}],
            "meshes":[{"primitives":[{"attributes":{"POSITION":0}}]}],
            "nodes":[{"mesh":0,"translation":[0,0,5]}]
        }"#;
        let mut binary_data = Vec::new();

        for value in [0.0_f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        {
            binary_data.extend_from_slice(&value.to_le_bytes());
        }

        let glb_file = create_glb_file(json.as_bytes(), &binary_data);
        let file_path = std::env::temp_dir().join("windows_graphics_gltf_loader_test.glb");
        std::fs::write(&file_path, glb_file).expect("The test .glb file should be written.");
        let objects = load_objects(&file_path, 100).expect("The test .glb file should load.");
        std::fs::remove_file(&file_path).expect("The test .glb file should be removed.");

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].identifier(), 100);
        assert_eq!(objects[0].vertices().len(), 3);
        assert_eq!(objects[0].indices(), [0, 1, 2]);
        assert_eq!(objects[0].vertices()[0].position, [0.0, 0.0, 0.0]);
        assert_eq!(objects[0].position(), [0.0, 0.0, 5.0]);
    }

    #[test]
    fn checked_in_gltf_example_loads()
    {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/example_cube.gltf");
        let objects = load_objects(&file_path, 1000).expect("The checked-in glTF example should load.");

        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].identifier(), 1000);
        assert_eq!(objects[0].vertices().len(), 8);
        assert_eq!(objects[0].indices().len(), 36);
        assert_eq!(objects[0].position(), [0.0, 1.6, 5.0]);
    }

    fn create_glb_file(json: &[u8], binary_data: &[u8]) -> Vec<u8>
    {
        let mut padded_json = json.to_vec();

        while padded_json.len() % 4 != 0
        {
            padded_json.push(b' ');
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
        file.extend_from_slice(binary_data);

        return file;
    }
}
