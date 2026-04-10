# wgpu_frame_pattern.cpp

## InDepth: wgpu mental model

wgpu keeps explicit GPU concepts but provides safer, cleaner API layering.

Typical value:
- cross-platform backend abstraction
- modern render-pass and command-encoder model
- easier onboarding than raw Vulkan/DirectX12

This example executes a real frame-state flow:
- acquire surface texture -> encode commands -> render pass -> submit -> present
- validates the command order in `WgpuFrameState` before reporting success
