#include <string>

using namespace std;

bool isAdult(int age) {
    return age >= 18;
}

bool canLogin(bool isActive, bool isEmailVerified, int failedAttempts) {
    if (!isActive) {
        return false;
    }
    if (!isEmailVerified) {
        return false;
    }
    return failedAttempts < 5;
}

bool shouldLockAccount(int failedAttempts, bool hasSuspiciousIp) {
    return failedAttempts >= 5 || hasSuspiciousIp;
}

string boolToText(bool value) {
    return value ? "true" : "false";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "isAdult(24): " << boolToText(isAdult(24)) << "\n";
    cout << "canLogin: " << boolToText(canLogin(true, true, 2)) << "\n";
    cout << "shouldLockAccount: " << boolToText(shouldLockAccount(6, false)) << "\n";
    return 0;
}
#endif
