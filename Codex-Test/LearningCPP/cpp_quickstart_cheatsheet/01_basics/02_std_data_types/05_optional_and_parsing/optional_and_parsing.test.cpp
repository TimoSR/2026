#include <cassert>
#include <optional>

using namespace std;

#include "optional_and_parsing.cpp"

int main() {
    optional<int> parsed = tryParseInt("42");
    assert(parsed.has_value() == true);
    assert(parsed.value() == 42);

    optional<int> parsedInvalid = tryParseInt("42abc");
    assert(parsedInvalid.has_value() == false);

    assert(valueOrDefault(parsed, -1) == 42);
    assert(valueOrDefault(parsedInvalid, -1) == -1);
    return 0;
}
