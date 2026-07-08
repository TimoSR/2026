#include <string>
#include <vector>

using namespace std;

class VulkanFrameState {
private:
    bool fenceWaited = false;
    bool swapchainImageAcquired = false;
    bool commandBufferStarted = false;
    bool renderPassActive = false;
    bool graphicsPipelineBound = false;
    int drawCallCount = 0;
    bool commandBufferEnded = false;
    bool queueSubmitted = false;
    bool imagePresented = false;

public:
    void waitFence() {
        fenceWaited = true;
    }

    bool canAcquireImage() const {
        return fenceWaited;
    }

    void acquireImage() {
        swapchainImageAcquired = true;
    }

    bool canStartCommandBuffer() const {
        return swapchainImageAcquired;
    }

    void startCommandBuffer() {
        commandBufferStarted = true;
    }

    bool canBeginRenderPass() const {
        return commandBufferStarted;
    }

    void beginRenderPass() {
        renderPassActive = true;
    }

    bool canBindPipeline() const {
        return renderPassActive;
    }

    void bindPipeline() {
        graphicsPipelineBound = true;
    }

    bool canDraw() const {
        return renderPassActive && graphicsPipelineBound;
    }

    void recordDrawCall() {
        drawCallCount += 1;
    }

    void endRenderPass() {
        renderPassActive = false;
    }

    bool canEndCommandBuffer() const {
        return commandBufferStarted && !renderPassActive;
    }

    void endCommandBuffer() {
        commandBufferEnded = true;
    }

    bool canSubmitQueue() const {
        return commandBufferEnded;
    }

    void submitQueue() {
        queueSubmitted = true;
    }

    bool canPresentImage() const {
        return queueSubmitted;
    }

    void presentImage() {
        imagePresented = true;
    }

    int getDrawCallCount() const {
        return drawCallCount;
    }

    bool didPresentImage() const {
        return imagePresented;
    }
};

bool executeVulkanFrame(VulkanFrameState& state, vector<string>& steps) {
    state.waitFence();
    steps.push_back("wait_for_in_flight_fence");

    if (!state.canAcquireImage()) {
        return false;
    }
    state.acquireImage();
    steps.push_back("acquire_next_swapchain_image");

    steps.push_back("reset_command_buffer");

    if (!state.canStartCommandBuffer()) {
        return false;
    }
    state.startCommandBuffer();
    steps.push_back("begin_command_buffer");

    if (!state.canBeginRenderPass()) {
        return false;
    }
    state.beginRenderPass();
    steps.push_back("begin_render_pass");

    if (!state.canBindPipeline()) {
        return false;
    }
    state.bindPipeline();
    steps.push_back("bind_graphics_pipeline");

    if (!state.canDraw()) {
        return false;
    }
    state.recordDrawCall();
    steps.push_back("vkCmdDraw");

    state.endRenderPass();
    steps.push_back("end_render_pass");

    if (!state.canEndCommandBuffer()) {
        return false;
    }
    state.endCommandBuffer();
    steps.push_back("end_command_buffer");

    if (!state.canSubmitQueue()) {
        return false;
    }
    state.submitQueue();
    steps.push_back("submit_graphics_queue");

    if (!state.canPresentImage()) {
        return false;
    }
    state.presentImage();
    steps.push_back("present_swapchain_image");
    return true;
}

vector<string> buildVulkanFrameSteps() {
    VulkanFrameState state;
    vector<string> steps;
    bool didSucceed = executeVulkanFrame(state, steps);
    if (!didSucceed) {
        steps.push_back("frame_failed");
    }
    return steps;
}

VulkanFrameState runVulkanFrame() {
    VulkanFrameState state;
    vector<string> steps;
    executeVulkanFrame(state, steps);
    return state;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<string> steps = buildVulkanFrameSteps();
    VulkanFrameState state = runVulkanFrame();
    cout << "[Vulkan Frame Pattern]\n";
    for (string step : steps) {
        cout << step << "\n";
    }
    cout << "\n[Vulkan Frame State]\n";
    cout << "draw calls: " << state.getDrawCallCount() << "\n";
    cout << "presented: " << state.didPresentImage() << "\n";
    return 0;
}
#endif
