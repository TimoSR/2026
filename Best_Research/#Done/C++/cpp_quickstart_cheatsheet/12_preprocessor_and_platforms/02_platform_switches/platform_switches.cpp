#include <cstdlib>
#include <string>

using namespace std;

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

string currentPlatformName() {
#if defined(_WIN32)
    return "Windows";
#elif defined(__APPLE__)
    return "Apple";
#elif defined(__linux__)
    return "Linux";
#else
    return "Unknown";
#endif
}

string pathSeparator() {
#if defined(_WIN32)
    return "\\";
#else
    return "/";
#endif
}

string lineEndingName() {
#if defined(_WIN32)
    return "CRLF";
#else
    return "LF";
#endif
}

string lineEndingCharacters() {
#if defined(_WIN32)
    return "\r\n";
#else
    return "\n";
#endif
}

string defaultConfigRoot() {
#if defined(_WIN32)
    string appData = readEnvironmentVariable("APPDATA");
    if (appData.empty() == false) {
        return appData;
    }

    string userProfile = readEnvironmentVariable("USERPROFILE");
    if (userProfile.empty() == false) {
        return userProfile + "\\AppData\\Roaming";
    }
    return ".";
#elif defined(__APPLE__)
    string home = readEnvironmentVariable("HOME");
    if (home.empty()) {
        return ".";
    }
    return home + "/Library/Application Support";
#elif defined(__linux__)
    string xdgConfigHome = readEnvironmentVariable("XDG_CONFIG_HOME");
    if (xdgConfigHome.empty() == false) {
        return xdgConfigHome;
    }

    string home = readEnvironmentVariable("HOME");
    if (home.empty()) {
        return ".";
    }
    return home + "/.config";
#else
    return ".";
#endif
}

string defaultAppConfigPath(string appName) {
    return defaultConfigRoot() + pathSeparator() + appName;
}

string normalizeToCurrentPlatformPath(string inputPath) {
    string normalized = inputPath;
    string separator = pathSeparator();
    for (size_t index = 0; index < normalized.size(); index += 1) {
        if (normalized[index] == '/' || normalized[index] == '\\') {
            normalized[index] = separator[0];
        }
    }
    return normalized;
}

string nativeSocketHeaderName() {
#if defined(_WIN32)
    return "winsock2.h";
#else
    return "sys/socket.h";
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << "Platform: " << currentPlatformName() << "\n";
    cout << "Path separator: " << pathSeparator() << "\n";
    cout << "Line ending style: " << lineEndingName() << "\n";
    cout << "Line ending bytes count: " << lineEndingCharacters().size() << "\n";
    cout << "Default config root: " << defaultConfigRoot() << "\n";
    cout << "Sample app config path: " << defaultAppConfigPath("cpp_quickstart") << "\n";
    cout << "Normalized path: " << normalizeToCurrentPlatformPath("assets\\icons/ui/logo.png") << "\n";
    cout << "Native socket header: " << nativeSocketHeaderName() << "\n";
    return 0;
}
#endif
