#include <string>

using namespace std;

class SingleFileBankAccount {
private:
    string ownerName;
    double balance;

public:
    SingleFileBankAccount(string ownerNameValue, double startingBalance)
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

#ifdef RUN_DEMO
#include <iostream>

int main() {
    SingleFileBankAccount account("Nora", 500.0);
    account.deposit(150.0);

    cout << account.getOwnerName() << "\n";
    cout << account.getBalance() << "\n";
    return 0;
}
#endif
