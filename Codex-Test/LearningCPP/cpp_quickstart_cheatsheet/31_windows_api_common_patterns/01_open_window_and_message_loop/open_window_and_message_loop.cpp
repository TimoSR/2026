#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#pragma comment(lib, "User32.lib")
#endif

using namespace std;

#ifdef _WIN32
LRESULT CALLBACK DemoWindowProcedure(HWND windowHandle, UINT message, WPARAM wParam, LPARAM lParam) {
    (void)wParam;
    (void)lParam;

    switch (message) {
    case WM_DESTROY:
        PostQuitMessage(0);
        return 0;
    default:
        return DefWindowProcW(windowHandle, message, wParam, lParam);
    }
}

bool registerDemoWindowClass(HINSTANCE moduleHandle, const wchar_t* className) {
    WNDCLASSW windowClass = {};
    windowClass.lpfnWndProc = DemoWindowProcedure;
    windowClass.hInstance = moduleHandle;
    windowClass.lpszClassName = className;
    return RegisterClassW(&windowClass) != 0;
}

HWND createDemoWindow(HINSTANCE moduleHandle, const wchar_t* className) {
    return CreateWindowExW(
        0,
        className,
        L"Learning C++ Win32 Demo",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        640,
        360,
        nullptr,
        nullptr,
        moduleHandle,
        nullptr
    );
}

bool runWindowForMilliseconds(int durationMilliseconds) {
    HINSTANCE moduleHandle = GetModuleHandleW(nullptr);
    const wchar_t* className = L"LearningCppWin32DemoClass";

    if (!registerDemoWindowClass(moduleHandle, className)) {
        return false;
    }

    HWND windowHandle = createDemoWindow(moduleHandle, className);

    if (windowHandle == nullptr) {
        return false;
    }

    ShowWindow(windowHandle, SW_SHOW);
    UpdateWindow(windowHandle);

    DWORD startTime = GetTickCount();
    MSG message;
    while (GetTickCount() - startTime < static_cast<DWORD>(durationMilliseconds)) {
        while (PeekMessageW(&message, nullptr, 0, 0, PM_REMOVE)) {
            if (message.message == WM_QUIT) {
                DestroyWindow(windowHandle);
                UnregisterClassW(className, moduleHandle);
                return true;
            }
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        Sleep(10);
    }

    DestroyWindow(windowHandle);
    UnregisterClassW(className, moduleHandle);
    return true;
}
#endif

#ifdef RUN_DEMO
#include <iostream>
int main() {
    cout << "[method] runWindowForMilliseconds\n";
    cout << "[apis] RegisterClassW, CreateWindowExW, PeekMessageW, DispatchMessageW\n";
    bool ok = runWindowForMilliseconds(1500);
    cout << "[result] window demo ok: " << ok << "\n";
    return 0;
}
#endif
