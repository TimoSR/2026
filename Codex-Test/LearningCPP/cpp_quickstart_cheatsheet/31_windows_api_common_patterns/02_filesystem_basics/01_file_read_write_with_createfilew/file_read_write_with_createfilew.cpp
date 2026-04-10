#include <string>

#ifdef _WIN32
#define NOMINMAX
#include <windows.h>
#endif

using namespace std;

wstring utf8ToWide(string text) {
#ifdef _WIN32
    if (text.empty()) {
        return L"";
    }
    int count = MultiByteToWideChar(CP_UTF8, 0, text.c_str(), static_cast<int>(text.size()), nullptr, 0);
    wstring wide(count, L'\0');
    MultiByteToWideChar(CP_UTF8, 0, text.c_str(), static_cast<int>(text.size()), wide.data(), count);
    return wide;
#else
    (void)text;
    return L"";
#endif
}

string wideToUtf8(wstring text) {
#ifdef _WIN32
    if (text.empty()) {
        return "";
    }
    int count = WideCharToMultiByte(CP_UTF8, 0, text.c_str(), static_cast<int>(text.size()), nullptr, 0, nullptr, nullptr);
    string utf8(count, '\0');
    WideCharToMultiByte(CP_UTF8, 0, text.c_str(), static_cast<int>(text.size()), utf8.data(), count, nullptr, nullptr);
    return utf8;
#else
    (void)text;
    return "";
#endif
}

string tempDirectoryPathWin32() {
#ifdef _WIN32
    wchar_t buffer[MAX_PATH];
    DWORD length = GetTempPathW(MAX_PATH, buffer);
    if (length == 0 || length > MAX_PATH) {
        return "";
    }
    return wideToUtf8(wstring(buffer, buffer + length));
#else
    return "";
#endif
}

bool writeTextFileWithCreateFileW(string filePath, string text) {
#ifdef _WIN32
    wstring widePath = utf8ToWide(filePath);
    HANDLE fileHandle = CreateFileW(widePath.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (fileHandle == INVALID_HANDLE_VALUE) {
        return false;
    }

    DWORD bytesWritten = 0;
    BOOL ok = WriteFile(fileHandle, text.data(), static_cast<DWORD>(text.size()), &bytesWritten, nullptr);
    CloseHandle(fileHandle);
    return ok && bytesWritten == text.size();
#else
    (void)filePath;
    (void)text;
    return false;
#endif
}

string readTextFileWithCreateFileW(string filePath) {
#ifdef _WIN32
    wstring widePath = utf8ToWide(filePath);
    HANDLE fileHandle = CreateFileW(widePath.c_str(), GENERIC_READ, FILE_SHARE_READ, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (fileHandle == INVALID_HANDLE_VALUE) {
        return "";
    }

    LARGE_INTEGER size;
    if (!GetFileSizeEx(fileHandle, &size) || size.QuadPart < 0) {
        CloseHandle(fileHandle);
        return "";
    }

    string text(static_cast<size_t>(size.QuadPart), '\0');
    DWORD bytesRead = 0;
    BOOL ok = ReadFile(fileHandle, text.data(), static_cast<DWORD>(text.size()), &bytesRead, nullptr);
    CloseHandle(fileHandle);

    if (!ok) {
        return "";
    }

    text.resize(bytesRead);
    return text;
#else
    (void)filePath;
    return "";
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    string filePath = tempDirectoryPathWin32() + "win32_file_io_demo.txt";
    bool writeOk = writeTextFileWithCreateFileW(filePath, "hello from CreateFileW");

    cout << "[filesystem] write ok: " << writeOk << "\n";
    cout << "[filesystem] read text: " << readTextFileWithCreateFileW(filePath) << "\n";
    return 0;
}
#endif
