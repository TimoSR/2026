# graphics_abstraction_wrapper.cpp

## InDepth: What got improved

Raw graphics APIs expose backend-specific setup and frame details.

`GameRendererFacade` finishes abstraction by:
- hiding backend-specific calls
- exposing profile-based behavior (`safe`, `balanced`, `high_fps`)
- letting project code call one method: `render()`

## InDepth: Why this helps projects

You can swap backend (OpenGL/Vulkan/etc.) without rewriting game/app layers, and teams can choose intent-based render behavior instead of tuning many low-level flags everywhere.
