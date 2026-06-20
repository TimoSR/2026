#ifndef BANK_ACCOUNT_HEADER_ONLY_H
#define BANK_ACCOUNT_HEADER_ONLY_H

#include <string>

using namespace std;

class BankAccountHeaderOnly {
private:
    string ownerName;
    double balance;

public:
    BankAccountHeaderOnly(string ownerNameValue, double startingBalance)
        : ownerName(ownerNameValue), balance(startingBalance) {
    }

    string getOwnerName() const {
        return ownerName;
    }

    double getBalance() const {
        return balance;
    }

    void deposit(double amount) {
        if (amount > 0) {
            balance += amount;
        }
    }
};

#endif
