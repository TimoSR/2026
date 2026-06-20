#include <string>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#include <tlhelp32.h>
#endif

using namespace std;

unsigned long currentProcessIdWin32() {
#ifdef _WIN32
    return static_cast<unsigned long>(GetCurrentProcessId());
#else
    return 0;
#endif
}

int runningProcessCountWin32() {
#ifdef _WIN32
    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (snapshot == INVALID_HANDLE_VALUE) {
        return 0;
    }

    PROCESSENTRY32W entry;
    entry.dwSize = sizeof(PROCESSENTRY32W);
    int count = 0;

    if (Process32FirstW(snapshot, &entry)) {
        count += 1;
        while (Process32NextW(snapshot, &entry)) {
            count += 1;
        }
    }

    CloseHandle(snapshot);
    return count;
#else
    return 0;
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[process] current pid: " << currentProcessIdWin32() << "\n";
    cout << "[process] running process count: " << runningProcessCountWin32() << "\n";
    return 0;
}
#endif
