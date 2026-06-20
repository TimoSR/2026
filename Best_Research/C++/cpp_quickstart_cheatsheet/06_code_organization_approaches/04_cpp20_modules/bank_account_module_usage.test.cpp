#include <cassert>
#include <cmath>
#include <string>

import bank_account_module;

using namespace std;

int main() {
    BankAccountModule account("Nora", 500.0);

    string ownerName = account.getOwnerName();
    assert(ownerName == "Nora");

    double balance = account.getBalance();
    assert(abs(balance - 500.0) < 0.000001);

    return 0;
}
