# directx12_frame_pattern.cpp

## InDepth: DirectX12 mental model

DirectX12 and Vulkan share similar explicit flow:
- command allocator/list management
- explicit resource state barriers
- explicit queue submission and fences

This makes frame behavior transparent and debuggable.

This example now runs a stateful frame execution:
- tracks present/render-target transitions explicitly
- enforces command-list close/execute/signal/present order before success
