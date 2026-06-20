#include <cassert>
#include <cmath>
#include <string>

using namespace std;

#include "bank_account_header_only.h"

int main() {
    BankAccountHeaderOnly account("Nora", 500.0);

    string ownerName = account.getOwnerName();
    assert(ownerName == "Nora");

    account.deposit(125.0);
    double balanceAfterDeposit = account.getBalance();
    assert(abs(balanceAfterDeposit - 625.0) < 0.000001);

    return 0;
}
