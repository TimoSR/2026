# vulkan_frame_pattern.cpp

## InDepth: Vulkan mental model

Vulkan is explicit:
- you manage synchronization
- you record command buffers
- you submit work explicitly to queues

More setup than OpenGL, but very predictable and scalable.

This example executes the frame flow through `VulkanFrameState` transitions:
- fence wait -> acquire image -> command buffer -> render pass -> draw -> submit -> present
- each step validates prerequisites, so ordering mistakes become visible in code
