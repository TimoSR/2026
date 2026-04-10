#include <iostream>
#include <memory>
#include <string>

using namespace std;

class BankAccount {
private:
    string ownerName;

public:
    BankAccount(string ownerNameValue) : ownerName(ownerNameValue) {
    }

    string getOwnerName() const {
        return ownerName;
    }
};

void uniquePointerExample() {
    auto account = make_unique<BankAccount>("Nora");
    cout << "unique_ptr owner: " << account->getOwnerName() << "\n";
}

void sharedPointerExample() {
    auto firstOwner = make_shared<BankAccount>("Nora");
    shared_ptr<BankAccount> secondOwner = firstOwner;

    cout << "shared_ptr first owner: " << firstOwner->getOwnerName() << "\n";
    cout << "shared_ptr second owner: " << secondOwner->getOwnerName() << "\n";
}

void weakPointerExample() {
    weak_ptr<BankAccount> observer;

    {
        auto sharedAccount = make_shared<BankAccount>("Nora");
        observer = sharedAccount;

        shared_ptr<BankAccount> locked = observer.lock();
        if (locked) {
            cout << "weak_ptr locked owner: " << locked->getOwnerName() << "\n";
        }
    }

    cout << "weak_ptr expired after scope: " << observer.expired() << "\n";
}

#ifdef RUN_DEMO
int main() {
    uniquePointerExample();
    sharedPointerExample();
    weakPointerExample();
    return 0;
}
#endif
