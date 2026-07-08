#include <cassert>
#include <memory>
#include <string>

using namespace std;

#include "smart_pointers_examples.cpp"

int main() {
    {
        unique_ptr<BankAccount> account = make_unique<BankAccount>("Nora");
        assert(account->getOwnerName() == "Nora");
    }

    {
        shared_ptr<BankAccount> firstOwner = make_shared<BankAccount>("Ava");
        shared_ptr<BankAccount> secondOwner = firstOwner;
        assert(firstOwner->getOwnerName() == "Ava");
        assert(secondOwner.use_count() == 2);
    }

    {
        weak_ptr<BankAccount> observer;
        {
            shared_ptr<BankAccount> account = make_shared<BankAccount>("Liam");
            observer = account;
            assert(observer.expired() == false);
        }
        assert(observer.expired() == true);
    }

    return 0;
}
