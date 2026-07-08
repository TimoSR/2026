#include <cassert>

#include "audio_and_gpu_api_map.cpp"

int main() {
    AudioGpuProbeResult probe = runAudioGpuProbeWin32();

    assert(probe.audioOutputDeviceCount < 1024);
    assert(probe.canCreateD3d11HardwareDevice || probe.canCreateD3d11WarpDevice);
    assert(hasAnyAudioOutputDevice(probe) == (probe.audioOutputDeviceCount > 0));

    std::string backend = chooseGraphicsBackend(probe);
    if (probe.canCreateD3d11HardwareDevice) {
        assert(backend == "d3d11-hardware");
    } else if (probe.canCreateD3d11WarpDevice) {
        assert(backend == "d3d11-warp");
    } else {
        assert(backend == "no-d3d11-backend");
    }
    return 0;
}
