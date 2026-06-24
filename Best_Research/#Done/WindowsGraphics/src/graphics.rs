use std::{ffi::c_void, mem::{size_of, size_of_val}, slice};
use windows::{
    core::{Error, PCSTR, Result},
    Win32::{
        Foundation::{E_FAIL, HWND},
        Graphics::{
            Direct3D::{
                Fxc::D3DCompile, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL,
                D3D_FEATURE_LEVEL_11_0, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST, ID3DInclude,
            },
            Direct3D11::{
                D3D11CreateDeviceAndSwapChain, D3D11_BIND_CONSTANT_BUFFER,
                D3D11_BIND_DEPTH_STENCIL, D3D11_BIND_FLAG, D3D11_BIND_INDEX_BUFFER,
                D3D11_BIND_VERTEX_BUFFER, D3D11_BUFFER_DESC, D3D11_CLEAR_DEPTH,
                D3D11_CULL_NONE, D3D11_FILL_SOLID, D3D11_INPUT_ELEMENT_DESC,
                D3D11_INPUT_PER_VERTEX_DATA, D3D11_RASTERIZER_DESC, D3D11_SDK_VERSION,
                D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
                D3D11_VIEWPORT, ID3D11Buffer, ID3D11ClassLinkage, ID3D11DepthStencilView,
                ID3D11Device, ID3D11DeviceContext, ID3D11InputLayout, ID3D11PixelShader,
                ID3D11RasterizerState, ID3D11RenderTargetView, ID3D11Texture2D,
                ID3D11VertexShader,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_FORMAT_R16_UINT,
                    DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC,
                    DXGI_RATIONAL, DXGI_SAMPLE_DESC,
                },
                DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT, IDXGIAdapter, IDXGISwapChain,
            },
        },
    },
};

// data structures
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GraphicsVertex
{
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Clone, Copy)]
pub struct GraphicsShaderProgram
{
    pub source: &'static [u8],
    pub source_name: PCSTR,
    pub vertex_entry_point: PCSTR,
    pub vertex_profile: PCSTR,
    pub pixel_entry_point: PCSTR,
    pub pixel_profile: PCSTR,
}

pub struct Direct3DGraphics
{
    device: ID3D11Device,
    swap_chain: IDXGISwapChain,
    device_context: ID3D11DeviceContext,
    render_target_view: ID3D11RenderTargetView,
    depth_stencil_view: ID3D11DepthStencilView,
    rasterizer_state: ID3D11RasterizerState,
    transform_buffer: ID3D11Buffer,
    viewport: D3D11_VIEWPORT,
    loaded_objects: Vec<LoadedGraphicsObject>,
}

struct LoadedGraphicsObject
{
    object_identifier: u64,
    object: Box<dyn GraphicsObject>,
    vertex_buffer: ID3D11Buffer,
    index_buffer: ID3D11Buffer,
    index_count: u32,
    input_layout: ID3D11InputLayout,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
}

struct CreatedDirect3DDevice
{
    swap_chain: IDXGISwapChain,
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TransformBuffer
{
    world_view_projection: Matrix4x4,
}
// data structures

// graphics object contract
pub trait GraphicsObject
{
    fn identifier(&self) -> u64;
    fn vertices(&self) -> &[GraphicsVertex];
    fn indices(&self) -> &[u16];
    fn shader_program(&self) -> GraphicsShaderProgram;
    fn position(&self) -> [f32; 3];
    fn rotation_radians(&self, elapsed_seconds: f32) -> [f32; 3];
}
// graphics object contract

// private types
type Color = [f32; 4];
type Matrix4x4 = [[f32; 4]; 4];
type ShaderBytecode = Vec<u8>;
// private types

// domain constants
const CLEAR_COLOR: Color = [0.05, 0.08, 0.12, 1.0];
const VERTEX_STRIDE: u32 = size_of::<GraphicsVertex>() as u32;
const VERTICAL_FIELD_OF_VIEW_DEGREES: f32 = 60.0;
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 100.0;
const POSITION_SEMANTIC: PCSTR = PCSTR(c"POSITION".as_ptr().cast());
const COLOR_SEMANTIC: PCSTR = PCSTR(c"COLOR".as_ptr().cast());
// domain constants

pub fn create_direct3d_graphics(
    window_handle: HWND,
    viewport_width: u32,
    viewport_height: u32,
) -> Result<Direct3DGraphics>
{
    unsafe
    {
        return Direct3DGraphics::create_internal(window_handle, viewport_width, viewport_height);
    }
}

impl Direct3DGraphics
{
    pub fn add_object<GameObject>(&mut self, object: GameObject) -> Result<()>
    where
        GameObject: GraphicsObject + 'static,
    {
        unsafe
        {
            return self.load_object_internal(object);
        }
    }

    pub fn render(&self, elapsed_seconds: f32) -> Result<()>
    {
        unsafe
        {
            return self.render_internal(elapsed_seconds);
        }
    }

    unsafe fn create_internal(
        window_handle: HWND,
        viewport_width: u32,
        viewport_height: u32,
    ) -> Result<Self>
    {
        let created_device = Self::create_device_and_swap_chain(window_handle, viewport_width, viewport_height)?;
        let back_buffer = created_device.swap_chain.GetBuffer::<ID3D11Texture2D>(0)?;
        let render_target_view = Self::create_render_target_view(&created_device.device, &back_buffer)?;
        let depth_stencil_view = Self::create_depth_stencil_view(&created_device.device, viewport_width, viewport_height)?;
        let rasterizer_state = Self::create_rasterizer_state(&created_device.device)?;
        let transform_buffer = Self::create_buffer(
            &created_device.device,
            &[TransformBuffer { world_view_projection: identity_matrix() }],
            D3D11_BIND_CONSTANT_BUFFER,
        )?;
        let viewport = D3D11_VIEWPORT {
            TopLeftX: 0.0, TopLeftY: 0.0,
            Width: viewport_width as f32, Height: viewport_height as f32,
            MinDepth: 0.0, MaxDepth: 1.0,
        };

        return Ok(Self {
            device: created_device.device,
            swap_chain: created_device.swap_chain,
            device_context: created_device.device_context,
            render_target_view,
            depth_stencil_view,
            rasterizer_state,
            transform_buffer,
            viewport,
            loaded_objects: Vec::new(),
        });
    }

    unsafe fn load_object_internal<GameObject>(&mut self, object: GameObject) -> Result<()>
    where
        GameObject: GraphicsObject + 'static,
    {
        if self.find_loaded_object(object.identifier()).is_ok()
        {
            return Err(Error::new(E_FAIL, "Graphics object identifier is already loaded."));
        }

        let shader_program = object.shader_program();
        let index_count_as_usize = object.indices().len();

        if index_count_as_usize > u32::MAX as usize
        {
            return Err(Error::new(E_FAIL, "Graphics object has too many indices."));
        }

        let index_count = index_count_as_usize as u32;
        let vertex_buffer = Self::create_buffer(&self.device, object.vertices(), D3D11_BIND_VERTEX_BUFFER)?;
        let index_buffer = Self::create_buffer(&self.device, object.indices(), D3D11_BIND_INDEX_BUFFER)?;
        let vertex_shader_bytecode = Self::compile_shader(
            shader_program.source, shader_program.source_name,
            shader_program.vertex_entry_point, shader_program.vertex_profile,
        )?;
        let pixel_shader_bytecode = Self::compile_shader(
            shader_program.source, shader_program.source_name,
            shader_program.pixel_entry_point, shader_program.pixel_profile,
        )?;
        let input_layout = Self::create_input_layout(&self.device, &vertex_shader_bytecode)?;
        let vertex_shader = Self::create_vertex_shader(&self.device, &vertex_shader_bytecode)?;
        let pixel_shader = Self::create_pixel_shader(&self.device, &pixel_shader_bytecode)?;

        self.loaded_objects.push(LoadedGraphicsObject {
            object_identifier: object.identifier(), object: Box::new(object), vertex_buffer, index_buffer, index_count,
            input_layout, vertex_shader, pixel_shader,
        });

        return Ok(());
    }

    unsafe fn render_internal(&self, elapsed_seconds: f32) -> Result<()>
    {
        let render_targets = [Some(self.render_target_view.clone())];
        let transform_buffers = [Some(self.transform_buffer.clone())];

        self.device_context.OMSetRenderTargets(Some(&render_targets), Some(&self.depth_stencil_view));
        self.device_context.ClearRenderTargetView(&self.render_target_view, &CLEAR_COLOR);
        self.device_context.ClearDepthStencilView(&self.depth_stencil_view, D3D11_CLEAR_DEPTH.0, 1.0, 0);
        self.device_context.RSSetState(&self.rasterizer_state);
        self.device_context.RSSetViewports(Some(&[self.viewport]));

        for loaded_object in &self.loaded_objects
        {
            let vertex_buffers = [Some(loaded_object.vertex_buffer.clone())];
            let transform = self.create_transform(loaded_object.object.as_ref(), elapsed_seconds);
            let vertex_offset = 0;

            self.device_context.IASetInputLayout(&loaded_object.input_layout);
            self.device_context.IASetVertexBuffers(0, 1, Some(vertex_buffers.as_ptr()), Some(&VERTEX_STRIDE), Some(&vertex_offset));
            self.device_context.IASetIndexBuffer(&loaded_object.index_buffer, DXGI_FORMAT_R16_UINT, 0);
            self.device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.device_context.VSSetShader(&loaded_object.vertex_shader, None);
            self.device_context.PSSetShader(&loaded_object.pixel_shader, None);
            self.device_context.VSSetConstantBuffers(0, Some(&transform_buffers));
            self.device_context.UpdateSubresource(&self.transform_buffer, 0, None, (&transform as *const TransformBuffer).cast::<c_void>(), 0, 0);
            self.device_context.DrawIndexed(loaded_object.index_count, 0, 0);
        }

        self.swap_chain.Present(1, DXGI_PRESENT(0)).ok()?;

        return Ok(());
    }

    fn find_loaded_object(&self, object_identifier: u64) -> Result<&LoadedGraphicsObject>
    {
        for loaded_object in &self.loaded_objects
        {
            if loaded_object.object_identifier == object_identifier
            {
                return Ok(loaded_object);
            }
        }

        return Err(Error::new(E_FAIL, "Graphics object has not been loaded."));
    }

    fn create_transform(&self, object: &dyn GraphicsObject, elapsed_seconds: f32) -> TransformBuffer
    {
        let rotation = object.rotation_radians(elapsed_seconds);
        let position = object.position();
        let (sine_x, cosine_x) = rotation[0].sin_cos();
        let (sine_y, cosine_y) = rotation[1].sin_cos();
        let (sine_z, cosine_z) = rotation[2].sin_cos();
        let rotation_x = [[1.0, 0.0, 0.0, 0.0], [0.0, cosine_x, sine_x, 0.0], [0.0, -sine_x, cosine_x, 0.0], [0.0, 0.0, 0.0, 1.0]];
        let rotation_y = [[cosine_y, 0.0, -sine_y, 0.0], [0.0, 1.0, 0.0, 0.0], [sine_y, 0.0, cosine_y, 0.0], [0.0, 0.0, 0.0, 1.0]];
        let rotation_z = [[cosine_z, sine_z, 0.0, 0.0], [-sine_z, cosine_z, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];
        let translation = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [position[0], position[1], position[2], 1.0]];
        let focal_length = 1.0 / ((VERTICAL_FIELD_OF_VIEW_DEGREES.to_radians() * 0.5).tan());
        let aspect_ratio = self.viewport.Width / self.viewport.Height;
        let perspective = [[focal_length / aspect_ratio, 0.0, 0.0, 0.0], [0.0, focal_length, 0.0, 0.0], [0.0, 0.0, FAR_PLANE / (FAR_PLANE - NEAR_PLANE), 1.0], [0.0, 0.0, -NEAR_PLANE * FAR_PLANE / (FAR_PLANE - NEAR_PLANE), 0.0]];
        let rotation_xy = multiply_matrices(rotation_x, rotation_y);
        let rotation_xyz = multiply_matrices(rotation_xy, rotation_z);
        let world = multiply_matrices(rotation_xyz, translation);

        return TransformBuffer { world_view_projection: multiply_matrices(world, perspective) };
    }

    unsafe fn create_device_and_swap_chain(window_handle: HWND, width: u32, height: u32) -> Result<CreatedDirect3DDevice>
    {
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC { Width: width, Height: height, RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 }, Format: DXGI_FORMAT_R8G8B8A8_UNORM, ScanlineOrdering: Default::default(), Scaling: Default::default() },
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT, BufferCount: 1, OutputWindow: window_handle,
            Windowed: true.into(), SwapEffect: DXGI_SWAP_EFFECT_DISCARD, Flags: 0,
        };
        let feature_levels = [D3D_FEATURE_LEVEL_11_0];
        let mut swap_chain = None;
        let mut device = None;
        let mut device_context = None;
        let mut selected_feature_level = D3D_FEATURE_LEVEL::default();
        D3D11CreateDeviceAndSwapChain(None::<&IDXGIAdapter>, D3D_DRIVER_TYPE_HARDWARE, Default::default(), Default::default(), Some(&feature_levels), D3D11_SDK_VERSION, Some(&swap_chain_desc), Some(&mut swap_chain), Some(&mut device), Some(&mut selected_feature_level), Some(&mut device_context))?;

        return Ok(CreatedDirect3DDevice {
            swap_chain: required_resource(swap_chain, "Direct3D returned no swap chain.")?,
            device: required_resource(device, "Direct3D returned no device.")?,
            device_context: required_resource(device_context, "Direct3D returned no device context.")?,
        });
    }

    unsafe fn create_render_target_view(device: &ID3D11Device, back_buffer: &ID3D11Texture2D) -> Result<ID3D11RenderTargetView>
    {
        let mut render_target_view = None;
        device.CreateRenderTargetView(back_buffer, None, Some(&mut render_target_view))?;
        return required_resource(render_target_view, "Direct3D returned no render-target view.");
    }

    unsafe fn create_depth_stencil_view(device: &ID3D11Device, width: u32, height: u32) -> Result<ID3D11DepthStencilView>
    {
        let desc = D3D11_TEXTURE2D_DESC { Width: width, Height: height, MipLevels: 1, ArraySize: 1, Format: DXGI_FORMAT_D24_UNORM_S8_UINT, SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 }, Usage: D3D11_USAGE_DEFAULT, BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32, CPUAccessFlags: 0, MiscFlags: 0 };
        let mut depth_buffer = None;
        let mut depth_stencil_view = None;
        device.CreateTexture2D(&desc, None, Some(&mut depth_buffer))?;
        let depth_buffer = required_resource(depth_buffer, "Direct3D returned no depth buffer.")?;
        device.CreateDepthStencilView(&depth_buffer, None, Some(&mut depth_stencil_view))?;
        return required_resource(depth_stencil_view, "Direct3D returned no depth-stencil view.");
    }

    unsafe fn create_rasterizer_state(device: &ID3D11Device) -> Result<ID3D11RasterizerState>
    {
        let desc = D3D11_RASTERIZER_DESC { FillMode: D3D11_FILL_SOLID, CullMode: D3D11_CULL_NONE, DepthClipEnable: true.into(), ..Default::default() };
        let mut rasterizer_state = None;
        device.CreateRasterizerState(&desc, Some(&mut rasterizer_state))?;
        return required_resource(rasterizer_state, "Direct3D returned no rasterizer state.");
    }

    unsafe fn create_buffer<BufferElement>(device: &ID3D11Device, elements: &[BufferElement], bind_flags: D3D11_BIND_FLAG) -> Result<ID3D11Buffer>
    {
        let byte_width_as_usize = size_of_val(elements);

        if byte_width_as_usize > u32::MAX as usize
        {
            return Err(Error::new(E_FAIL, "Direct3D buffer is too large."));
        }

        let byte_width = byte_width_as_usize as u32;
        let desc = D3D11_BUFFER_DESC { ByteWidth: byte_width, Usage: D3D11_USAGE_DEFAULT, BindFlags: bind_flags.0 as u32, CPUAccessFlags: 0, MiscFlags: 0, StructureByteStride: 0 };
        let initial_data = D3D11_SUBRESOURCE_DATA { pSysMem: elements.as_ptr().cast::<c_void>(), SysMemPitch: 0, SysMemSlicePitch: 0 };
        let mut buffer = None;
        device.CreateBuffer(&desc, Some(&initial_data), Some(&mut buffer))?;
        return required_resource(buffer, "Direct3D returned no buffer.");
    }

    unsafe fn compile_shader(source: &[u8], source_name: PCSTR, entry_point: PCSTR, profile: PCSTR) -> Result<ShaderBytecode>
    {
        let mut bytecode = None;
        D3DCompile(source.as_ptr().cast::<c_void>(), source.len(), source_name, None, None::<&ID3DInclude>, entry_point, profile, 0, 0, &mut bytecode, None)?;
        let bytecode = required_resource(bytecode, "The HLSL compiler returned no bytecode.")?;
        return Ok(slice::from_raw_parts(bytecode.GetBufferPointer().cast(), bytecode.GetBufferSize()).to_vec());
    }

    unsafe fn create_vertex_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11VertexShader>
    {
        let mut vertex_shader = None;
        device.CreateVertexShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut vertex_shader))?;
        return required_resource(vertex_shader, "Direct3D returned no vertex shader.");
    }

    unsafe fn create_pixel_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11PixelShader>
    {
        let mut pixel_shader = None;
        device.CreatePixelShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut pixel_shader))?;
        return required_resource(pixel_shader, "Direct3D returned no pixel shader.");
    }

    unsafe fn create_input_layout(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11InputLayout>
    {
        let elements = [
            D3D11_INPUT_ELEMENT_DESC { SemanticName: POSITION_SEMANTIC, SemanticIndex: 0, Format: DXGI_FORMAT_R32G32B32_FLOAT, InputSlot: 0, AlignedByteOffset: 0, InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA, InstanceDataStepRate: 0 },
            D3D11_INPUT_ELEMENT_DESC { SemanticName: COLOR_SEMANTIC, SemanticIndex: 0, Format: DXGI_FORMAT_R32G32B32_FLOAT, InputSlot: 0, AlignedByteOffset: 12, InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA, InstanceDataStepRate: 0 },
        ];
        let mut input_layout = None;
        device.CreateInputLayout(&elements, bytecode, Some(&mut input_layout))?;
        return required_resource(input_layout, "Direct3D returned no input layout.");
    }
}

fn required_resource<Direct3DResource>(resource: Option<Direct3DResource>, message: &str) -> Result<Direct3DResource>
{
    return resource.ok_or_else(|| Error::new(E_FAIL, message));
}

fn identity_matrix() -> Matrix4x4
{
    return [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];
}

fn multiply_matrices(left: Matrix4x4, right: Matrix4x4) -> Matrix4x4
{
    let mut result = [[0.0; 4]; 4];
    for row in 0..4 { for column in 0..4 { for index in 0..4 { result[row][column] += left[row][index] * right[index][column]; } } }
    return result;
}
