// data structures
/// A vertex accepted by the graphics renderer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GraphicsVertex
{
    /// The vertex position in object-local coordinates.
    pub position: [f32; 3],

    /// The linear RGB vertex colour used when no texture is supplied.
    pub color: [f32; 3],
}
// data structures

/// A vertex emitted by a user-interface library for the graphics renderer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GraphicsUserInterfaceVertex
{
    /// Normalized device coordinates in the range `[-1.0, 1.0]`.
    pub position: [f32; 2],

    /// The linear RGBA colour for this vertex.
    pub color: [f32; 4],
}

/// External shader source supplied by a user-interface library.
#[derive(Clone, Copy)]
pub struct GraphicsUserInterfaceShader
{
    /// HLSL source embedded from an external shader file by the UI library.
    pub source: &'static [u8],

    /// Stable shader identity; this must change whenever the shader source changes.
    pub identifier: &'static str,
}

// graphics object contract
// Mesh indices must wind clockwise when viewed from the object's exterior.
// This is the Direct3D front-face convention used by this renderer.
/// Supplies mesh, material, and transform data that the renderer can draw.
pub trait GraphicsObject
{
    /// Returns the unique identifier for this loaded object instance.
    fn identifier(&self) -> u64;

    /// Returns the identifier used to share identical mesh GPU resources.
    fn mesh_identifier(&self) -> u64;

    /// Returns the identifier used to share material GPU resources.
    ///
    /// Objects with a shared mesh and material identifier share GPU resources.
    /// This value must change when the texture or surface appearance changes.
    fn material_identifier(&self) -> u64;

    /// Returns the mesh vertices in object-local coordinates.
    fn vertices(&self) -> &[GraphicsVertex];

    /// Returns clockwise triangle indices into [`GraphicsObject::vertices`].
    fn indices(&self) -> &[u16];

    /// Returns the object position in world coordinates.
    fn position(&self) -> [f32; 3];

    /// Returns the object's Euler rotation in radians at the supplied elapsed time.
    fn rotation_radians(&self, elapsed_seconds: f32) -> [f32; 3];

    /// Returns the radius of a sphere that encloses the object-local mesh.
    fn bounding_radius(&self) -> f32;

    /// Returns the texture dimensions in pixels, when the object has an RGBA texture.
    fn texture_size(&self) -> Option<[u32; 2]>
    {
        return None;
    }

    /// Returns RGBA8 texture pixels in row-major order, when the object has a texture.
    fn texture_pixels(&self) -> Option<&[u8]>
    {
        return None;
    }
}
// graphics object contract

/// Supplies immediate-mode user-interface geometry and its pixel shader to the renderer.
pub trait GraphicsUserInterface
{
    /// Returns the shader used to render the emitted UI vertices.
    fn shader(&self) -> GraphicsUserInterfaceShader;

    /// Returns non-indexed triangles in normalized device coordinates.
    fn vertices(&self) -> &[GraphicsUserInterfaceVertex];
}
