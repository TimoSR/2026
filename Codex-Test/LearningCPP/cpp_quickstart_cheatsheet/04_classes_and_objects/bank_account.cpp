#include <string>

using namespace std;

class BankAccount {
private:
    string ownerName;
    double balance;

public:
    BankAccount(string ownerNameValue, double startingBalance)
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

    bool withdraw(double amount) {
        if (amount <= 0) {
            return false;
        }
        if (amount > balance) {
            return false;
        }

        balance -= amount;
        return true;
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    auto account = new BankAccount("Sofia", 1000.0);
    account->deposit(250.0);
    bool didWithdraw = account->withdraw(300.0);

    cout << "Owner: " << account->getOwnerName() << "\n";
    cout << "Withdraw success: " << didWithdraw << "\n";
    cout << "Balance: " << account->getBalance() << "\n";

    delete account;
    return 0;
}
#endif
