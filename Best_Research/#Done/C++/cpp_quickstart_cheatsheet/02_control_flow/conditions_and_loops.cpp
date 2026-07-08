#include <algorithm>
#include <string>
#include <vector>

using namespace std;

string categoryFromTemperature(int temperatureCelsius) {
    if (temperatureCelsius < 0) {
        return "freezing";
    }
    if (temperatureCelsius < 20) {
        return "cool";
    }
    if (temperatureCelsius < 30) {
        return "warm";
    }
    return "hot";
}

string fizzBuzzValue(int number) {
    bool isDivisibleByThree = number % 3 == 0;
    bool isDivisibleByFive = number % 5 == 0;

    if (isDivisibleByThree && isDivisibleByFive) {
        return "FizzBuzz";
    }
    if (isDivisibleByThree) {
        return "Fizz";
    }
    if (isDivisibleByFive) {
        return "Buzz";
    }
    return to_string(number);
}

int firstEvenNumberOrMinusOne(vector<int> numbers) {
    for (int number : numbers) {
        if (number % 2 == 0) {
            return number;
        }
    }
    return -1;
}

string gradeBucketFromScore(int score) {
    int normalizedScore = score;
    if (normalizedScore < 0) {
        normalizedScore = 0;
    }
    if (normalizedScore > 100) {
        normalizedScore = 100;
    }

    switch (normalizedScore / 10) {
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

string weekdayNameFromNumber(int weekdayNumber) {
    switch (weekdayNumber) {
    case 1:
        return "Monday";
    case 2:
        return "Tuesday";
    case 3:
        return "Wednesday";
    case 4:
        return "Thursday";
    case 5:
        return "Friday";
    case 6:
        return "Saturday";
    case 7:
        return "Sunday";
    default:
        return "Invalid";
    }
}

int sumUntilLimitWithBreak(vector<int> numbers, int limit) {
    int runningTotal = 0;
    for (int number : numbers) {
        runningTotal += number;
        if (runningTotal >= limit) {
            break; // Stop as soon as target is reached.
        }
    }
    return runningTotal;
}

int sumOnlyPositiveWithContinue(vector<int> numbers) {
    int total = 0;
    for (int number : numbers) {
        if (number <= 0) {
            continue; // Skip non-positive values.
        }
        total += number;
    }
    return total;
}

vector<int> countDownValues(int start) {
    vector<int> values;
    int current = start;
    while (current > 0) {
        values.push_back(current);
        current -= 1;
    }
    return values;
}

int attemptsNeededForPin(string typedPin, string expectedPin, int maxAttempts) {
    int attempts = 0;
    do {
        attempts += 1;
        if (typedPin == expectedPin) {
            return attempts;
        }
    } while (attempts < maxAttempts);

    return attempts;
}

int indexOfNameOrMinusOne(vector<string> names, string targetName) {
    for (size_t index = 0; index < names.size(); index += 1) {
        if (names[index] == targetName) {
            return static_cast<int>(index);
        }
    }
    return -1;
}

bool hasAllowedRole(vector<string> roles, vector<string> allowedRoles) {
    for (string role : roles) {
        if (auto found = find(allowedRoles.begin(), allowedRoles.end(), role); found != allowedRoles.end()) {
            return true; // Modern pattern: if initializer + scoped variable.
        }
    }
    return false;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << categoryFromTemperature(18) << "\n";
    cout << fizzBuzzValue(15) << "\n";
    cout << firstEvenNumberOrMinusOne({1, 3, 7, 10, 11}) << "\n";
    cout << gradeBucketFromScore(82) << "\n";
    cout << weekdayNameFromNumber(5) << "\n";
    cout << sumUntilLimitWithBreak({4, 3, 5, 9}, 10) << "\n";
    cout << sumOnlyPositiveWithContinue({-2, 3, 0, 9, -1}) << "\n";
    cout << indexOfNameOrMinusOne({"Nora", "Ava", "Liam"}, "Ava") << "\n";
    cout << hasAllowedRole({"viewer", "editor"}, {"admin", "editor"}) << "\n";
    return 0;
}
#endif
