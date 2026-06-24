use std::{ffi::c_void, mem::size_of, slice};
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

// data structures
pub(crate) struct MetricsOverlay
{
    vertex_buffer: ID3D11Buffer,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    input_layout: ID3D11InputLayout,
    rasterizer_state: ID3D11RasterizerState,
    viewport_width: f32,
    viewport_height: f32,
    vertex_count: u32,
    is_visible: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MetricsVertex
{
    position: [f32; 2],
    colour: [f32; 4],
}
// data structures

// domain constants
const METRICS_OVERLAY_SHADER_SOURCE: &[u8] = br#"
struct VertexInput
{
    float2 position : POSITION;
    float4 colour : COLOR;
};

struct PixelInput
{
    float4 position : SV_POSITION;
    float4 colour : COLOR;
};

PixelInput vertex_main(VertexInput input)
{
    PixelInput output;
    output.position = float4(input.position, 0.0f, 1.0f);
    output.colour = input.colour;
    return output;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    return input.colour;
}
"#;
const METRICS_OVERLAY_SHADER_NAME: PCSTR = PCSTR(c"metrics_overlay.hlsl".as_ptr().cast());
const VERTEX_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"vertex_main".as_ptr().cast());
const PIXEL_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"pixel_main".as_ptr().cast());
const VERTEX_SHADER_PROFILE: PCSTR = PCSTR(c"vs_5_0".as_ptr().cast());
const PIXEL_SHADER_PROFILE: PCSTR = PCSTR(c"ps_5_0".as_ptr().cast());
const POSITION_SEMANTIC: PCSTR = PCSTR(c"POSITION".as_ptr().cast());
const COLOUR_SEMANTIC: PCSTR = PCSTR(c"COLOR".as_ptr().cast());
const MAXIMUM_VERTEX_COUNT: usize = 65_536;
const VERTEX_STRIDE: u32 = size_of::<MetricsVertex>() as u32;
const OVERLAY_LEFT: f32 = 16.0;
const OVERLAY_TOP: f32 = 16.0;
const OVERLAY_WIDTH: f32 = 470.0;
const OVERLAY_HEIGHT: f32 = 190.0;
const GLYPH_SCALE: f32 = 3.0;
const GLYPH_WIDTH: f32 = 5.0;
const GLYPH_HEIGHT: f32 = 7.0;
const CHARACTER_ADVANCE: f32 = 18.0;
const LINE_ADVANCE: f32 = 27.0;
const BACKGROUND_COLOUR: [f32; 4] = [0.02, 0.03, 0.05, 1.0];
const TEXT_COLOUR: [f32; 4] = [0.85, 0.95, 1.0, 1.0];
// domain constants

impl MetricsOverlay
{
    pub(crate) unsafe fn create(
        device: &ID3D11Device,
        viewport_width: u32,
        viewport_height: u32,
    ) -> Result<Self>
    {
        let vertex_shader_bytecode = compile_shader(
            METRICS_OVERLAY_SHADER_SOURCE,
            METRICS_OVERLAY_SHADER_NAME,
            VERTEX_SHADER_ENTRY_POINT,
            VERTEX_SHADER_PROFILE,
        )?;
        let pixel_shader_bytecode = compile_shader(
            METRICS_OVERLAY_SHADER_SOURCE,
            METRICS_OVERLAY_SHADER_NAME,
            PIXEL_SHADER_ENTRY_POINT,
            PIXEL_SHADER_PROFILE,
        )?;

        return Ok(Self {
            vertex_buffer: Self::create_vertex_buffer(device)?,
            vertex_shader: create_vertex_shader(device, &vertex_shader_bytecode)?,
            pixel_shader: create_pixel_shader(device, &pixel_shader_bytecode)?,
            input_layout: Self::create_input_layout(device, &vertex_shader_bytecode)?,
            rasterizer_state: Self::create_rasterizer_state(device)?,
            viewport_width: viewport_width as f32,
            viewport_height: viewport_height as f32,
            vertex_count: 0,
            is_visible: false,
        });
    }

    pub(crate) fn set_visible(&mut self, is_visible: bool)
    {
        self.is_visible = is_visible;
    }

    pub(crate) unsafe fn set_text(
        &mut self,
        device_context: &ID3D11DeviceContext,
        text: &str,
    ) -> Result<()>
    {
        let vertices = self.create_text_vertices(text);

        if vertices.len() > MAXIMUM_VERTEX_COUNT
        {
            return Err(Error::new(E_FAIL, "The metrics overlay text is too large."));
        }

        let vertex_count = vertices.len() as u32;
        let byte_count = vertex_count as usize * size_of::<MetricsVertex>();
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
        self.vertex_count = vertex_count;

        return Ok(());
    }

    pub(crate) unsafe fn render(
        &self,
        device_context: &ID3D11DeviceContext,
        render_target_view: &ID3D11RenderTargetView,
    )
    {
        if !self.is_visible || self.vertex_count == 0
        {
            return;
        }

        let render_targets = [Some(render_target_view.clone())];
        let vertex_buffers = [Some(self.vertex_buffer.clone())];
        let vertex_offset = 0;

        device_context.OMSetRenderTargets(Some(&render_targets), None);
        device_context.RSSetState(&self.rasterizer_state);
        device_context.IASetInputLayout(&self.input_layout);
        device_context.IASetVertexBuffers(
            0,
            1,
            Some(vertex_buffers.as_ptr()),
            Some(&VERTEX_STRIDE),
            Some(&vertex_offset),
        );
        device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        device_context.VSSetShader(&self.vertex_shader, None);
        device_context.PSSetShader(&self.pixel_shader, None);
        device_context.Draw(self.vertex_count, 0);
    }

    unsafe fn create_vertex_buffer(device: &ID3D11Device) -> Result<ID3D11Buffer>
    {
        let description = D3D11_BUFFER_DESC {
            ByteWidth: (MAXIMUM_VERTEX_COUNT * size_of::<MetricsVertex>()) as u32,
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let mut vertex_buffer = None;
        device.CreateBuffer(&description, None, Some(&mut vertex_buffer))?;
        return required_resource(vertex_buffer, "Direct3D returned no metrics overlay vertex buffer.");
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
                SemanticName: COLOUR_SEMANTIC,
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
        return required_resource(input_layout, "Direct3D returned no metrics overlay input layout.");
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
        return required_resource(rasterizer_state, "Direct3D returned no metrics overlay rasterizer state.");
    }

    fn create_text_vertices(&self, text: &str) -> Vec<MetricsVertex>
    {
        let mut vertices = Vec::with_capacity(16_384);
        let mut character_left = OVERLAY_LEFT + 12.0;
        let mut character_top = OVERLAY_TOP + 12.0;

        self.add_rectangle(
            &mut vertices,
            OVERLAY_LEFT,
            OVERLAY_TOP,
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
            BACKGROUND_COLOUR,
        );

        for character in text.chars()
        {
            if character == '\n'
            {
                character_left = OVERLAY_LEFT + 12.0;
                character_top += LINE_ADVANCE;
                continue;
            }

            self.add_glyph(&mut vertices, character.to_ascii_uppercase(), character_left, character_top);
            character_left += CHARACTER_ADVANCE;
        }

        return vertices;
    }

    fn add_glyph(
        &self,
        vertices: &mut Vec<MetricsVertex>,
        character: char,
        glyph_left: f32,
        glyph_top: f32,
    )
    {
        let glyph_rows = glyph_rows(character);
        let mut row_index = 0;

        while row_index < GLYPH_HEIGHT as usize
        {
            let mut column_index = 0;

            while column_index < GLYPH_WIDTH as usize
            {
                let pixel_mask = 1 << (GLYPH_WIDTH as usize - 1 - column_index);

                if glyph_rows[row_index] & pixel_mask != 0
                {
                    self.add_rectangle(
                        vertices,
                        glyph_left + column_index as f32 * GLYPH_SCALE,
                        glyph_top + row_index as f32 * GLYPH_SCALE,
                        GLYPH_SCALE,
                        GLYPH_SCALE,
                        TEXT_COLOUR,
                    );
                }

                column_index += 1;
            }

            row_index += 1;
        }
    }

    fn add_rectangle(
        &self,
        vertices: &mut Vec<MetricsVertex>,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        colour: [f32; 4],
    )
    {
        let left = left / self.viewport_width * 2.0 - 1.0;
        let right = left + width / self.viewport_width * 2.0;
        let top = 1.0 - top / self.viewport_height * 2.0;
        let bottom = top - height / self.viewport_height * 2.0;
        let top_left = MetricsVertex { position: [left, top], colour };
        let top_right = MetricsVertex { position: [right, top], colour };
        let bottom_left = MetricsVertex { position: [left, bottom], colour };
        let bottom_right = MetricsVertex { position: [right, bottom], colour };

        vertices.push(top_left);
        vertices.push(top_right);
        vertices.push(bottom_right);
        vertices.push(top_left);
        vertices.push(bottom_right);
        vertices.push(bottom_left);
    }
}

fn glyph_rows(character: char) -> [u8; 7]
{
    match character
    {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 14],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'J' => [7, 2, 2, 2, 2, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 21, 19, 17, 17, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '.' => [0, 0, 0, 0, 0, 12, 12],
        ':' => [0, 12, 12, 0, 12, 12, 0],
        '/' => [1, 2, 2, 4, 8, 8, 16],
        '%' => [17, 2, 4, 8, 16, 17, 0],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        _ => [0, 0, 0, 0, 0, 0, 0],
    }
}

unsafe fn compile_shader(source: &[u8], source_name: PCSTR, entry_point: PCSTR, profile: PCSTR) -> Result<Vec<u8>>
{
    let mut bytecode = None;
    D3DCompile(source.as_ptr().cast::<c_void>(), source.len(), source_name, None, None::<&ID3DInclude>, entry_point, profile, 0, 0, &mut bytecode, None)?;
    let bytecode = required_resource(bytecode, "The metrics overlay HLSL compiler returned no bytecode.")?;
    return Ok(slice::from_raw_parts(bytecode.GetBufferPointer().cast(), bytecode.GetBufferSize()).to_vec());
}

unsafe fn create_vertex_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11VertexShader>
{
    let mut vertex_shader = None;
    device.CreateVertexShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut vertex_shader))?;
    return required_resource(vertex_shader, "Direct3D returned no metrics overlay vertex shader.");
}

unsafe fn create_pixel_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11PixelShader>
{
    let mut pixel_shader = None;
    device.CreatePixelShader(bytecode, None::<&ID3D11ClassLinkage>, Some(&mut pixel_shader))?;
    return required_resource(pixel_shader, "Direct3D returned no metrics overlay pixel shader.");
}

fn required_resource<Resource>(resource: Option<Resource>, message: &'static str) -> Result<Resource>
{
    return resource.ok_or_else(|| Error::new(E_FAIL, message));
}
