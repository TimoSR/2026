#include <cassert>
#include <vector>

using namespace std;

#include "vulkan_frame_pattern.cpp"

int main() {
    vector<string> steps = buildVulkanFrameSteps();
    assert(steps.size() == 11);
    assert(steps[0] == "wait_for_in_flight_fence");
    assert(steps[1] == "acquire_next_swapchain_image");
    assert(steps[6] == "vkCmdDraw");
    assert(steps[10] == "present_swapchain_image");

    VulkanFrameState state = runVulkanFrame();
    assert(state.getDrawCallCount() == 1);
    assert(state.didPresentImage() == true);
    return 0;
}
