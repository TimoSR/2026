#include <cstdint>
#include <functional>
#include <iomanip>
#include <sstream>
#include <string>

using namespace std;

uint64_t fnv1a64(string text) {
    const uint64_t offset = 14695981039346656037ull;
    const uint64_t prime = 1099511628211ull;

    uint64_t hash = offset;
    for (char character : text) {
        hash ^= static_cast<uint8_t>(character);
        hash *= prime;
    }
    return hash;
}

string toHex64(uint64_t value) {
    stringstream output;
    output << hex << uppercase << setw(16) << setfill('0') << value;
    return output.str();
}

uint64_t combineHashes(uint64_t left, uint64_t right) {
    // Practical hash combine pattern used in many codebases.
    return left ^ (right + 0x9E3779B97F4A7C15ull + (left << 6) + (left >> 2));
}

uint64_t stdHashText(string text) {
    hash<string> hasher;
    return static_cast<uint64_t>(hasher(text));
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    string text = "nora@example.com";

    cout << "[fnv1a64]\n";
    uint64_t fnv = fnv1a64(text);
    cout << "value: " << fnv << "\n";
    cout << "hex: " << toHex64(fnv) << "\n\n";

    cout << "[stdHashText]\n";
    uint64_t standard = stdHashText(text);
    cout << "value: " << standard << "\n\n";

    cout << "[combineHashes]\n";
    cout << combineHashes(fnv, standard) << "\n";
    return 0;
}
#endif
