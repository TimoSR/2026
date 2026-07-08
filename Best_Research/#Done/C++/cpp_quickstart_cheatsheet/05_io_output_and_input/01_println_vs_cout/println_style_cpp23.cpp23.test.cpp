#include <cassert>
#include <sstream>
#include <string>

using namespace std;

#include "println_style_cpp23.cpp"

int main() {
    string formattedText = formatWithStdFormat("Nora", 95);
    assert(formattedText == "User: Nora, Score: 95");

    ostringstream output;
    writeLineWithStdFormat(output, "Coins", 42);
    assert(output.str() == "Coins: 42\n");

    string builtLine = buildLineWithStdFormat("Coins", 42);
    assert(builtLine == "Coins: 42");

    // Compile/runtime usage examples for std::print and std::println.
    printLineWithStdPrint("Coins", 42);
    printLineWithStdPrintln("Coins", 42);

    return 0;
}
