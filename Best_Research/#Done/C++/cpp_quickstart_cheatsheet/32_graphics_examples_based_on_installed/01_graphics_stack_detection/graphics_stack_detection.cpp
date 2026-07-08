#include <cstdlib>
#include <filesystem>
#include <map>
#include <string>
#include <vector>

using namespace std;

class GraphicsStackCapabilities {
private:
    bool hasVulkanInfo = false;
    bool hasCargo = false;
    bool hasCmake = false;
    bool hasNinja = false;
    bool hasWindowsSdk = false;

public:
    GraphicsStackCapabilities(bool vulkanInfo, bool cargo, bool cmake, bool ninja, bool windowsSdk)
        : hasVulkanInfo(vulkanInfo), hasCargo(cargo), hasCmake(cmake), hasNinja(ninja), hasWindowsSdk(windowsSdk) {
    }

    bool canStartVulkan() const {
        return hasVulkanInfo;
    }

    bool canStartWgpuRust() const {
        return hasCargo;
    }

    bool canRunCmakeProjects() const {
        return hasCmake && hasNinja;
    }

    bool canStartDirectX() const {
        return hasWindowsSdk;
    }
};

GraphicsStackCapabilities detectGraphicsStackFromFlags(map<string, bool> commandPresence) {
    bool hasVulkanInfo = commandPresence["vulkaninfo"];
    bool hasCargo = commandPresence["cargo"];
    bool hasCmake = commandPresence["cmake"];
    bool hasNinja = commandPresence["ninja"];
    bool hasWindowsSdk = commandPresence["windows_sdk"];

    return GraphicsStackCapabilities(hasVulkanInfo, hasCargo, hasCmake, hasNinja, hasWindowsSdk);
}

bool commandExistsOnPath(string commandName) {
#ifdef _WIN32
    string command = "where /q " + commandName + " >nul 2>&1";
#else
    string command = "command -v " + commandName + " >/dev/null 2>&1";
#endif
    int exitCode = system(command.c_str());
    return exitCode == 0;
}

string readEnvironmentVariable(string key) {
#ifdef _WIN32
    char* valueBuffer = nullptr;
    size_t valueLength = 0;
    errno_t result = _dupenv_s(&valueBuffer, &valueLength, key.c_str());
    if (result != 0 || valueBuffer == nullptr) {
        return "";
    }

    string value = valueBuffer;
    free(valueBuffer);
    return value;
#else
    const char* value = getenv(key.c_str());
    if (value == nullptr) {
        return "";
    }
    return string(value);
#endif
}

bool detectWindowsSdkInstalled() {
#ifdef _WIN32
    string windowsSdkDir = readEnvironmentVariable("WindowsSdkDir");
    if (windowsSdkDir.empty() == false) {
        return true;
    }

    string windowsSdkVersion = readEnvironmentVariable("WindowsSDKVersion");
    if (windowsSdkVersion.empty() == false) {
        return true;
    }

    string programFilesX86 = readEnvironmentVariable("ProgramFiles(x86)");
    if (programFilesX86.empty() == false) {
        filesystem::path includePath = filesystem::path(programFilesX86) / "Windows Kits" / "10" / "Include";
        if (filesystem::exists(includePath)) {
            return true;
        }
    }
#endif
    return false;
}

map<string, bool> detectGraphicsToolPresenceOnCurrentMachine() {
    map<string, bool> presence;
    presence["vulkaninfo"] = commandExistsOnPath("vulkaninfo");
    presence["cargo"] = commandExistsOnPath("cargo");
    presence["cmake"] = commandExistsOnPath("cmake");
    presence["ninja"] = commandExistsOnPath("ninja");
    presence["windows_sdk"] = detectWindowsSdkInstalled();
    return presence;
}

vector<string> recommendedLearningOrder(GraphicsStackCapabilities capabilities) {
    vector<string> order;

    if (capabilities.canStartVulkan()) {
        order.push_back("vulkan_frame_pattern");
    }
    if (capabilities.canStartWgpuRust()) {
        order.push_back("wgpu_frame_pattern");
    }
    if (capabilities.canStartDirectX()) {
        order.push_back("directx12_frame_pattern");
    }
    order.push_back("opengl_frame_pattern");

    return order;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    map<string, bool> presence = detectGraphicsToolPresenceOnCurrentMachine();

    GraphicsStackCapabilities capabilities = detectGraphicsStackFromFlags(presence);
    vector<string> order = recommendedLearningOrder(capabilities);

    cout << "[Detected Tooling]\n";
    for (pair<string, bool> item : presence) {
        cout << item.first << ": " << (item.second ? "yes" : "no") << "\n";
    }

    cout << "\n";
    cout << "[Recommended Graphics Path]\n";
    for (string item : order) {
        cout << item << "\n";
    }
    return 0;
}
#endif
