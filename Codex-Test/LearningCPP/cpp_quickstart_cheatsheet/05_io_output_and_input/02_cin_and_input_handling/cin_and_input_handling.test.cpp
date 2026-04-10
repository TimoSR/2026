#include <cassert>
#include <sstream>
#include <string>

using namespace std;

#include "cin_and_input_handling.cpp"

int main() {
    {
        istringstream input("42\n");
        int value = 0;
        bool didRead = tryReadInteger(input, value);
        assert(didRead == true);
        assert(value == 42);
    }

    {
        istringstream input("abc\n");
        int value = 0;
        bool didRead = tryReadInteger(input, value);
        assert(didRead == false);
    }

    {
        istringstream input("77\nHello World\n");
        int number = 0;
        bool didReadNumber = tryReadInteger(input, number);
        assert(didReadNumber == true);
        string line = readLineAfterTokenInput(input);
        assert(line == "Hello World");
    }

    {
        istringstream input("29\n");
        string result = askAgeFlow(input);
        assert(result == "Age accepted");
    }

    {
        istringstream input("-5\n");
        string result = askAgeFlow(input);
        assert(result == "Out of range");
    }

    {
        istringstream input("hello\n");
        string result = askAgeFlow(input);
        assert(result == "Invalid number");
    }

    return 0;
}
