use graphics::{GraphicsObject, GraphicsVertex};

// data structures
/// A coloured cube that rotates at a fixed rate.
pub struct SpinningCube
{
    object_identifier: u64,
    position: [f32; 3],
    rotation_radians_per_second: [f32; 3],
}
// data structures

// domain constants
const CUBE_INDEX_COUNT: usize = 36;
const CUBE_MESH_IDENTIFIER: u64 = 1;
const CUBE_MATERIAL_IDENTIFIER: u64 = 1;
const CUBE_BOUNDING_RADIUS: f32 = 1.3;
// domain constants

// private domain language
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
    0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 8, 9, 10, 8, 10, 11,
    12, 13, 14, 12, 14, 15, 16, 17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
];
// private domain language

impl SpinningCube
{
    /// Creates a spinning cube with a unique identifier, world position, and rotation rate.
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

    fn mesh_identifier(&self) -> u64
    {
        return CUBE_MESH_IDENTIFIER;
    }

    fn material_identifier(&self) -> u64
    {
        return CUBE_MATERIAL_IDENTIFIER;
    }

    fn vertices(&self) -> &[GraphicsVertex]
    {
        return &CUBE_VERTICES;
    }

    fn indices(&self) -> &[u16]
    {
        return &CUBE_INDICES;
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
        return CUBE_BOUNDING_RADIUS;
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

    #[test]
    fn cube_triangles_face_outward()
    {
        let mut triangle_start = 0;

        while triangle_start < CUBE_INDICES.len()
        {
            let first = CUBE_VERTICES[CUBE_INDICES[triangle_start] as usize].position;
            let second = CUBE_VERTICES[CUBE_INDICES[triangle_start + 1] as usize].position;
            let third = CUBE_VERTICES[CUBE_INDICES[triangle_start + 2] as usize].position;
            let first_edge = subtract_points(second, first);
            let second_edge = subtract_points(third, first);
            let normal = cross_product(first_edge, second_edge);
            let face_position = add_points(add_points(first, second), third);

            assert!(dot_product(normal, face_position) > 0.0);

            triangle_start += 3;
        }
    }

    fn subtract_points(left: [f32; 3], right: [f32; 3]) -> [f32; 3]
    {
        return [
            left[0] - right[0],
            left[1] - right[1],
            left[2] - right[2],
        ];
    }

    fn add_points(left: [f32; 3], right: [f32; 3]) -> [f32; 3]
    {
        return [
            left[0] + right[0],
            left[1] + right[1],
            left[2] + right[2],
        ];
    }

    fn cross_product(left: [f32; 3], right: [f32; 3]) -> [f32; 3]
    {
        return [
            left[1] * right[2] - left[2] * right[1],
            left[2] * right[0] - left[0] * right[2],
            left[0] * right[1] - left[1] * right[0],
        ];
    }

    fn dot_product(left: [f32; 3], right: [f32; 3]) -> f32
    {
        return left[0] * right[0] + left[1] * right[1] + left[2] * right[2];
    }
}
