#include <iostream>

#include "bank_account_header_only.h"

using namespace std;

#ifdef RUN_DEMO
int main() {
    BankAccountHeaderOnly account("Nora", 500.0);
    account.deposit(100.0);

    cout << account.getOwnerName() << "\n";
    cout << account.getBalance() << "\n";
    return 0;
}
#endif
