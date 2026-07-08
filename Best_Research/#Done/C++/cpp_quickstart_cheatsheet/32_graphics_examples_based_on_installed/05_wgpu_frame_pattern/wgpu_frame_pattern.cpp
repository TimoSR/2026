#include <string>
#include <vector>

using namespace std;

class WgpuFrameState {
private:
    bool surfaceTextureAcquired = false;
    bool commandEncoderCreated = false;
    bool renderPassActive = false;
    bool pipelineSet = false;
    bool vertexBufferSet = false;
    int drawCallCount = 0;
    bool workSubmitted = false;
    bool presented = false;

public:
    void acquireSurfaceTexture() {
        surfaceTextureAcquired = true;
    }

    bool canCreateCommandEncoder() const {
        return surfaceTextureAcquired;
    }

    void createCommandEncoder() {
        commandEncoderCreated = true;
    }

    bool canBeginRenderPass() const {
        return commandEncoderCreated;
    }

    void beginRenderPass() {
        renderPassActive = true;
    }

    bool canSetPipeline() const {
        return renderPassActive;
    }

    void setPipeline() {
        pipelineSet = true;
    }

    bool canSetVertexBuffer() const {
        return pipelineSet;
    }

    void setVertexBuffer() {
        vertexBufferSet = true;
    }

    bool canDraw() const {
        return renderPassActive && pipelineSet && vertexBufferSet;
    }

    void recordDrawCall() {
        drawCallCount += 1;
    }

    bool canEndRenderPass() const {
        return renderPassActive;
    }

    void endRenderPass() {
        renderPassActive = false;
    }

    bool canSubmitQueue() const {
        return commandEncoderCreated && !renderPassActive;
    }

    void submitQueue() {
        workSubmitted = true;
    }

    bool canPresent() const {
        return workSubmitted;
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

bool executeWgpuFrame(WgpuFrameState& state, vector<string>& steps) {
    state.acquireSurfaceTexture();
    steps.push_back("surface_get_current_texture");

    if (!state.canCreateCommandEncoder()) {
        return false;
    }
    state.createCommandEncoder();
    steps.push_back("device_create_command_encoder");

    if (!state.canBeginRenderPass()) {
        return false;
    }
    state.beginRenderPass();
    steps.push_back("begin_render_pass");

    if (!state.canSetPipeline()) {
        return false;
    }
    state.setPipeline();
    steps.push_back("set_pipeline");

    if (!state.canSetVertexBuffer()) {
        return false;
    }
    state.setVertexBuffer();
    steps.push_back("set_vertex_buffer");

    if (!state.canDraw()) {
        return false;
    }
    state.recordDrawCall();
    steps.push_back("draw");

    if (!state.canEndRenderPass()) {
        return false;
    }
    state.endRenderPass();
    steps.push_back("end_render_pass");

    if (!state.canSubmitQueue()) {
        return false;
    }
    state.submitQueue();
    steps.push_back("queue_submit");

    if (!state.canPresent()) {
        return false;
    }
    state.present();
    steps.push_back("surface_present");
    return true;
}

vector<string> buildWgpuFrameSteps() {
    WgpuFrameState state;
    vector<string> steps;
    bool didSucceed = executeWgpuFrame(state, steps);
    if (!didSucceed) {
        steps.push_back("frame_failed");
    }
    return steps;
}

WgpuFrameState runWgpuFrame() {
    WgpuFrameState state;
    vector<string> steps;
    executeWgpuFrame(state, steps);
    return state;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<string> steps = buildWgpuFrameSteps();
    WgpuFrameState state = runWgpuFrame();
    cout << "[wgpu Frame Pattern]\n";
    for (string step : steps) {
        cout << step << "\n";
    }
    cout << "\n[wgpu Frame State]\n";
    cout << "draw calls: " << state.getDrawCallCount() << "\n";
    cout << "presented: " << state.didPresent() << "\n";
    return 0;
}
#endif
