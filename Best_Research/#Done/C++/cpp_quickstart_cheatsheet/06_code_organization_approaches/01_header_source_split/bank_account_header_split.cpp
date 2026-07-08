#include "bank_account_header_split.h"

BankAccountHeaderSplit::BankAccountHeaderSplit(string ownerNameValue, double startingBalance)
    : ownerName(ownerNameValue), balance(startingBalance) {
}

string BankAccountHeaderSplit::getOwnerName() const {
    return ownerName;
}

double BankAccountHeaderSplit::getBalance() const {
    return balance;
}

void BankAccountHeaderSplit::deposit(double amount) {
    if (amount > 0) {
        balance += amount;
    }
}

bool BankAccountHeaderSplit::withdraw(double amount) {
    if (amount <= 0) {
        return false;
    }
    if (amount > balance) {
        return false;
    }

    balance -= amount;
    return true;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    BankAccountHeaderSplit account("Nora", 500.0);
    account.deposit(100.0);

    cout << account.getOwnerName() << "\n";
    cout << account.getBalance() << "\n";
    return 0;
}
#endif
