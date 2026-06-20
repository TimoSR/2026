#include <string>
#include <vector>

using namespace std;

[[deprecated("Use createDisplayNameV2 instead")]]
string createDisplayName(string firstName, string lastName) {
    return firstName + " " + lastName;
}

string createDisplayNameV2(string firstName, string lastName) {
    return lastName + ", " + firstName;
}

int sumPositiveValues(vector<int> values) {
    int total = 0;

    for (int value : values) {
        if (value <= 0) {
            continue;
        }

        total += value;
    }

    return total;
}

string categoryFromScore(int score) {
    switch (score / 10) {
        case 10:
        case 9:
            return "A";
        case 8:
            return "B";
        case 7:
            return "C";
        case 6:
            return "D";
        default:
            return "F";
    }
}

void touchForLogging([[maybe_unused]] string tag) {
    // maybe_unused is useful when a parameter is only used in some builds.
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    cout << createDisplayNameV2("Nora", "Jensen") << "\n";
    cout << sumPositiveValues({3, -1, 8, 0, 2}) << "\n";
    cout << categoryFromScore(85) << "\n";
    touchForLogging("startup");
    return 0;
}
#endif
