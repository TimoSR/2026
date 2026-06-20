#include <cassert>
#include <map>
#include <vector>

using namespace std;

#include "graphics_stack_detection.cpp"

int main() {
    map<string, bool> presence;
    presence["vulkaninfo"] = true;
    presence["cargo"] = true;
    presence["cmake"] = false;
    presence["ninja"] = false;
    presence["windows_sdk"] = true;

    GraphicsStackCapabilities capabilities = detectGraphicsStackFromFlags(presence);
    assert(capabilities.canStartVulkan() == true);
    assert(capabilities.canStartWgpuRust() == true);
    assert(capabilities.canRunCmakeProjects() == false);
    assert(capabilities.canStartDirectX() == true);

    vector<string> order = recommendedLearningOrder(capabilities);
    assert(order.size() == 4);
    assert(order[0] == "vulkan_frame_pattern");
    assert(order[1] == "wgpu_frame_pattern");
    return 0;
}
