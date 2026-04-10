#include <cassert>
#include <cmath>
#include <iostream>
#include <string>

using namespace std;

#include "variables_and_output.cpp"

int main() {
    // Arrange + Act
    int firstNumber = 10;
    int secondNumber = 5;
    int totalSum = addTwoNumbers(firstNumber, secondNumber);
    // Assert
    assert(totalSum == 15);

    double circleRadius = 3.0;
    double circleArea = calculateAreaOfCircle(circleRadius);
    assert(abs(circleArea - 28.27433388231) < 0.000001);

    string firstName = "Ada";
    string lastName = "Lovelace";
    int age = 36;
    string userLabel = formatUserLabel(firstName, lastName, age);
    assert(userLabel == "Ada Lovelace (age 36)");

    cout << "variables_and_output tests passed\n";
    return 0;
}
