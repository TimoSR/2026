module;
#include <string>

export module bank_account_module;

using namespace std;

export class BankAccountModule {
private:
    string ownerName;
    double balance;

public:
    BankAccountModule(string ownerNameValue, double startingBalance)
        : ownerName(ownerNameValue), balance(startingBalance) {
    }

    string getOwnerName() const {
        return ownerName;
    }

    double getBalance() const {
        return balance;
    }
};
