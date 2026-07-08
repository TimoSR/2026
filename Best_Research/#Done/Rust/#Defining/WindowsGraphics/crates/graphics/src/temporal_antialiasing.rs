use std::{ffi::c_void, mem::size_of, slice};
use windows::{
    core::{Error, PCSTR, Result},
    Win32::{
        Foundation::E_FAIL,
        Graphics::{
            Direct3D::{
                Fxc::D3DCompile, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST, ID3DInclude,
            },
            Direct3D11::{
                D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_RENDER_TARGET,
                D3D11_BIND_SHADER_RESOURCE, D3D11_BUFFER_DESC, D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                D3D11_SAMPLER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC,
                D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_USAGE_DEFAULT, ID3D11Buffer,
                ID3D11ClassLinkage, ID3D11Device, ID3D11DeviceContext, ID3D11PixelShader,
                ID3D11RenderTargetView, ID3D11SamplerState, ID3D11ShaderResourceView,
                ID3D11Texture2D, ID3D11VertexShader,
            },
            Dxgi::Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC},
        },
    },
};

// data structures
pub(crate) struct TemporalAntialiasing
{
    history_targets: [TemporalHistoryTarget; 2],
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    sampler_state: ID3D11SamplerState,
    constants_buffer: ID3D11Buffer,
    read_history_index: usize,
    is_history_valid: bool,
    frame_index: usize,
}

struct TemporalHistoryTarget
{
    texture: ID3D11Texture2D,
    render_target_view: ID3D11RenderTargetView,
    shader_resource_view: ID3D11ShaderResourceView,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TemporalAntialiasingConstants
{
    render_target_size: [f32; 2],
    history_weight: f32,
    is_history_valid: u32,
}
// data structures

// domain constants
const TEMPORAL_ANTIALIASING_SHADER_SOURCE: &[u8] = include_bytes!("../shaders/temporal_antialiasing.hlsl");
const TEMPORAL_ANTIALIASING_SHADER_NAME: PCSTR = PCSTR(c"temporal_antialiasing.hlsl".as_ptr().cast());
const VERTEX_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"vertex_main".as_ptr().cast());
const PIXEL_SHADER_ENTRY_POINT: PCSTR = PCSTR(c"pixel_main".as_ptr().cast());
const VERTEX_SHADER_PROFILE: PCSTR = PCSTR(c"vs_5_0".as_ptr().cast());
const PIXEL_SHADER_PROFILE: PCSTR = PCSTR(c"ps_5_0".as_ptr().cast());
const TEMPORAL_HISTORY_WEIGHT: f32 = 0.7;
const TEMPORAL_JITTER_PATTERN: [[f32; 2]; 8] = [
    [0.5, 0.33333334],
    [0.25, 0.6666667],
    [0.75, 0.11111111],
    [0.125, 0.44444445],
    [0.625, 0.7777778],
    [0.375, 0.22222222],
    [0.875, 0.5555556],
    [0.0625, 0.8888889],
];
// domain constants

impl TemporalAntialiasing
{
    pub(crate) unsafe fn create(
        device: &ID3D11Device,
        width: u32,
        height: u32,
    ) -> Result<Self>
    {
        let first_history_target = Self::create_history_target(device, width, height)?;
        let second_history_target = Self::create_history_target(device, width, height)?;
        let vertex_shader_bytecode = compile_shader(
            TEMPORAL_ANTIALIASING_SHADER_SOURCE,
            TEMPORAL_ANTIALIASING_SHADER_NAME,
            VERTEX_SHADER_ENTRY_POINT,
            VERTEX_SHADER_PROFILE,
        )?;
        let pixel_shader_bytecode = compile_shader(
            TEMPORAL_ANTIALIASING_SHADER_SOURCE,
            TEMPORAL_ANTIALIASING_SHADER_NAME,
            PIXEL_SHADER_ENTRY_POINT,
            PIXEL_SHADER_PROFILE,
        )?;
        let vertex_shader = create_vertex_shader(device, &vertex_shader_bytecode)?;
        let pixel_shader = create_pixel_shader(device, &pixel_shader_bytecode)?;
        let sampler_state = Self::create_sampler_state(device)?;
        let constants_buffer = Self::create_constants_buffer(device, width, height)?;

        return Ok(Self {
            history_targets: [first_history_target, second_history_target],
            vertex_shader,
            pixel_shader,
            sampler_state,
            constants_buffer,
            read_history_index: 0,
            is_history_valid: false,
            frame_index: 0,
        });
    }

    pub(crate) fn reset_history(&mut self)
    {
        self.read_history_index = 0;
        self.is_history_valid = false;
        self.frame_index = 0;
    }

    pub(crate) fn jitter_in_normalized_device_coordinates(
        &self,
        width: f32,
        height: f32,
    ) -> [f32; 2]
    {
        return temporal_jitter_in_normalized_device_coordinates(self.frame_index, width, height);
    }

    pub(crate) unsafe fn resolve(
        &mut self,
        device_context: &ID3D11DeviceContext,
        current_frame_shader_resource_view: &ID3D11ShaderResourceView,
        destination_back_buffer: &ID3D11Texture2D,
        width: f32,
        height: f32,
    )
    {
        let write_history_index = 1 - self.read_history_index;
        let render_targets = [Some(self.history_targets[write_history_index].render_target_view.clone())];
        let shader_resources = [
            Some(current_frame_shader_resource_view.clone()),
            Some(self.history_targets[self.read_history_index].shader_resource_view.clone()),
        ];
        let samplers = [Some(self.sampler_state.clone())];
        let constants_buffers = [Some(self.constants_buffer.clone())];
        let constants = TemporalAntialiasingConstants {
            render_target_size: [width, height],
            history_weight: TEMPORAL_HISTORY_WEIGHT,
            is_history_valid: u32::from(self.is_history_valid),
        };

        device_context.OMSetRenderTargets(Some(&render_targets), None);
        device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        device_context.VSSetShader(&self.vertex_shader, None);
        device_context.PSSetShader(&self.pixel_shader, None);
        device_context.PSSetShaderResources(0, Some(&shader_resources));
        device_context.PSSetSamplers(0, Some(&samplers));
        device_context.PSSetConstantBuffers(0, Some(&constants_buffers));
        device_context.UpdateSubresource(
            &self.constants_buffer,
            0,
            None,
            (&constants as *const TemporalAntialiasingConstants).cast::<c_void>(),
            0,
            0,
        );
        device_context.Draw(3, 0);
        device_context.OMSetRenderTargets(None, None);
        device_context.CopyResource(
            destination_back_buffer,
            &self.history_targets[write_history_index].texture,
        );
        device_context.PSSetShaderResources(0, Some(&[None, None]));

        self.read_history_index = write_history_index;
        self.is_history_valid = true;
        self.frame_index += 1;
    }

    unsafe fn create_history_target(
        device: &ID3D11Device,
        width: u32,
        height: u32,
    ) -> Result<TemporalHistoryTarget>
    {
        let texture_description = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE).0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut texture = None;
        let mut render_target_view = None;
        let mut shader_resource_view = None;
        device.CreateTexture2D(&texture_description, None, Some(&mut texture))?;
        let texture = required_resource(texture, "Direct3D returned no temporal history texture.")?;
        device.CreateRenderTargetView(&texture, None, Some(&mut render_target_view))?;
        device.CreateShaderResourceView(&texture, None, Some(&mut shader_resource_view))?;

        return Ok(TemporalHistoryTarget {
            texture,
            render_target_view: required_resource(
                render_target_view,
                "Direct3D returned no temporal history render-target view.",
            )?,
            shader_resource_view: required_resource(
                shader_resource_view,
                "Direct3D returned no temporal history shader-resource view.",
            )?,
        });
    }

    unsafe fn create_sampler_state(device: &ID3D11Device) -> Result<ID3D11SamplerState>
    {
        let description = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            MinLOD: 0.0,
            MaxLOD: f32::MAX,
            MaxAnisotropy: 1,
            ..Default::default()
        };
        let mut sampler_state = None;
        device.CreateSamplerState(&description, Some(&mut sampler_state))?;
        return required_resource(sampler_state, "Direct3D returned no temporal antialiasing sampler state.");
    }

    unsafe fn create_constants_buffer(
        device: &ID3D11Device,
        width: u32,
        height: u32,
    ) -> Result<ID3D11Buffer>
    {
        let constants = TemporalAntialiasingConstants {
            render_target_size: [width as f32, height as f32],
            history_weight: TEMPORAL_HISTORY_WEIGHT,
            is_history_valid: 0,
        };
        let description = D3D11_BUFFER_DESC {
            ByteWidth: size_of::<TemporalAntialiasingConstants>() as u32,
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        let initial_data = D3D11_SUBRESOURCE_DATA {
            pSysMem: (&constants as *const TemporalAntialiasingConstants).cast::<c_void>(),
            SysMemPitch: 0,
            SysMemSlicePitch: 0,
        };
        let mut constants_buffer = None;
        device.CreateBuffer(&description, Some(&initial_data), Some(&mut constants_buffer))?;
        return required_resource(constants_buffer, "Direct3D returned no temporal antialiasing constants buffer.");
    }
}

fn temporal_jitter_in_normalized_device_coordinates(
    frame_index: usize,
    width: f32,
    height: f32,
) -> [f32; 2]
{
    let jitter_sample = TEMPORAL_JITTER_PATTERN[frame_index % TEMPORAL_JITTER_PATTERN.len()];

    return [
        (jitter_sample[0] - 0.5) * 2.0 / width,
        (jitter_sample[1] - 0.5) * 2.0 / height,
    ];
}

unsafe fn compile_shader(
    source: &[u8],
    source_name: PCSTR,
    entry_point: PCSTR,
    profile: PCSTR,
) -> Result<Vec<u8>>
{
    let mut bytecode = None;
    D3DCompile(
        source.as_ptr().cast::<c_void>(),
        source.len(),
        source_name,
        None,
        None::<&ID3DInclude>,
        entry_point,
        profile,
        0,
        0,
        &mut bytecode,
        None,
    )?;
    let bytecode = required_resource(bytecode, "The temporal antialiasing HLSL compiler returned no bytecode.")?;
    return Ok(slice::from_raw_parts(
        bytecode.GetBufferPointer().cast(),
        bytecode.GetBufferSize(),
    ).to_vec());
}

unsafe fn create_vertex_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11VertexShader>
{
    let mut vertex_shader = None;
    device.CreateVertexShader(
        bytecode,
        None::<&ID3D11ClassLinkage>,
        Some(&mut vertex_shader),
    )?;
    return required_resource(vertex_shader, "Direct3D returned no temporal antialiasing vertex shader.");
}

unsafe fn create_pixel_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11PixelShader>
{
    let mut pixel_shader = None;
    device.CreatePixelShader(
        bytecode,
        None::<&ID3D11ClassLinkage>,
        Some(&mut pixel_shader),
    )?;
    return required_resource(pixel_shader, "Direct3D returned no temporal antialiasing pixel shader.");
}

fn required_resource<TemporalResource>(
    resource: Option<TemporalResource>,
    message: &'static str,
) -> Result<TemporalResource>
{
    return resource.ok_or_else(|| Error::new(E_FAIL, message));
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn temporal_jitter_changes_between_frames()
    {
        let first_jitter = temporal_jitter_in_normalized_device_coordinates(0, 1280.0, 720.0);
        let second_jitter = temporal_jitter_in_normalized_device_coordinates(1, 1280.0, 720.0);

        assert_ne!(first_jitter, second_jitter);
        assert!(first_jitter[0].abs() <= 1.0 / 1280.0);
        assert!(first_jitter[1].abs() <= 1.0 / 720.0);
    }
}
