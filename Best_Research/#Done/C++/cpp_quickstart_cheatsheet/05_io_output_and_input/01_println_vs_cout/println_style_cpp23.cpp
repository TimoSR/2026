#include <format>
#include <iostream>
#include <print>
#include <string>

using namespace std;

string formatWithStdFormat(string name, int score) {
    return format("User: {}, Score: {}", name, score);
}

void writeLineWithStdFormat(ostream& output, string label, int value) {
    output << format("{}: {}", label, value) << "\n";
}

string buildLineWithStdFormat(string label, int value) {
    return format("{}: {}", label, value);
}

void printLineWithStdPrint(string label, int value) {
    // std::print does not add newline by default.
    print("[std::print] {}: {}\n", label, value);
}

void printLineWithStdPrintln(string label, int value) {
    // std::println adds newline automatically.
    println("[std::println] {}: {}", label, value);
}

#ifdef RUN_DEMO
int main() {
    cout << "[std::format -> cout] " << formatWithStdFormat("Nora", 95) << "\n";
    writeLineWithStdFormat(cout, "[std::format -> ostream] Coins", 42);

    printLineWithStdPrint("Coins", 42);
    printLineWithStdPrintln("Coins", 42);
    return 0;
}
#endif
