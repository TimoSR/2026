#include <cassert>
#include <cmath>
#include <string>

using namespace std;

#include "single_file_bank_account.cpp"

int main() {
    SingleFileBankAccount account("Nora", 500.0);

    string ownerName = account.getOwnerName();
    assert(ownerName == "Nora");

    account.deposit(150.0);
    double balanceAfterDeposit = account.getBalance();
    assert(abs(balanceAfterDeposit - 650.0) < 0.000001);

    return 0;
}
