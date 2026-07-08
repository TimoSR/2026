#include <cassert>
#include <cmath>
#include <iostream>
#include <string>

using namespace std;

#include "bank_account.cpp"

int main() {
    // Requested style:
    //BankAccount acount("Nora", 500.0);
    //auto account = new BankAccount("Nora", 500.0);
    auto account = make_unique<BankAccount>("Nora", 500.0);

    string ownerName = account->getOwnerName();
    assert(ownerName == "Nora");

    double openingBalance = account->getBalance();
    assert(abs(openingBalance - 500.0) < 0.000001);

    account->deposit(200.0);
    double balanceAfterDeposit = account->getBalance();
    assert(abs(balanceAfterDeposit - 700.0) < 0.000001);

    bool smallWithdraw = account->withdraw(100.0);
    assert(smallWithdraw == true);
    double balanceAfterSmallWithdraw = account->getBalance();
    assert(abs(balanceAfterSmallWithdraw - 600.0) < 0.000001);

    bool overdraw = account->withdraw(1000.0);
    assert(overdraw == false);
    double balanceAfterFailedWithdraw = account->getBalance();
    assert(abs(balanceAfterFailedWithdraw - 600.0) < 0.000001);

    //delete account;

    cout << "bank_account tests passed\n";
    return 0;
}
