use windows::core::PCSTR;

use crate::graphics::{
    GraphicsObject, GraphicsShaderProgram, GraphicsVertex,
};

// data structures
pub struct SpinningCube
{
    object_identifier: u64,
    position: [f32; 3],
    rotation_radians_per_second: [f32; 3],
}
// data structures

// domain constants
const CUBE_INDEX_COUNT: usize = 36;
const VERTEX_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"vertex_main".as_ptr().cast());
const PIXEL_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"pixel_main".as_ptr().cast());
const VERTEX_SHADER_PROFILE: PCSTR = PCSTR(c"vs_5_0".as_ptr().cast());
const PIXEL_SHADER_PROFILE: PCSTR = PCSTR(c"ps_5_0".as_ptr().cast());
const CUBE_SHADER_NAME: PCSTR = PCSTR(c"spinning_cube.hlsl".as_ptr().cast());
// domain constants

// private domain language
const CUBE_SHADER_SOURCE: &[u8] = br#"
cbuffer Transform : register(b0)
{
    row_major float4x4 world_view_projection;
};

static const float cube_scale = 0.75f;

struct VertexInput
{
    float3 position : POSITION;
    float3 color : COLOR;
};

struct PixelInput
{
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

PixelInput vertex_main(VertexInput input)
{
    PixelInput output;
    output.position = mul(float4(input.position * cube_scale, 1.0f), world_view_projection);
    output.color = input.color;
    return output;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    return float4(input.color, 1.0f);
}
"#;

const CUBE_VERTICES: [GraphicsVertex; 24] = [
    GraphicsVertex { position: [-1.0, -1.0, -1.0], color: [0.85, 0.15, 0.15] },
    GraphicsVertex { position: [-1.0, 1.0, -1.0], color: [0.85, 0.15, 0.15] },
    GraphicsVertex { position: [1.0, 1.0, -1.0], color: [0.85, 0.15, 0.15] },
    GraphicsVertex { position: [1.0, -1.0, -1.0], color: [0.85, 0.15, 0.15] },
    GraphicsVertex { position: [-1.0, -1.0, 1.0], color: [0.15, 0.75, 0.85] },
    GraphicsVertex { position: [-1.0, 1.0, 1.0], color: [0.15, 0.75, 0.85] },
    GraphicsVertex { position: [1.0, 1.0, 1.0], color: [0.15, 0.75, 0.85] },
    GraphicsVertex { position: [1.0, -1.0, 1.0], color: [0.15, 0.75, 0.85] },
    GraphicsVertex { position: [-1.0, -1.0, -1.0], color: [0.20, 0.35, 0.85] },
    GraphicsVertex { position: [-1.0, -1.0, 1.0], color: [0.20, 0.35, 0.85] },
    GraphicsVertex { position: [-1.0, 1.0, 1.0], color: [0.20, 0.35, 0.85] },
    GraphicsVertex { position: [-1.0, 1.0, -1.0], color: [0.20, 0.35, 0.85] },
    GraphicsVertex { position: [1.0, -1.0, -1.0], color: [0.95, 0.50, 0.15] },
    GraphicsVertex { position: [1.0, 1.0, -1.0], color: [0.95, 0.50, 0.15] },
    GraphicsVertex { position: [1.0, 1.0, 1.0], color: [0.95, 0.50, 0.15] },
    GraphicsVertex { position: [1.0, -1.0, 1.0], color: [0.95, 0.50, 0.15] },
    GraphicsVertex { position: [-1.0, 1.0, -1.0], color: [0.35, 0.85, 0.30] },
    GraphicsVertex { position: [-1.0, 1.0, 1.0], color: [0.35, 0.85, 0.30] },
    GraphicsVertex { position: [1.0, 1.0, 1.0], color: [0.35, 0.85, 0.30] },
    GraphicsVertex { position: [1.0, 1.0, -1.0], color: [0.35, 0.85, 0.30] },
    GraphicsVertex { position: [-1.0, -1.0, 1.0], color: [0.60, 0.20, 0.75] },
    GraphicsVertex { position: [-1.0, -1.0, -1.0], color: [0.60, 0.20, 0.75] },
    GraphicsVertex { position: [1.0, -1.0, -1.0], color: [0.60, 0.20, 0.75] },
    GraphicsVertex { position: [1.0, -1.0, 1.0], color: [0.60, 0.20, 0.75] },
];

const CUBE_INDICES: [u16; CUBE_INDEX_COUNT] = [
    0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 8, 10, 9, 8, 11, 10,
    12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
];
// private domain language

impl SpinningCube
{
    pub fn new(
        object_identifier: u64,
        position: [f32; 3],
        rotation_radians_per_second: [f32; 3],
    ) -> Self
    {
        return Self {
            object_identifier,
            position,
            rotation_radians_per_second,
        };
    }
}

impl GraphicsObject for SpinningCube
{
    fn identifier(&self) -> u64
    {
        return self.object_identifier;
    }

    fn vertices(&self) -> &[GraphicsVertex]
    {
        return &CUBE_VERTICES;
    }

    fn indices(&self) -> &[u16]
    {
        return &CUBE_INDICES;
    }

    fn shader_program(&self) -> GraphicsShaderProgram
    {
        return GraphicsShaderProgram {
            source: CUBE_SHADER_SOURCE,
            source_name: CUBE_SHADER_NAME,
            vertex_entry_point: VERTEX_SHADER_ENTRY_POINT,
            vertex_profile: VERTEX_SHADER_PROFILE,
            pixel_entry_point: PIXEL_SHADER_ENTRY_POINT,
            pixel_profile: PIXEL_SHADER_PROFILE,
        };
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
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn cube_meets_the_graphics_object_contract()
    {
        let cube = SpinningCube::new(7, [1.0, 2.0, 3.0], [0.8, 1.2, 0.0]);

        assert_eq!(cube.identifier(), 7);
        assert_eq!(cube.vertices().len(), 24);
        assert_eq!(cube.indices().len(), CUBE_INDEX_COUNT);
        assert_eq!(cube.position(), [1.0, 2.0, 3.0]);
    }
}
