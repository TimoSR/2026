#include <cassert>
#include <string>

using namespace std;

#include "platform_switches.cpp"

int main() {
    string platformName = currentPlatformName();
    string separator = pathSeparator();
    string lineEnding = lineEndingName();
    string lineEndingChars = lineEndingCharacters();
    string socketHeader = nativeSocketHeaderName();
    string normalized = normalizeToCurrentPlatformPath("a\\b/c");
    string appConfigPath = defaultAppConfigPath("demo_app");

#if defined(_WIN32)
    assert(platformName == "Windows");
    assert(separator == "\\");
    assert(lineEnding == "CRLF");
    assert(lineEndingChars == "\r\n");
    assert(socketHeader == "winsock2.h");
    assert(normalized == "a\\b\\c");
    assert(appConfigPath.find("\\demo_app") != string::npos);
#elif defined(__APPLE__)
    assert(platformName == "Apple");
    assert(separator == "/");
    assert(lineEnding == "LF");
    assert(lineEndingChars == "\n");
    assert(socketHeader == "sys/socket.h");
    assert(normalized == "a/b/c");
    assert(appConfigPath.find("/demo_app") != string::npos);
#elif defined(__linux__)
    assert(platformName == "Linux");
    assert(separator == "/");
    assert(lineEnding == "LF");
    assert(lineEndingChars == "\n");
    assert(socketHeader == "sys/socket.h");
    assert(normalized == "a/b/c");
    assert(appConfigPath.find("/demo_app") != string::npos);
#else
    assert(platformName == "Unknown");
    assert(socketHeader == "sys/socket.h");
#endif

    return 0;
}
