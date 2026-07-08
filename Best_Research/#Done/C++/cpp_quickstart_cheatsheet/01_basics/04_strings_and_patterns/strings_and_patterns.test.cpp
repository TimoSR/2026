#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "strings_and_patterns.cpp"

int main() {
    string csv = buildCsvLine("A", "B");
    assert(csv == "A, B");

    string replaced = replaceAll("A, B, A", "A", "X");
    assert(replaced == "X, B, X");

    string rendered = renderTemplate("Hello {name}, welcome to {product}!", "Nora", "CPP Quickstart");
    assert(rendered == "Hello Nora, welcome to CPP Quickstart!");

    vector<string> parts = splitByCommaSpace("A, B, C");
    assert(parts.size() == 3);
    assert(parts[0] == "A");
    assert(parts[1] == "B");
    assert(parts[2] == "C");

    return 0;
}
