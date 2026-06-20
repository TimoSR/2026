# opengl_frame_pattern.cpp

## InDepth: OpenGL mental model

OpenGL is state-machine driven:
- set global GPU state
- bind resources
- issue draw call

It is often the fastest path to first triangle for learners.

This example executes a stateful frame pipeline (not just a static step list):
- validates draw preconditions before issuing `draw_arrays`
- tracks depth clear / draw call count / buffer swap in `OpenGlFrameState`
