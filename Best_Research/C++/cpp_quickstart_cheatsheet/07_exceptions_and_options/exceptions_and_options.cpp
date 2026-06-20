#include <fstream>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

using namespace std;

// Exception style: caller gets a failure reason through exception type/message.
void writeLinesToFileOrThrow(string filePath, vector<string> lines) {
    ofstream outputFile(filePath);
    if (!outputFile.is_open()) {
        throw runtime_error("Could not open file for writing: " + filePath);
    }

    for (string line : lines) {
        outputFile << line << "\n";
    }
}

vector<string> readLinesFromFileOrThrow(string filePath) {
    ifstream inputFile(filePath);
    if (!inputFile.is_open()) {
        throw runtime_error("Could not open file for reading: " + filePath);
    }

    vector<string> lines;
    string currentLine;

    while (getline(inputFile, currentLine)) {
        lines.push_back(currentLine);
    }

    return lines;
}

double divideNumbersOrThrow(double firstNumber, double secondNumber) {
    if (secondNumber == 0.0) {
        throw invalid_argument("Cannot divide by zero");
    }
    return firstNumber / secondNumber;
}

int parsePortOrThrow(string text) {
    if (text.empty()) {
        throw invalid_argument("Port text is empty");
    }

    int value = 0;
    for (char character : text) {
        if (character < '0' || character > '9') {
            throw invalid_argument("Port contains non-digit characters: " + text);
        }

        value = (value * 10) + (character - '0');
        if (value > 65535) {
            throw out_of_range("Port must be between 1 and 65535");
        }
    }

    if (value == 0) {
        throw out_of_range("Port must be between 1 and 65535");
    }

    return value;
}

// Optional style: caller gets success/failure without error details.
optional<vector<string>> tryReadLinesFromFile(string filePath) {
    ifstream inputFile(filePath);
    if (!inputFile.is_open()) {
        return nullopt;
    }

    vector<string> lines;
    string currentLine;
    while (getline(inputFile, currentLine)) {
        lines.push_back(currentLine);
    }

    return lines;
}

optional<double> tryDivideNumbers(double firstNumber, double secondNumber) {
    if (secondNumber == 0.0) {
        return nullopt;
    }
    return firstNumber / secondNumber;
}

optional<int> tryParsePort(string text) {
    if (text.empty()) {
        return nullopt;
    }

    int value = 0;
    for (char character : text) {
        if (character < '0' || character > '9') {
            return nullopt;
        }

        value = (value * 10) + (character - '0');
        if (value > 65535) {
            return nullopt;
        }
    }

    if (value == 0) {
        return nullopt;
    }

    return value;
}

string chooseErrorHandlingStyle(bool failureIsExpected, bool callerNeedsDetailedReason) {
    if (callerNeedsDetailedReason || !failureIsExpected) {
        return "exceptions";
    }
    return "optional";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    writeLinesToFileOrThrow("demo.txt", {"line one", "line two"});
    vector<string> lines = readLinesFromFileOrThrow("demo.txt");
    cout << "[exceptions] loaded lines: " << lines.size() << "\n";

    try {
        cout << "[exceptions] divide 10/2 = " << divideNumbersOrThrow(10, 2) << "\n";
        cout << "[exceptions] divide 10/0 = " << divideNumbersOrThrow(10, 0) << "\n";
    } catch (const exception& caughtException) {
        cout << "[exceptions] error: " << caughtException.what() << "\n";
    }

    optional<double> divisionResult = tryDivideNumbers(10, 0);
    if (divisionResult.has_value()) {
        cout << "[optional] divide 10/0 = " << divisionResult.value() << "\n";
    } else {
        cout << "[optional] divide 10/0 failed\n";
    }

    cout << "[chooser] login parse flow: "
         << chooseErrorHandlingStyle(true, false) << "\n";
    cout << "[chooser] payments flow: "
         << chooseErrorHandlingStyle(false, true) << "\n";

    return 0;
}
#endif
