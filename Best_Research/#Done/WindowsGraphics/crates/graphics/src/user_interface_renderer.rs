use std::{ffi::c_void, mem::{size_of, size_of_val}, slice};
use windows::{
    core::{Error, PCSTR, Result},
    Win32::{
        Foundation::E_FAIL,
        Graphics::{
            Direct3D::{Fxc::D3DCompile, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST, ID3DInclude},
            Direct3D11::{
                D3D11_BIND_VERTEX_BUFFER, D3D11_BOX, D3D11_BUFFER_DESC, D3D11_CULL_NONE,
                D3D11_FILL_SOLID, D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA,
                D3D11_RASTERIZER_DESC, D3D11_USAGE_DEFAULT,
                ID3D11Buffer, ID3D11ClassLinkage, ID3D11Device, ID3D11DeviceContext,
                ID3D11InputLayout, ID3D11PixelShader, ID3D11RasterizerState,
                ID3D11RenderTargetView, ID3D11VertexShader,
            },
            Dxgi::Common::{DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT},
        },
    },
};

use crate::{GraphicsUserInterface, GraphicsUserInterfaceShader, GraphicsUserInterfaceVertex};

// data structures
pub(crate) struct UserInterfaceRenderer
{
    vertex_buffer: ID3D11Buffer,
    rasterizer_state: ID3D11RasterizerState,
    shader_pipeline: Option<UserInterfaceShaderPipeline>,
    vertex_count: u32,
}

struct UserInterfaceShaderPipeline
{
    shader_identifier: &'static str,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    input_layout: ID3D11InputLayout,
}
// data structures

// domain constants
const USER_INTERFACE_SHADER_NAME: PCSTR = PCSTR(c"user_interface.hlsl".as_ptr().cast());
const VERTEX_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"vertex_main".as_ptr().cast());
const PIXEL_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"pixel_main".as_ptr().cast());
const VERTEX_SHADER_PROFILE: PCSTR = PCSTR(c"vs_5_0".as_ptr().cast());
const PIXEL_SHADER_PROFILE: PCSTR = PCSTR(c"ps_5_0".as_ptr().cast());
const POSITION_SEMANTIC: PCSTR = PCSTR(c"POSITION".as_ptr().cast());
const COLOR_SEMANTIC: PCSTR = PCSTR(c"COLOR".as_ptr().cast());
const MAXIMUM_VERTEX_COUNT: usize = 65_536;
const VERTEX_STRIDE: u32 = size_of::<GraphicsUserInterfaceVertex>() as u32;
// domain constants

impl UserInterfaceRenderer
{
    pub(crate) unsafe fn create(device: &ID3D11Device) -> Result<Self>
    {
        return Ok(Self {
            vertex_buffer: Self::create_vertex_buffer(device)?,
            rasterizer_state: Self::create_rasterizer_state(device)?,
            shader_pipeline: None,
            vertex_count: 0,
        });
    }

    pub(crate) unsafe fn submit<UserInterface>(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        user_interface: &UserInterface,
    ) -> Result<()>
    where
        UserInterface: GraphicsUserInterface,
    {
        let vertices = user_interface.vertices();

        if vertices.len() > MAXIMUM_VERTEX_COUNT
        {
            return Err(Error::new(E_FAIL, "The user interface contains too many vertices."));
        }

        let shader = user_interface.shader();
        self.ensure_shader_pipeline(device, shader)?;
        self.vertex_count = vertices.len() as u32;

        if vertices.is_empty()
        {
            return Ok(());
        }

        let byte_count = size_of_val(vertices);
        let update_box = D3D11_BOX {
            left: 0,
            top: 0,
            front: 0,
            right: byte_count as u32,
            bottom: 1,
            back: 1,
        };
        device_context.UpdateSubresource(
            &self.vertex_buffer,
            0,
            Some(&update_box),
            vertices.as_ptr().cast::<c_void>(),
            0,
            0,
        );

        return Ok(());
    }

    pub(crate) unsafe fn render(
        &self,
        device_context: &ID3D11DeviceContext,
        render_target_view: &ID3D11RenderTargetView,
    )
    {
        if self.vertex_count == 0
        {
            return;
        }

        let shader_pipeline = match &self.shader_pipeline
        {
            Some(shader_pipeline) => shader_pipeline,
            None => return,
        };
        let render_targets = [Some(render_target_view.clone())];
        let vertex_buffers = [Some(self.vertex_buffer.clone())];
        let vertex_offset = 0;

        device_context.OMSetRenderTargets(Some(&render_targets), None);
        device_context.RSSetState(&self.rasterizer_state);
        device_context.IASetInputLayout(&shader_pipeline.input_layout);
        device_context.IASetVertexBuffers(
            0,
            1,
            Some(vertex_buffers.as_ptr()),
            Some(&VERTEX_STRIDE),
            Some(&vertex_offset),
        );
        device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        device_context.VSSetShader(&shader_pipeline.vertex_shader, None);
        device_context.PSSetShader(&shader_pipeline.pixel_shader, None);
        device_context.Draw(self.vertex_count, 0);
    }

    unsafe fn create_vertex_buffer(device: &ID3D11Device) -> Result<ID3D11Buffer>
    {
        let description = D3D11_BUFFER_DESC {
            ByteWidth: (MAXIMUM_VERTEX_COUNT * size_of::<GraphicsUserInterfaceVertex>()) as u32,
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let mut vertex_buffer = None;
        device.CreateBuffer(&description, None, Some(&mut vertex_buffer))?;
        return required_resource(vertex_buffer, "Direct3D returned no user-interface vertex buffer.");
    }

    unsafe fn create_rasterizer_state(device: &ID3D11Device) -> Result<ID3D11RasterizerState>
    {
        let description = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_NONE,
            DepthClipEnable: true.into(),
            ..Default::default()
        };
        let mut rasterizer_state = None;
        device.CreateRasterizerState(&description, Some(&mut rasterizer_state))?;
        return required_resource(rasterizer_state, "Direct3D returned no user-interface rasterizer state.");
    }

    unsafe fn ensure_shader_pipeline(
        &mut self,
        device: &ID3D11Device,
        shader: GraphicsUserInterfaceShader,
    ) -> Result<()>
    {
        if let Some(shader_pipeline) = &self.shader_pipeline
        {
            if shader_pipeline.shader_identifier == shader.identifier
            {
                return Ok(());
            }
        }

        let vertex_shader_bytecode = compile_shader(
            shader.source,
            VERTEX_SHADER_ENTRY_POINT,
            VERTEX_SHADER_PROFILE,
        )?;
        let pixel_shader_bytecode = compile_shader(
            shader.source,
            PIXEL_SHADER_ENTRY_POINT,
            PIXEL_SHADER_PROFILE,
        )?;
        self.shader_pipeline = Some(UserInterfaceShaderPipeline {
            shader_identifier: shader.identifier,
            vertex_shader: create_vertex_shader(device, &vertex_shader_bytecode)?,
            pixel_shader: create_pixel_shader(device, &pixel_shader_bytecode)?,
            input_layout: create_input_layout(device, &vertex_shader_bytecode)?,
        });

        return Ok(());
    }
}

unsafe fn compile_shader(source: &[u8], entry_point: PCSTR, profile: PCSTR) -> Result<Vec<u8>>
{
    let mut bytecode = None;
    D3DCompile(
        source.as_ptr().cast::<c_void>(),
        source.len(),
        USER_INTERFACE_SHADER_NAME,
        None,
        None::<&ID3DInclude>,
        entry_point,
        profile,
        0,
        0,
        &mut bytecode,
        None,
    )?;
    let bytecode = required_resource(bytecode, "The user-interface HLSL compiler returned no bytecode.")?;
    return Ok(slice::from_raw_parts(bytecode.GetBufferPointer().cast(), bytecode.GetBufferSize()).to_vec());
}

unsafe fn create_vertex_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11VertexShader>
{
    let mut vertex_shader = None;
    device.CreateVertexShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut vertex_shader))?;
    return required_resource(vertex_shader, "Direct3D returned no user-interface vertex shader.");
}

unsafe fn create_pixel_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11PixelShader>
{
    let mut pixel_shader = None;
    device.CreatePixelShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut pixel_shader))?;
    return required_resource(pixel_shader, "Direct3D returned no user-interface pixel shader.");
}

unsafe fn create_input_layout(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11InputLayout>
{
    let elements = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: POSITION_SEMANTIC,
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: COLOR_SEMANTIC,
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 8,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];
    let mut input_layout = None;
    device.CreateInputLayout(&elements, bytecode, Some(&mut input_layout))?;
    return required_resource(input_layout, "Direct3D returned no user-interface input layout.");
}

fn required_resource<Resource>(resource: Option<Resource>, message: &'static str) -> Result<Resource>
{
    return resource.ok_or_else(|| Error::new(E_FAIL, message));
}
