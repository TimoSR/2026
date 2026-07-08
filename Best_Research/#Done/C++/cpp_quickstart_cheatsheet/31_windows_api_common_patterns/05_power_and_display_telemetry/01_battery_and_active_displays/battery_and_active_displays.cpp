#include <string>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#pragma comment(lib, "User32.lib")
#endif

using namespace std;

int batteryLifePercentWin32() {
#ifdef _WIN32
    SYSTEM_POWER_STATUS status;
    if (!GetSystemPowerStatus(&status)) {
        return -1;
    }
    if (status.BatteryLifePercent == 255) {
        return -1;
    }
    return static_cast<int>(status.BatteryLifePercent);
#else
    return -1;
#endif
}

int activeDisplayCountWin32() {
#ifdef _WIN32
    int count = 0;
    DISPLAY_DEVICEW adapter = {};
    adapter.cb = sizeof(DISPLAY_DEVICEW);

    for (DWORD index = 0; EnumDisplayDevicesW(nullptr, index, &adapter, 0); index += 1) {
        if (adapter.StateFlags & DISPLAY_DEVICE_ACTIVE) {
            count += 1;
        }
        adapter.cb = sizeof(DISPLAY_DEVICEW);
    }

    return count;
#else
    return 0;
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[telemetry] battery % (-1 unknown): " << batteryLifePercentWin32() << "\n";
    cout << "[telemetry] active displays: " << activeDisplayCountWin32() << "\n";
    return 0;
}
#endif
