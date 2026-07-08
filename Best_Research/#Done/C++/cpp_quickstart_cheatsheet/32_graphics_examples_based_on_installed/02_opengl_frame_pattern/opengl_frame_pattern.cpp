#include <string>
#include <vector>

using namespace std;

class OpenGlFrameState {
private:
    bool viewportSet = false;
    bool colorBufferCleared = false;
    bool depthBufferCleared = false;
    bool shaderProgramBound = false;
    bool vertexArrayBound = false;
    int drawCallCount = 0;
    bool buffersSwapped = false;

public:
    void markViewportSet() {
        viewportSet = true;
    }

    void markColorBufferCleared() {
        colorBufferCleared = true;
    }

    void markDepthBufferCleared() {
        depthBufferCleared = true;
    }

    void markShaderProgramBound() {
        shaderProgramBound = true;
    }

    void markVertexArrayBound() {
        vertexArrayBound = true;
    }

    bool canDraw() const {
        return viewportSet && colorBufferCleared && shaderProgramBound && vertexArrayBound;
    }

    void recordDrawCall() {
        drawCallCount += 1;
    }

    void markBuffersSwapped() {
        buffersSwapped = true;
    }

    bool didClearDepthBuffer() const {
        return depthBufferCleared;
    }

    int getDrawCallCount() const {
        return drawCallCount;
    }

    bool didSwapBuffers() const {
        return buffersSwapped;
    }
};

bool executeOpenGlFrame(bool withDepth, OpenGlFrameState& state, vector<string>& steps) {
    steps.push_back("poll_window_events");

    state.markViewportSet();
    steps.push_back("set_viewport");

    state.markColorBufferCleared();
    steps.push_back("clear_color_buffer");

    if (withDepth) {
        state.markDepthBufferCleared();
        steps.push_back("clear_depth_buffer");
    }

    state.markShaderProgramBound();
    steps.push_back("bind_shader_program");

    state.markVertexArrayBound();
    steps.push_back("bind_vertex_array");

    if (!state.canDraw()) {
        return false;
    }

    state.recordDrawCall();
    steps.push_back("draw_arrays");

    state.markBuffersSwapped();
    steps.push_back("swap_buffers");
    return true;
}

vector<string> buildOpenGlFrameSteps(bool withDepth) {
    OpenGlFrameState state;
    vector<string> steps;
    bool didSucceed = executeOpenGlFrame(withDepth, state, steps);
    if (!didSucceed) {
        steps.push_back("frame_failed");
    }
    return steps;
}

OpenGlFrameState runOpenGlFrame(bool withDepth) {
    OpenGlFrameState state;
    vector<string> steps;
    executeOpenGlFrame(withDepth, state, steps);
    return state;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<string> steps = buildOpenGlFrameSteps(true);
    OpenGlFrameState state = runOpenGlFrame(true);
    cout << "[OpenGL Frame Pattern]\n";
    for (string step : steps) {
        cout << step << "\n";
    }
    cout << "\n[OpenGL Frame State]\n";
    cout << "draw calls: " << state.getDrawCallCount() << "\n";
    cout << "depth cleared: " << state.didClearDepthBuffer() << "\n";
    cout << "buffers swapped: " << state.didSwapBuffers() << "\n";
    return 0;
}
#endif
