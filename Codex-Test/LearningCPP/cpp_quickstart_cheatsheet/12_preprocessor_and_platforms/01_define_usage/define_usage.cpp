#include <string>

using namespace std;

#define APP_NAME "CPP Quickstart"
#define BAD_SQUARE(value) value * value
#define SAFE_SQUARE(value) ((value) * (value))

string appNameFromDefine() {
    return APP_NAME;
}

int squareWithBadMacro(int value) {
    return BAD_SQUARE(value);
}

int squareWithSafeMacro(int value) {
    return SAFE_SQUARE(value);
}

int squareWithConstexprFunction(int value) {
    // Prefer this over macros for type safety and debugger-friendly behavior.
    auto square = [](int number) {
        return number * number;
    };
    return square(value);
}

bool isDebugBuild() {
#ifdef NDEBUG
    return false;
#else
    return true;
#endif
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << appNameFromDefine() << "\n";
    cout << "BAD_SQUARE(2 + 3): " << BAD_SQUARE(2 + 3) << "\n";
    cout << "SAFE_SQUARE(2 + 3): " << SAFE_SQUARE(2 + 3) << "\n";
    cout << "constexpr style square: " << squareWithConstexprFunction(5) << "\n";
    cout << "Debug build: " << isDebugBuild() << "\n";
    return 0;
}
#endif
