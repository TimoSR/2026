#include <string>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#endif

using namespace std;

unsigned long currentProcessIdViaWin32() {
#ifdef _WIN32
    return static_cast<unsigned long>(GetCurrentProcessId());
#else
    return 0;
#endif
}

bool winrtRuntimeApisAvailable() {
#ifdef _WIN32
    HMODULE combaseModule = LoadLibraryW(L"combase.dll");
    if (combaseModule == nullptr) {
        return false;
    }

    FARPROC roInitialize = GetProcAddress(combaseModule, "RoInitialize");
    FARPROC roUninitialize = GetProcAddress(combaseModule, "RoUninitialize");

    bool available = (roInitialize != nullptr && roUninitialize != nullptr);
    FreeLibrary(combaseModule);
    return available;
#else
    return false;
#endif
}

bool tryInitializeWinrtApartment() {
#ifdef _WIN32
    HMODULE combaseModule = LoadLibraryW(L"combase.dll");
    if (combaseModule == nullptr) {
        return false;
    }

    FARPROC initializeProc = GetProcAddress(combaseModule, "RoInitialize");
    FARPROC uninitializeProc = GetProcAddress(combaseModule, "RoUninitialize");

    if (initializeProc == nullptr || uninitializeProc == nullptr) {
        FreeLibrary(combaseModule);
        return false;
    }

    typedef HRESULT(WINAPI* RoInitializeFunction)(unsigned int);
    typedef void (WINAPI* RoUninitializeFunction)();

    RoInitializeFunction roInitialize = reinterpret_cast<RoInitializeFunction>(initializeProc);
    RoUninitializeFunction roUninitialize = reinterpret_cast<RoUninitializeFunction>(uninitializeProc);

    const unsigned int roInitMultithreaded = 1;
    HRESULT result = roInitialize(roInitMultithreaded);

    bool success = SUCCEEDED(result) || result == static_cast<HRESULT>(0x80010106L); // RPC_E_CHANGED_MODE

    if (SUCCEEDED(result)) {
        roUninitialize();
    }

    FreeLibrary(combaseModule);
    return success;
#else
    return false;
#endif
}

bool canUseWinrtInCurrentProcess() {
    return winrtRuntimeApisAvailable() && tryInitializeWinrtApartment();
}

string chooseWindowsApiLayer(bool needsLowLevelControl, bool needsRuntimeProjection) {
    if (needsLowLevelControl && needsRuntimeProjection) {
        return "hybrid";
    }
    if (needsLowLevelControl) {
        return "win32";
    }
    if (needsRuntimeProjection) {
        return "winrt";
    }
    return "win32";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[win32/winrt real checks]\n";
    cout << "current process id (Win32): " << currentProcessIdViaWin32() << "\n";
    cout << "WinRT runtime APIs available: " << winrtRuntimeApisAvailable() << "\n";
    cout << "RoInitialize check: " << tryInitializeWinrtApartment() << "\n";
    cout << "can use WinRT now: " << canUseWinrtInCurrentProcess() << "\n";
    cout << "choose layer (low-level=true, projection=false): "
         << chooseWindowsApiLayer(true, false) << "\n";
    cout << "choose layer (low-level=true, projection=true): "
         << chooseWindowsApiLayer(true, true) << "\n";
    return 0;
}
#endif
