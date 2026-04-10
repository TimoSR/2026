#include <cassert>
#include <memory>
#include <string>

using namespace std;

#include "graphics_abstraction_wrapper.cpp"

int main() {
    {
        shared_ptr<IRendererBackend> backend = make_shared<OpenGlBackend>();
        GameRendererFacade renderer(backend);
        string result = renderer.render();
        assert(renderer.backendName() == "opengl");
        assert(result.find("fps=60") != string::npos);
        assert(result.find("vsync=on") != string::npos);
    }

    {
        shared_ptr<IRendererBackend> backend = make_shared<VulkanBackend>();
        GameRendererFacade renderer(backend);
        renderer.useProfile(balancedProfile());
        string result = renderer.render();
        assert(renderer.backendName() == "vulkan");
        assert(result.find("fps=75") != string::npos);
        assert(result.find("post=on") != string::npos);
    }

    {
        RenderProfile highFps = highFpsProfile();
        assert(highFps.getName() == "high_fps");
        assert(highFps.getTargetFramesPerSecond() == 144);
        assert(highFps.getVsync() == false);
    }

    return 0;
}
