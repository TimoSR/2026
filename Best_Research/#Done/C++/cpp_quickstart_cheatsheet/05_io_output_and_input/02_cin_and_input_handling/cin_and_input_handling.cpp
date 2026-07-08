#include <istream>
#include <limits>
#include <sstream>
#include <string>

using namespace std;

bool tryReadInteger(istream& input, int& parsedValue) {
    input >> parsedValue;

    if (input.fail()) {
        input.clear();
        input.ignore(numeric_limits<streamsize>::max(), '\n');
        return false;
    }

    return true;
}

string readLineAfterTokenInput(istream& input) {
    string line;

    // Remove leftover newline from previous operator>> usage.
    input.ignore(numeric_limits<streamsize>::max(), '\n');
    getline(input, line);
    return line;
}

bool isWithinRange(int value, int minValue, int maxValue) {
    return value >= minValue && value <= maxValue;
}

string askAgeFlow(istream& input) {
    int age = 0;
    bool didParse = tryReadInteger(input, age);
    if (!didParse) {
        return "Invalid number";
    }

    if (!isWithinRange(age, 0, 130)) {
        return "Out of range";
    }

    return "Age accepted";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "Enter your age: ";
    string result = askAgeFlow(cin);
    cout << result << "\n";
    return 0;
}
#endif
