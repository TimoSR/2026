#include <string>
#include <vector>

using namespace std;

class DirectX12FrameState {
private:
    bool fenceWaited = false;
    bool allocatorReset = false;
    bool commandListReset = false;
    bool inRenderTargetState = false;
    bool renderTargetCleared = false;
    bool pipelineStateSet = false;
    int drawCallCount = 0;
    bool inPresentState = false;
    bool commandListClosed = false;
    bool commandListsExecuted = false;
    bool fenceSignaled = false;
    bool presented = false;

public:
    void waitFence() {
        fenceWaited = true;
    }

    bool canResetAllocator() const {
        return fenceWaited;
    }

    void resetAllocator() {
        allocatorReset = true;
    }

    bool canResetCommandList() const {
        return allocatorReset;
    }

    void resetCommandList() {
        commandListReset = true;
    }

    bool canTransitionToRenderTarget() const {
        return commandListReset;
    }

    void transitionToRenderTarget() {
        inRenderTargetState = true;
        inPresentState = false;
    }

    bool canClearRenderTarget() const {
        return inRenderTargetState;
    }

    void clearRenderTarget() {
        renderTargetCleared = true;
    }

    bool canSetPipeline() const {
        return renderTargetCleared;
    }

    void setPipeline() {
        pipelineStateSet = true;
    }

    bool canDraw() const {
        return inRenderTargetState && pipelineStateSet;
    }

    void recordDrawCall() {
        drawCallCount += 1;
    }

    bool canTransitionToPresent() const {
        return inRenderTargetState;
    }

    void transitionToPresent() {
        inRenderTargetState = false;
        inPresentState = true;
    }

    bool canCloseCommandList() const {
        return inPresentState;
    }

    void closeCommandList() {
        commandListClosed = true;
    }

    bool canExecuteCommandLists() const {
        return commandListClosed;
    }

    void executeCommandLists() {
        commandListsExecuted = true;
    }

    bool canSignalFence() const {
        return commandListsExecuted;
    }

    void signalFence() {
        fenceSignaled = true;
    }

    bool canPresent() const {
        return inPresentState && fenceSignaled;
    }

    void present() {
        presented = true;
    }

    int getDrawCallCount() const {
        return drawCallCount;
    }

    bool didPresent() const {
        return presented;
    }
};

bool executeDirectX12Frame(DirectX12FrameState& state, vector<string>& steps) {
    state.waitFence();
    steps.push_back("wait_for_previous_frame_fence");

    if (!state.canResetAllocator()) {
        return false;
    }
    state.resetAllocator();
    steps.push_back("reset_command_allocator");

    if (!state.canResetCommandList()) {
        return false;
    }
    state.resetCommandList();
    steps.push_back("reset_command_list");

    if (!state.canTransitionToRenderTarget()) {
        return false;
    }
    state.transitionToRenderTarget();
    steps.push_back("resource_barrier_present_to_render_target");

    if (!state.canClearRenderTarget()) {
        return false;
    }
    state.clearRenderTarget();
    steps.push_back("clear_render_target_view");

    if (!state.canSetPipeline()) {
        return false;
    }
    state.setPipeline();
    steps.push_back("set_pipeline_state");

    if (!state.canDraw()) {
        return false;
    }
    state.recordDrawCall();
    steps.push_back("draw_instanced");

    if (!state.canTransitionToPresent()) {
        return false;
    }
    state.transitionToPresent();
    steps.push_back("resource_barrier_render_target_to_present");

    if (!state.canCloseCommandList()) {
        return false;
    }
    state.closeCommandList();
    steps.push_back("close_command_list");

    if (!state.canExecuteCommandLists()) {
        return false;
    }
    state.executeCommandLists();
    steps.push_back("execute_command_lists");

    if (!state.canSignalFence()) {
        return false;
    }
    state.signalFence();
    steps.push_back("signal_fence");

    if (!state.canPresent()) {
        return false;
    }
    state.present();
    steps.push_back("present");
    return true;
}

vector<string> buildDirectX12FrameSteps() {
    DirectX12FrameState state;
    vector<string> steps;
    bool didSucceed = executeDirectX12Frame(state, steps);
    if (!didSucceed) {
        steps.push_back("frame_failed");
    }
    return steps;
}

DirectX12FrameState runDirectX12Frame() {
    DirectX12FrameState state;
    vector<string> steps;
    executeDirectX12Frame(state, steps);
    return state;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<string> steps = buildDirectX12FrameSteps();
    DirectX12FrameState state = runDirectX12Frame();
    cout << "[DirectX12 Frame Pattern]\n";
    for (string step : steps) {
        cout << step << "\n";
    }
    cout << "\n[DirectX12 Frame State]\n";
    cout << "draw calls: " << state.getDrawCallCount() << "\n";
    cout << "presented: " << state.didPresent() << "\n";
    return 0;
}
#endif
