#include <cassert>
#include <iostream>
#include <string>

using namespace std;

#include "object_creation_styles.cpp"

int main() {
    string stackOwnerName = ownerWithStackObject();
    assert(stackOwnerName == "Nora");

    string rawNewOwnerName = ownerWithRawNewAndDelete();
    assert(rawNewOwnerName == "Nora");

    string uniquePointerOwnerName = ownerWithUniquePointer();
    assert(uniquePointerOwnerName == "Nora");

    string sharedPointerOwnerName = ownerWithSharedPointer();
    assert(sharedPointerOwnerName == "Nora");

    string weakPointerOwnerName = ownerWithWeakPointerLock();
    assert(weakPointerOwnerName == "Nora");

    int liveAccountCount = countLiveAccountsWithWeakPointers();
    assert(liveAccountCount == 0);

    cout << "object_creation_styles tests passed\n";
    return 0;
}
