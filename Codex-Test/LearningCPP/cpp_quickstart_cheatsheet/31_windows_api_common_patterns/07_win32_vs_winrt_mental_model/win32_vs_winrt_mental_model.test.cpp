#include <cassert>

#include "win32_vs_winrt_mental_model.cpp"

int main() {
    assert(currentProcessIdViaWin32() > 0);
    assert(winrtRuntimeApisAvailable() == true);
    assert(tryInitializeWinrtApartment() == true);
    assert(canUseWinrtInCurrentProcess() == true);
    assert(chooseWindowsApiLayer(true, false) == "win32");
    assert(chooseWindowsApiLayer(false, true) == "winrt");
    assert(chooseWindowsApiLayer(true, true) == "hybrid");
    return 0;
}
