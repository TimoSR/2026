#include <cmath>
#include <cstdint>
#include <string>

using namespace std;

double calculateAverage(double firstValue, double secondValue) {
    return (firstValue + secondValue) / 2.0;
}

long long yearlyRequestCount(long long dailyRequests) {
    return dailyRequests * 365LL;
}

bool areEquivalentNumberLiterals() {
    int decimalValue = 42;
    int hexValue = 0x2A;
    int octalValue = 052;
    int binaryValue = 0b101010;

    return decimalValue == hexValue && hexValue == octalValue && octalValue == binaryValue;
}

int parseAsDecimalFromHexLiteral() {
    int value = 0xFF;
    return value;
}

bool closeEnough(double leftValue, double rightValue, double epsilon) {
    return abs(leftValue - rightValue) < epsilon;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    double average = calculateAverage(10.5, 14.5);
    cout << "Average: " << average << "\n";

    long long yearly = yearlyRequestCount(120000LL);
    cout << "Yearly requests: " << yearly << "\n";

    cout << "Literal values equivalent: " << areEquivalentNumberLiterals() << "\n";
    cout << "0xFF in decimal: " << parseAsDecimalFromHexLiteral() << "\n";
    return 0;
}
#endif
