#include <memory>
#include <string>

using namespace std;

class RenderProfile {
private:
    string name = "";
    int targetFramesPerSecond = 60;
    bool vsync = true;
    bool enablePostProcessing = false;

public:
    RenderProfile(string nameValue, int fpsValue, bool vsyncValue, bool postProcessingValue)
        : name(nameValue), targetFramesPerSecond(fpsValue), vsync(vsyncValue), enablePostProcessing(postProcessingValue) {
    }

    string getName() const {
        return name;
    }

    int getTargetFramesPerSecond() const {
        return targetFramesPerSecond;
    }

    bool getVsync() const {
        return vsync;
    }

    bool getEnablePostProcessing() const {
        return enablePostProcessing;
    }
};

RenderProfile safeProfile() {
    return RenderProfile("safe", 60, true, false);
}

RenderProfile balancedProfile() {
    return RenderProfile("balanced", 75, true, true);
}

RenderProfile highFpsProfile() {
    return RenderProfile("high_fps", 144, false, false);
}

class IRendererBackend {
public:
    virtual ~IRendererBackend() = default;
    virtual string getBackendName() = 0;
    virtual string renderFrame(RenderProfile profile) = 0;
};

class OpenGlBackend : public IRendererBackend {
public:
    string getBackendName() override {
        return "opengl";
    }

    string renderFrame(RenderProfile profile) override {
        return "opengl frame: fps=" + to_string(profile.getTargetFramesPerSecond()) +
               ", vsync=" + (profile.getVsync() ? "on" : "off");
    }
};

class VulkanBackend : public IRendererBackend {
public:
    string getBackendName() override {
        return "vulkan";
    }

    string renderFrame(RenderProfile profile) override {
        return "vulkan frame: fps=" + to_string(profile.getTargetFramesPerSecond()) +
               ", post=" + (profile.getEnablePostProcessing() ? "on" : "off");
    }
};

class GameRendererFacade {
private:
    shared_ptr<IRendererBackend> backend;
    RenderProfile profile = safeProfile();

public:
    GameRendererFacade(shared_ptr<IRendererBackend> backendValue) : backend(backendValue) {
    }

    void useProfile(RenderProfile profileValue) {
        profile = profileValue;
    }

    string render() {
        return backend->renderFrame(profile);
    }

    string backendName() {
        return backend->getBackendName();
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    shared_ptr<IRendererBackend> backend = make_shared<VulkanBackend>();
    GameRendererFacade renderer(backend);

    cout << "[default safe profile]\n";
    cout << renderer.render() << "\n\n";

    renderer.useProfile(highFpsProfile());
    cout << "[high fps profile]\n";
    cout << renderer.render() << "\n";
    return 0;
}
#endif
