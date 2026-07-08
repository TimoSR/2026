import bank_account_module;
#include <iostream>

using namespace std;

int main() {
    BankAccountModule account("Nora", 500.0);
    cout << account.getOwnerName() << "\n";
    cout << account.getBalance() << "\n";
    return 0;
}
