#include <iostream>
#include <string>

using namespace std;

int addTwoNumbers(int firstNumber, int secondNumber) {
    return firstNumber + secondNumber;
}

double calculateAreaOfCircle(double radius) {
    const double pi = 3.14159265359;
    return pi * radius * radius;
}

string formatUserLabel(string firstName, string lastName, int age) {
    // to_string converts a number into text so we can build one message.
    return firstName + " " + lastName + " (age " + to_string(age) + ")";
}

void runVariablesAndOutputDemo() {
    int unreadMessageCount = 3;
    double coffeePrice = 4.5;
    string userName = "Mia";

    cout << "User: " << userName << "\n";
    cout << "Unread: " << unreadMessageCount << "\n";
    cout << "Coffee price: " << coffeePrice << "\n";

    cout << "2 + 5 = " << addTwoNumbers(2, 5) << "\n";
    cout << "Area radius 2 = " << calculateAreaOfCircle(2.0) << "\n";
    cout << formatUserLabel("Mia", "Jensen", 27) << "\n";
}

#ifdef RUN_DEMO
int main() {
    runVariablesAndOutputDemo();
    return 0;
}
#endif
