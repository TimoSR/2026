use std::{
    ffi::c_void,
    mem::{size_of, size_of_val},
    ptr::null_mut,
    slice,
    time::Instant,
};
use windows::{
    core::{w, Error, Result, PCSTR},
    Win32::{
        Foundation::{E_FAIL, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::{
            Direct3D::{
                Fxc::D3DCompile, ID3DBlob, ID3DInclude, D3D_DRIVER_TYPE_HARDWARE,
                D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_11_0, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            },
            Direct3D11::{
                D3D11CreateDeviceAndSwapChain, ID3D11Buffer, ID3D11ClassLinkage,
                ID3D11DepthStencilView, ID3D11Device, ID3D11DeviceContext, ID3D11InputLayout,
                ID3D11PixelShader, ID3D11RasterizerState, ID3D11RenderTargetView, ID3D11Texture2D,
                ID3D11VertexShader, D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_DEPTH_STENCIL,
                D3D11_BIND_FLAG, D3D11_BIND_INDEX_BUFFER, D3D11_BIND_VERTEX_BUFFER,
                D3D11_BUFFER_DESC, D3D11_CLEAR_DEPTH, D3D11_CULL_NONE, D3D11_FILL_SOLID,
                D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_RASTERIZER_DESC,
                D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC,
                D3D11_USAGE_DEFAULT, D3D11_VIEWPORT,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_FORMAT_R16_UINT,
                    DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC,
                    DXGI_RATIONAL, DXGI_SAMPLE_DESC,
                },
                IDXGIAdapter, IDXGISwapChain, DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC,
                DXGI_SWAP_EFFECT_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
            RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
            MSG, PM_REMOVE, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_QUIT, WNDCLASSW,
            WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const CUBE_INDEX_COUNT: u32 = 36;

const CUBE_SHADER_SOURCE: &[u8] = br#"
cbuffer Transform : register(b0)
{
    row_major float4x4 world_view_projection;
};

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
    output.position = mul(float4(input.position, 1.0f), world_view_projection);
    output.color = input.color;
    return output;
}

float4 pixel_main(PixelInput input) : SV_TARGET
{
    return float4(input.color, 1.0f);
}
"#;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const CUBE_VERTICES: [Vertex; 24] = [
    // Back.
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [0.85, 0.15, 0.15],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        color: [0.85, 0.15, 0.15],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        color: [0.85, 0.15, 0.15],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [0.85, 0.15, 0.15],
    },
    // Front.
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [0.15, 0.75, 0.85],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        color: [0.15, 0.75, 0.85],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        color: [0.15, 0.75, 0.85],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [0.15, 0.75, 0.85],
    },
    // Left.
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [0.20, 0.35, 0.85],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [0.20, 0.35, 0.85],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        color: [0.20, 0.35, 0.85],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        color: [0.20, 0.35, 0.85],
    },
    // Right.
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [0.95, 0.50, 0.15],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        color: [0.95, 0.50, 0.15],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        color: [0.95, 0.50, 0.15],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [0.95, 0.50, 0.15],
    },
    // Top.
    Vertex {
        position: [-1.0, 1.0, -1.0],
        color: [0.35, 0.85, 0.30],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        color: [0.35, 0.85, 0.30],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        color: [0.35, 0.85, 0.30],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        color: [0.35, 0.85, 0.30],
    },
    // Bottom.
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [0.60, 0.20, 0.75],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [0.60, 0.20, 0.75],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [0.60, 0.20, 0.75],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [0.60, 0.20, 0.75],
    },
];

const CUBE_INDICES: [u16; CUBE_INDEX_COUNT as usize] = [
    0, 1, 2, 0, 2, 3, // Back.
    4, 6, 5, 4, 7, 6, // Front.
    8, 10, 9, 8, 11, 10, // Left.
    12, 13, 14, 12, 14, 15, // Right.
    16, 17, 18, 16, 18, 19, // Top.
    20, 21, 22, 20, 22, 23, // Bottom.
];

#[repr(C)]
#[derive(Clone, Copy)]
struct TransformBuffer {
    world_view_projection: [[f32; 4]; 4],
}

struct Dx11State {
    swap_chain: IDXGISwapChain,
    device_context: ID3D11DeviceContext,
    render_target_view: ID3D11RenderTargetView,
    depth_stencil_view: ID3D11DepthStencilView,
    rasterizer_state: ID3D11RasterizerState,
    vertex_buffer: ID3D11Buffer,
    index_buffer: ID3D11Buffer,
    transform_buffer: ID3D11Buffer,
    input_layout: ID3D11InputLayout,
    vertex_shader: ID3D11VertexShader,
    pixel_shader: ID3D11PixelShader,
    started_at: Instant,
}

impl Dx11State {
    unsafe fn new(hwnd: HWND) -> Result<Self> {
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: WIDTH as u32,
                Height: HEIGHT as u32,
                RefreshRate: DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: Default::default(),
                Scaling: Default::default(),
            },
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: hwnd,
            Windowed: true.into(),
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };

        let feature_levels = [D3D_FEATURE_LEVEL_11_0];
        let mut swap_chain: Option<IDXGISwapChain> = None;
        let mut device: Option<ID3D11Device> = None;
        let mut device_context: Option<ID3D11DeviceContext> = None;
        let mut selected_feature_level = D3D_FEATURE_LEVEL::default();

        D3D11CreateDeviceAndSwapChain(
            None::<&IDXGIAdapter>,
            D3D_DRIVER_TYPE_HARDWARE,
            Default::default(),
            Default::default(),
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&swap_chain_desc),
            Some(&mut swap_chain),
            Some(&mut device),
            Some(&mut selected_feature_level),
            Some(&mut device_context),
        )?;

        let swap_chain = required_resource(swap_chain, "Direct3D returned no swap chain.")?;
        let device = required_resource(device, "Direct3D returned no device.")?;
        let device_context =
            required_resource(device_context, "Direct3D returned no device context.")?;

        let back_buffer = swap_chain.GetBuffer::<ID3D11Texture2D>(0)?;
        let render_target_view = create_render_target_view(&device, &back_buffer)?;
        let depth_stencil_view = create_depth_stencil_view(&device)?;
        let rasterizer_state = create_rasterizer_state(&device)?;
        let vertex_buffer = create_buffer(&device, &CUBE_VERTICES, D3D11_BIND_VERTEX_BUFFER)?;
        let index_buffer = create_buffer(&device, &CUBE_INDICES, D3D11_BIND_INDEX_BUFFER)?;
        let transform_buffer = create_buffer(
            &device,
            &[TransformBuffer {
                world_view_projection: identity_matrix(),
            }],
            D3D11_BIND_CONSTANT_BUFFER,
        )?;
        let vertex_shader_bytecode = compile_shader(
            PCSTR(c"vertex_main".as_ptr().cast()),
            PCSTR(c"vs_5_0".as_ptr().cast()),
        )?;
        let pixel_shader_bytecode = compile_shader(
            PCSTR(c"pixel_main".as_ptr().cast()),
            PCSTR(c"ps_5_0".as_ptr().cast()),
        )?;
        let vertex_shader = create_vertex_shader(&device, &vertex_shader_bytecode)?;
        let pixel_shader = create_pixel_shader(&device, &pixel_shader_bytecode)?;
        let input_layout = create_input_layout(&device, &vertex_shader_bytecode)?;

        Ok(Self {
            swap_chain,
            device_context,
            render_target_view,
            depth_stencil_view,
            rasterizer_state,
            vertex_buffer,
            index_buffer,
            transform_buffer,
            input_layout,
            vertex_shader,
            pixel_shader,
            started_at: Instant::now(),
        })
    }

    unsafe fn render(&self) -> Result<()> {
        let clear_color = [0.05, 0.08, 0.12, 1.0];
        let render_targets = [Some(self.render_target_view.clone())];
        let vertex_buffers = [Some(self.vertex_buffer.clone())];
        let transform_buffers = [Some(self.transform_buffer.clone())];
        let vertex_stride = size_of::<Vertex>() as u32;
        let vertex_offset = 0;
        let elapsed_seconds = self.started_at.elapsed().as_secs_f32();
        let transform = TransformBuffer {
            world_view_projection: create_world_view_projection(elapsed_seconds),
        };

        self.device_context
            .OMSetRenderTargets(Some(&render_targets), Some(&self.depth_stencil_view));
        self.device_context
            .ClearRenderTargetView(&self.render_target_view, &clear_color);
        self.device_context.ClearDepthStencilView(
            &self.depth_stencil_view,
            D3D11_CLEAR_DEPTH.0,
            1.0,
            0,
        );
        self.device_context.RSSetState(&self.rasterizer_state);
        self.device_context.RSSetViewports(Some(&[D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: WIDTH as f32,
            Height: HEIGHT as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        }]));
        self.device_context.IASetInputLayout(&self.input_layout);
        self.device_context.IASetVertexBuffers(
            0,
            1,
            Some(vertex_buffers.as_ptr()),
            Some(&vertex_stride),
            Some(&vertex_offset),
        );
        self.device_context
            .IASetIndexBuffer(&self.index_buffer, DXGI_FORMAT_R16_UINT, 0);
        self.device_context
            .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        self.device_context.VSSetShader(&self.vertex_shader, None);
        self.device_context.PSSetShader(&self.pixel_shader, None);
        self.device_context
            .VSSetConstantBuffers(0, Some(&transform_buffers));
        self.device_context.UpdateSubresource(
            &self.transform_buffer,
            0,
            None,
            (&transform as *const TransformBuffer).cast::<c_void>(),
            0,
            0,
        );
        self.device_context.DrawIndexed(CUBE_INDEX_COUNT, 0, 0);

        self.swap_chain.Present(1, DXGI_PRESENT(0)).ok()?;

        Ok(())
    }
}

unsafe fn create_render_target_view(
    device: &ID3D11Device,
    back_buffer: &ID3D11Texture2D,
) -> Result<ID3D11RenderTargetView> {
    let mut render_target_view: Option<ID3D11RenderTargetView> = None;
    device.CreateRenderTargetView(back_buffer, None, Some(&mut render_target_view))?;
    required_resource(
        render_target_view,
        "Direct3D returned no render-target view.",
    )
}

unsafe fn create_depth_stencil_view(device: &ID3D11Device) -> Result<ID3D11DepthStencilView> {
    let depth_buffer_desc = D3D11_TEXTURE2D_DESC {
        Width: WIDTH as u32,
        Height: HEIGHT as u32,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_DEPTH_STENCIL.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };
    let mut depth_buffer: Option<ID3D11Texture2D> = None;
    device.CreateTexture2D(&depth_buffer_desc, None, Some(&mut depth_buffer))?;

    let depth_buffer = required_resource(depth_buffer, "Direct3D returned no depth buffer.")?;
    let mut depth_stencil_view: Option<ID3D11DepthStencilView> = None;
    device.CreateDepthStencilView(&depth_buffer, None, Some(&mut depth_stencil_view))?;

    required_resource(
        depth_stencil_view,
        "Direct3D returned no depth-stencil view.",
    )
}

unsafe fn create_rasterizer_state(device: &ID3D11Device) -> Result<ID3D11RasterizerState> {
    let rasterizer_desc = D3D11_RASTERIZER_DESC {
        FillMode: D3D11_FILL_SOLID,
        CullMode: D3D11_CULL_NONE,
        DepthClipEnable: true.into(),
        ..Default::default()
    };
    let mut rasterizer_state: Option<ID3D11RasterizerState> = None;
    device.CreateRasterizerState(&rasterizer_desc, Some(&mut rasterizer_state))?;

    required_resource(rasterizer_state, "Direct3D returned no rasterizer state.")
}

unsafe fn create_buffer<BufferElement>(
    device: &ID3D11Device,
    elements: &[BufferElement],
    bind_flags: D3D11_BIND_FLAG,
) -> Result<ID3D11Buffer> {
    let buffer_desc = D3D11_BUFFER_DESC {
        ByteWidth: size_of_val(elements) as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: bind_flags.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0,
    };
    let initial_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: elements.as_ptr().cast::<c_void>(),
        SysMemPitch: 0,
        SysMemSlicePitch: 0,
    };
    let mut buffer: Option<ID3D11Buffer> = None;
    device.CreateBuffer(&buffer_desc, Some(&initial_data), Some(&mut buffer))?;

    required_resource(buffer, "Direct3D returned no buffer.")
}

unsafe fn compile_shader(entry_point: PCSTR, profile: PCSTR) -> Result<Vec<u8>> {
    let mut bytecode: Option<ID3DBlob> = None;

    D3DCompile(
        CUBE_SHADER_SOURCE.as_ptr().cast::<c_void>(),
        CUBE_SHADER_SOURCE.len(),
        PCSTR(c"cube.hlsl".as_ptr().cast()),
        None,
        None::<&ID3DInclude>,
        entry_point,
        profile,
        0,
        0,
        &mut bytecode,
        None,
    )?;

    let bytecode = required_resource(bytecode, "The HLSL compiler returned no bytecode.")?;
    Ok(
        slice::from_raw_parts(bytecode.GetBufferPointer().cast(), bytecode.GetBufferSize())
            .to_vec(),
    )
}

unsafe fn create_vertex_shader(
    device: &ID3D11Device,
    bytecode: &[u8],
) -> Result<ID3D11VertexShader> {
    let mut vertex_shader: Option<ID3D11VertexShader> = None;
    device.CreateVertexShader(
        bytecode,
        None::<&ID3D11ClassLinkage>,
        Some(&mut vertex_shader),
    )?;

    required_resource(vertex_shader, "Direct3D returned no vertex shader.")
}

unsafe fn create_pixel_shader(device: &ID3D11Device, bytecode: &[u8]) -> Result<ID3D11PixelShader> {
    let mut pixel_shader: Option<ID3D11PixelShader> = None;
    device.CreatePixelShader(
        bytecode,
        None::<&ID3D11ClassLinkage>,
        Some(&mut pixel_shader),
    )?;

    required_resource(pixel_shader, "Direct3D returned no pixel shader.")
}

unsafe fn create_input_layout(
    device: &ID3D11Device,
    vertex_shader_bytecode: &[u8],
) -> Result<ID3D11InputLayout> {
    let input_elements = [
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(c"POSITION".as_ptr().cast()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 0,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
        D3D11_INPUT_ELEMENT_DESC {
            SemanticName: PCSTR(c"COLOR".as_ptr().cast()),
            SemanticIndex: 0,
            Format: DXGI_FORMAT_R32G32B32_FLOAT,
            InputSlot: 0,
            AlignedByteOffset: 12,
            InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        },
    ];
    let mut input_layout: Option<ID3D11InputLayout> = None;
    device.CreateInputLayout(
        &input_elements,
        vertex_shader_bytecode,
        Some(&mut input_layout),
    )?;

    required_resource(input_layout, "Direct3D returned no input layout.")
}

fn required_resource<Direct3DResource>(
    resource: Option<Direct3DResource>,
    error_message: &str,
) -> Result<Direct3DResource> {
    resource.ok_or_else(|| Error::new(E_FAIL, error_message))
}

fn create_world_view_projection(elapsed_seconds: f32) -> [[f32; 4]; 4] {
    let (sine_x, cosine_x) = (elapsed_seconds * 0.8).sin_cos();
    let (sine_y, cosine_y) = (elapsed_seconds * 1.2).sin_cos();
    let rotation_x = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cosine_x, sine_x, 0.0],
        [0.0, -sine_x, cosine_x, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let rotation_y = [
        [cosine_y, 0.0, -sine_y, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sine_y, 0.0, cosine_y, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let translation = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 5.0, 1.0],
    ];
    let vertical_field_of_view = 60.0_f32.to_radians();
    let focal_length = 1.0 / (vertical_field_of_view * 0.5).tan();
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let near_plane = 0.1;
    let far_plane = 100.0;
    let perspective = [
        [focal_length / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, focal_length, 0.0, 0.0],
        [0.0, 0.0, far_plane / (far_plane - near_plane), 1.0],
        [
            0.0,
            0.0,
            -near_plane * far_plane / (far_plane - near_plane),
            0.0,
        ],
    ];

    multiply_matrices(
        multiply_matrices(multiply_matrices(rotation_x, rotation_y), translation),
        perspective,
    )
}

fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn multiply_matrices(left: [[f32; 4]; 4], right: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];

    for row in 0..4 {
        for column in 0..4 {
            for index in 0..4 {
                result[row][column] += left[row][index] * right[index][column];
            }
        }
    }

    result
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let class_name = w!("RustDx11WindowClass");
        let window_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };

        RegisterClassW(&window_class);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("Rust Direct3D 11 Spinning Cube"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH,
            HEIGHT,
            None,
            None,
            Some(instance.into()),
            Some(null_mut()),
        )?;

        let _ = ShowWindow(hwnd, SW_SHOW);

        let dx = Dx11State::new(hwnd)?;
        let mut message = MSG::default();

        loop {
            while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool() {
                if message.message == WM_QUIT {
                    return Ok(());
                }

                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }

            dx.render()?;
        }
    }
}
