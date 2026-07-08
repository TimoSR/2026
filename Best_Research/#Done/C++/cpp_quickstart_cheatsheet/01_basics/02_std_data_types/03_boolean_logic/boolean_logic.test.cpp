#include <cassert>
#include <string>

using namespace std;

#include "boolean_logic.cpp"

int main() {
    assert(isAdult(18) == true);
    assert(isAdult(17) == false);

    assert(canLogin(true, true, 0) == true);
    assert(canLogin(false, true, 0) == false);
    assert(canLogin(true, false, 0) == false);
    assert(canLogin(true, true, 5) == false);

    assert(shouldLockAccount(6, false) == true);
    assert(shouldLockAccount(2, true) == true);
    assert(shouldLockAccount(2, false) == false);

    assert(boolToText(true) == "true");
    assert(boolToText(false) == "false");
    return 0;
}
