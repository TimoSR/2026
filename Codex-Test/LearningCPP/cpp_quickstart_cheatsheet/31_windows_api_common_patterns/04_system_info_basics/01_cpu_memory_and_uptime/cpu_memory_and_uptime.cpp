#include <string>
#include <sysinfoapi.h>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#endif

using namespace std;

unsigned int logicalCpuCountWin32() {
#ifdef _WIN32
    SYSTEM_INFO info;
    GetSystemInfo(&info);
    return static_cast<unsigned int>(info.dwNumberOfProcessors);
#else
    return 0;
#endif
}

unsigned long long totalPhysicalMemoryMbWin32() {
#ifdef _WIN32
    SYSTEM_INFO::MEMORYSTATUSEX status;
    status.dwLength = sizeof(MEMORYSTATUSEX);
    if (!GlobalMemoryStatusEx(&status)) {
        return 0;
    }
    return static_cast<unsigned long long>(status.ullTotalPhys / (1024ull * 1024ull));
#else
    return 0;
#endif
}

unsigned long long uptimeMillisecondsWin32() {
#ifdef _WIN32
    return static_cast<unsigned long long>(GetTickCount64());
#else
    return 0;
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[system] logical CPU count: " << logicalCpuCountWin32() << "\n";
    cout << "[system] total memory MB: " << totalPhysicalMemoryMbWin32() << "\n";
    cout << "[system] uptime ms: " << uptimeMillisecondsWin32() << "\n";
    return 0;
}
#endif
