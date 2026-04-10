# Smart Pointers Docs

## Fast Map

```cpp
auto account = make_unique<BankAccount>("Nora", 500.0);   // unique_ptr
auto account = make_shared<BankAccount>("Nora", 500.0);   // shared_ptr
weak_ptr<BankAccount> watcher = account;                    // weak_ptr
```

## `unique_ptr<T>`

Use when there is exactly one owner.

```cpp
auto account = make_unique<BankAccount>("Nora", 500.0);
string ownerName = account->getOwnerName();
```

- Auto cleanup when pointer goes out of scope.
- Can move ownership with `std::move(...)`.
- Cannot copy.

Full example:

```cpp
#include <iostream>
#include <memory>
#include <string>

using namespace std;

class BankAccount {
private:
    string ownerName;
public:
    BankAccount(string ownerNameValue) : ownerName(ownerNameValue) {}
    string getOwnerName() const { return ownerName; }
};

int main() {
    auto account = make_unique<BankAccount>("Nora");
    cout << account->getOwnerName() << "\n";
    return 0;
}
```

## `shared_ptr<T>`

Use when multiple objects must co-own the same instance.

```cpp
auto sharedAccount = make_shared<BankAccount>("Nora", 500.0);
auto secondOwner = sharedAccount;
```

- Reference-counted ownership.
- Object is destroyed when last owner goes away.

Full example:

```cpp
#include <iostream>
#include <memory>
#include <string>

using namespace std;

class BankAccount {
private:
    string ownerName;
public:
    BankAccount(string ownerNameValue) : ownerName(ownerNameValue) {}
    string getOwnerName() const { return ownerName; }
};

int main() {
    auto firstOwner = make_shared<BankAccount>("Nora");
    shared_ptr<BankAccount> secondOwner = firstOwner;

    cout << firstOwner->getOwnerName() << "\n";
    cout << secondOwner->getOwnerName() << "\n";
    return 0;
}
```

## `weak_ptr<T>`

Use to observe a `shared_ptr` object without keeping it alive.

```cpp
weak_ptr<BankAccount> observer = sharedAccount;
shared_ptr<BankAccount> locked = observer.lock();
if (locked) {
    string ownerName = locked->getOwnerName();
}
```

- `lock()` gives temporary access only if object still exists.
- `expired()` tells if object is already gone.

Full example:

```cpp
#include <iostream>
#include <memory>
#include <string>

using namespace std;

class BankAccount {
private:
    string ownerName;
public:
    BankAccount(string ownerNameValue) : ownerName(ownerNameValue) {}
    string getOwnerName() const { return ownerName; }
};

int main() {
    weak_ptr<BankAccount> observer;

    {
        auto sharedAccount = make_shared<BankAccount>("Nora");
        observer = sharedAccount;

        shared_ptr<BankAccount> locked = observer.lock();
        if (locked) {
            cout << locked->getOwnerName() << "\n";
        }
    }

    cout << "Expired after scope: " << observer.expired() << "\n";
    return 0;
}
```

## Practical Rule

- Prefer stack objects first.
- If heap is needed: prefer `make_unique`.
- Use `make_shared` only with real shared ownership.
- Use `weak_ptr` to break cycles and create non-owning observers.
- Avoid raw `new` in new code unless required by a low-level API.

## InDepths

- Reference cycles with `shared_ptr` keep objects alive forever; use `weak_ptr` to break cycles.
- Prefer passing raw references/pointers for temporary access; ownership should stay at composition boundaries.
- Smart pointers solve lifetime, not architecture. Keep ownership graph understandable.

