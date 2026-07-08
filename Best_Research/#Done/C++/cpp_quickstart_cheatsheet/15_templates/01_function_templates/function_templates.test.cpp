#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "function_templates.cpp"

int main() {
    int maxInt = getMaxValue<int>(10, 30);
    assert(maxInt == 30);

    string maxText = getMaxValue<string>("A", "B");
    assert(maxText == "B");

    vector<int> repeatedInts = repeatValue<int>(7, 3);
    assert(repeatedInts.size() == 3);
    assert(repeatedInts[0] == 7);
    assert(repeatedInts[2] == 7);

    vector<string> repeatedText = repeatValue<string>("Hi", 2);
    assert(repeatedText.size() == 2);
    assert(repeatedText[0] == "Hi");
    assert(repeatedText[1] == "Hi");

    return 0;
}
