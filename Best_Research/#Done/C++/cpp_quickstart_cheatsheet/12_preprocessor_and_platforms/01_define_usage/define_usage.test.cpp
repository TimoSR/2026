#include <cassert>
#include <string>

using namespace std;

#include "define_usage.cpp"

int main() {
    string appName = appNameFromDefine();
    assert(appName == "CPP Quickstart");

    int badMacroResult = BAD_SQUARE(2 + 3);
    assert(badMacroResult == 11);

    int safeMacroResult = SAFE_SQUARE(2 + 3);
    assert(safeMacroResult == 25);

    int functionResult = squareWithConstexprFunction(5);
    assert(functionResult == 25);

    bool debugFlag = isDebugBuild();
#ifdef NDEBUG
    assert(debugFlag == false);
#else
    assert(debugFlag == true);
#endif

    return 0;
}
