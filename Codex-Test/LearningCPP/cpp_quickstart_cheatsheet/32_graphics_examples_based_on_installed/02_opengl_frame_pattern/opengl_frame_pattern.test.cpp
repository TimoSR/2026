#include <cassert>
#include <vector>

using namespace std;

#include "opengl_frame_pattern.cpp"

int main() {
    vector<string> steps = buildOpenGlFrameSteps(true);
    assert(steps.size() == 8);
    assert(steps[0] == "poll_window_events");
    assert(steps[6] == "draw_arrays");
    assert(steps[7] == "swap_buffers");
    OpenGlFrameState depthState = runOpenGlFrame(true);
    assert(depthState.getDrawCallCount() == 1);
    assert(depthState.didClearDepthBuffer() == true);
    assert(depthState.didSwapBuffers() == true);

    vector<string> stepsNoDepth = buildOpenGlFrameSteps(false);
    assert(stepsNoDepth.size() == 7);
    OpenGlFrameState noDepthState = runOpenGlFrame(false);
    assert(noDepthState.didClearDepthBuffer() == false);
    assert(noDepthState.getDrawCallCount() == 1);
    assert(noDepthState.didSwapBuffers() == true);
    return 0;
}
