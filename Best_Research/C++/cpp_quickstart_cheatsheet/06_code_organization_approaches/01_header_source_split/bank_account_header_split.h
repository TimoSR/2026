#ifndef BANK_ACCOUNT_HEADER_SPLIT_H
#define BANK_ACCOUNT_HEADER_SPLIT_H

#include <string>

using namespace std;

class BankAccountHeaderSplit {
private:
    string ownerName;
    double balance;

public:
    BankAccountHeaderSplit(string ownerNameValue, double startingBalance);

    string getOwnerName() const;
    double getBalance() const;

    void deposit(double amount);
    bool withdraw(double amount);
};

#endif
