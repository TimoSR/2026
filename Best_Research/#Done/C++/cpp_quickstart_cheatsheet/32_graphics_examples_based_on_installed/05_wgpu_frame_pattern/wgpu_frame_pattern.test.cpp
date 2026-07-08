#include <cassert>
#include <vector>

using namespace std;

#include "wgpu_frame_pattern.cpp"

int main() {
    vector<string> steps = buildWgpuFrameSteps();
    assert(steps.size() == 9);
    assert(steps[0] == "surface_get_current_texture");
    assert(steps[2] == "begin_render_pass");
    assert(steps[7] == "queue_submit");
    assert(steps[8] == "surface_present");

    WgpuFrameState state = runWgpuFrame();
    assert(state.getDrawCallCount() == 1);
    assert(state.didPresent() == true);
    return 0;
}
