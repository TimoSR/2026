#include <optional>
#include <sstream>
#include <string>

using namespace std;

optional<int> tryParseInt(string text) {
    istringstream input(text);
    int value = 0;
    input >> value;

    if (input.fail()) {
        return nullopt;
    }

    char extra = '\0';
    input >> extra;
    if (!input.fail()) {
        return nullopt;
    }

    return value;
}

int valueOrDefault(optional<int> value, int fallback) {
    return value.value_or(fallback);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    optional<int> parsed42 = tryParseInt("42");
    optional<int> parsedBad = tryParseInt("42abc");

    cout << "42 => " << valueOrDefault(parsed42, -1) << "\n";
    cout << "42abc => " << valueOrDefault(parsedBad, -1) << "\n";
    return 0;
}
#endif
