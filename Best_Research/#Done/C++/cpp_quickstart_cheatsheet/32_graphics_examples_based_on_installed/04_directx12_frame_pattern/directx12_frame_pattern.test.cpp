#include <cassert>
#include <vector>

using namespace std;

#include "directx12_frame_pattern.cpp"

int main() {
    vector<string> steps = buildDirectX12FrameSteps();
    assert(steps.size() == 12);
    assert(steps[0] == "wait_for_previous_frame_fence");
    assert(steps[3] == "resource_barrier_present_to_render_target");
    assert(steps[7] == "resource_barrier_render_target_to_present");
    assert(steps[11] == "present");

    DirectX12FrameState state = runDirectX12Frame();
    assert(state.getDrawCallCount() == 1);
    assert(state.didPresent() == true);
    return 0;
}
