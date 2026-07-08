#include <cassert>
#include <cmath>
#include <string>

using namespace std;

#include "bank_account_header_split.h"
#include "bank_account_header_split.cpp"

int main() {
    BankAccountHeaderSplit account("Nora", 500.0);

    string ownerName = account.getOwnerName();
    assert(ownerName == "Nora");

    double openingBalance = account.getBalance();
    assert(abs(openingBalance - 500.0) < 0.000001);

    account.deposit(200.0);
    double balanceAfterDeposit = account.getBalance();
    assert(abs(balanceAfterDeposit - 700.0) < 0.000001);

    bool didWithdraw = account.withdraw(100.0);
    assert(didWithdraw == true);

    double balanceAfterWithdraw = account.getBalance();
    assert(abs(balanceAfterWithdraw - 600.0) < 0.000001);

    return 0;
}
