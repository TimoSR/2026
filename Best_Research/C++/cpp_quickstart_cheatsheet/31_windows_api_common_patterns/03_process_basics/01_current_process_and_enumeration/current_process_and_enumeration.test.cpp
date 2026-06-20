#include <cassert>

#include "current_process_and_enumeration.cpp"

int main() {
    assert(currentProcessIdWin32() > 0);
    assert(runningProcessCountWin32() > 0);
    return 0;
}
