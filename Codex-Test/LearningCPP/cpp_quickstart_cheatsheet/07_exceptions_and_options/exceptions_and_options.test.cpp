#include <cassert>
#include <cmath>
#include <filesystem>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

using namespace std;

#include "exceptions_and_options.cpp"

int main() {
    string testFilePath = "exceptions_and_options_test_output.txt";
    vector<string> linesToWrite = {"alpha", "beta", "gamma"};

    writeLinesToFileOrThrow(testFilePath, linesToWrite);
    vector<string> loadedLines = readLinesFromFileOrThrow(testFilePath);

    int loadedLineCount = static_cast<int>(loadedLines.size());
    string firstLine = loadedLines[0];
    string secondLine = loadedLines[1];
    string thirdLine = loadedLines[2];
    assert(loadedLineCount == 3);
    assert(firstLine == "alpha");
    assert(secondLine == "beta");
    assert(thirdLine == "gamma");

    bool threwReadError = false;
    try {
        readLinesFromFileOrThrow("missing_file_for_exceptions_and_options.txt");
    } catch (const runtime_error&) {
        threwReadError = true;
    }
    assert(threwReadError == true);

    optional<vector<string>> missingLines = tryReadLinesFromFile("missing_file_for_exceptions_and_options.txt");
    assert(missingLines.has_value() == false);

    bool threwDivideError = false;
    try {
        divideNumbersOrThrow(42.0, 0.0);
    } catch (const invalid_argument&) {
        threwDivideError = true;
    }
    assert(threwDivideError == true);

    double divisionResult = divideNumbersOrThrow(9.0, 3.0);
    assert(abs(divisionResult - 3.0) < 0.000001);

    optional<double> optionalDivision = tryDivideNumbers(9.0, 3.0);
    assert(optionalDivision.has_value() == true);
    assert(abs(optionalDivision.value() - 3.0) < 0.000001);

    optional<double> missingDivision = tryDivideNumbers(9.0, 0.0);
    assert(missingDivision.has_value() == false);

    int port = parsePortOrThrow("8080");
    assert(port == 8080);

    bool threwPortError = false;
    try {
        parsePortOrThrow("80a0");
    } catch (const invalid_argument&) {
        threwPortError = true;
    }
    assert(threwPortError == true);

    optional<int> optionalPort = tryParsePort("443");
    assert(optionalPort.has_value() == true);
    assert(optionalPort.value() == 443);

    optional<int> badOptionalPort = tryParsePort("70x0");
    assert(badOptionalPort.has_value() == false);

    assert(chooseErrorHandlingStyle(true, false) == "optional");
    assert(chooseErrorHandlingStyle(false, true) == "exceptions");
    assert(chooseErrorHandlingStyle(false, false) == "exceptions");

    filesystem::remove(testFilePath);

    cout << "exceptions_and_options tests passed\n";
    return 0;
}
