#include <memory>
#include <string>
#include <vector>

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
};

string ownerWithStackObject() {
    // Old and still best in many cases: local stack object.
    BankAccount account("Nora", 500.0);
    return account.getOwnerName();
}

string ownerWithRawNewAndDelete() {
    // Legacy style: manual heap allocation.
    auto account = new BankAccount("Nora", 500.0);
    string ownerName = account->getOwnerName();
    delete account; // Required to avoid memory leak.
    return ownerName;
}

string ownerWithUniquePointer() {
    // Modern heap ownership: automatic cleanup.
    auto account = make_unique<BankAccount>("Nora", 500.0);
    return account->getOwnerName();
}

string ownerWithSharedPointer() {
    // Shared ownership when many objects need the same account.
    shared_ptr<BankAccount> account = make_shared<BankAccount>("Nora", 500.0);
    return account->getOwnerName();
}

string ownerWithWeakPointerLock() {
    // weak_ptr observes a shared object without increasing ownership count.
    shared_ptr<BankAccount> sharedAccount = make_shared<BankAccount>("Nora", 500.0);
    weak_ptr<BankAccount> weakAccount = sharedAccount;

    // lock() gives a temporary shared_ptr if object is still alive.
    shared_ptr<BankAccount> lockedAccount = weakAccount.lock();
    if (lockedAccount) {
        return lockedAccount->getOwnerName();
    }
    return "";
}

int countLiveAccountsWithWeakPointers() {
    vector<weak_ptr<BankAccount>> observers;

    {
        shared_ptr<BankAccount> first = make_shared<BankAccount>("Nora", 500.0);
        shared_ptr<BankAccount> second = make_shared<BankAccount>("Mia", 800.0);
        observers.push_back(first);
        observers.push_back(second);
    } // first and second are destroyed here.

    int liveCount = 0;
    for (weak_ptr<BankAccount> observer : observers) {
        if (!observer.expired()) {
            liveCount += 1;
        }
    }
    return liveCount;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << "Stack object: " << ownerWithStackObject() << "\n";
    cout << "Raw new/delete: " << ownerWithRawNewAndDelete() << "\n";
    cout << "unique_ptr: " << ownerWithUniquePointer() << "\n";
    cout << "shared_ptr: " << ownerWithSharedPointer() << "\n";
    cout << "weak_ptr lock: " << ownerWithWeakPointerLock() << "\n";
    cout << "Live weak observers after scope: " << countLiveAccountsWithWeakPointers() << "\n";
    return 0;
}
#endif
