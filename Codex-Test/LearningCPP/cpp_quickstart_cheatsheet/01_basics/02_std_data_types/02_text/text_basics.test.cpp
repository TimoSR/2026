#include <cassert>
#include <vector>

using namespace std;

#include "text_basics.cpp"

int main() {
    assert(buildFullName("Nora", "Jensen") == "Nora Jensen");
    assert(buildEmailAddress("mina", "mail.com") == "mina@mail.com");
    assert(trimSpaces("  abc ") == "abc");

    vector<string> values = splitCommaSeparated("A, B, C");
    assert(values.size() == 3);
    assert(trimSpaces(values[1]) == "B");

    vector<string> parsedValues = splitCsvLine("one,two,three");
    assert(parsedValues.size() == 3);
    assert(parsedValues[0] == "one");
    assert(parsedValues[1] == "two");
    assert(parsedValues[2] == "three");

    assert(joinWithComma({" A", "B ", " C "}) == "A, B, C");
    assert(repeatText("Hi", 3, "-") == "Hi-Hi-Hi");
    assert(renderTemplateAB("A, B", "House", "Number") == "House, Number");
    assert(renderTemplateAB("A, B, C", "House", "Number") == "House, Number, C");
    return 0;
}
