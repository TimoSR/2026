#include <string>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#include <mmsystem.h>
#include <d3d11.h>
#pragma comment(lib, "Winmm.lib")
#pragma comment(lib, "D3D11.lib")
#endif

using namespace std;

struct AudioGpuProbeResult {
    unsigned int audioOutputDeviceCount = 0;
    bool canCreateD3d11HardwareDevice = false;
    bool canCreateD3d11WarpDevice = false;
};

bool tryCreateD3d11Device(D3D_DRIVER_TYPE driverType) {
#ifdef _WIN32
    D3D_FEATURE_LEVEL featureLevel = D3D_FEATURE_LEVEL_11_0;
    ID3D11Device* device = nullptr;
    ID3D11DeviceContext* context = nullptr;

    HRESULT result = D3D11CreateDevice(
        nullptr,
        driverType,
        nullptr,
        0,
        nullptr,
        0,
        D3D11_SDK_VERSION,
        &device,
        &featureLevel,
        &context
    );

    if (context != nullptr) {
        context->Release();
    }
    if (device != nullptr) {
        device->Release();
    }

    return SUCCEEDED(result);
#else
    (void)driverType;
    return false;
#endif
}

AudioGpuProbeResult runAudioGpuProbeWin32() {
    AudioGpuProbeResult result;
#ifdef _WIN32
    result.audioOutputDeviceCount = static_cast<unsigned int>(waveOutGetNumDevs());
    result.canCreateD3d11HardwareDevice = tryCreateD3d11Device(D3D_DRIVER_TYPE_HARDWARE);
    result.canCreateD3d11WarpDevice = tryCreateD3d11Device(D3D_DRIVER_TYPE_WARP);
#endif
    return result;
}

bool hasAnyAudioOutputDevice(AudioGpuProbeResult probe) {
    return probe.audioOutputDeviceCount > 0;
}

string chooseGraphicsBackend(AudioGpuProbeResult probe) {
    if (probe.canCreateD3d11HardwareDevice) {
        return "d3d11-hardware";
    }
    if (probe.canCreateD3d11WarpDevice) {
        return "d3d11-warp";
    }
    return "no-d3d11-backend";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    AudioGpuProbeResult probe = runAudioGpuProbeWin32();

    cout << "[audio/gpu probe]\n";
    cout << "audio output devices (waveOut): " << probe.audioOutputDeviceCount << "\n";
    cout << "d3d11 hardware device creation: " << probe.canCreateD3d11HardwareDevice << "\n";
    cout << "d3d11 WARP device creation: " << probe.canCreateD3d11WarpDevice << "\n";
    cout << "has audio output device: " << hasAnyAudioOutputDevice(probe) << "\n";
    cout << "recommended backend: " << chooseGraphicsBackend(probe) << "\n";
    return 0;
}
#endif
