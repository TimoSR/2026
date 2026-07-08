#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "generics_concept.cpp"

int main() {
    UserName user("Nora");
    OrderId order(42);

    vector<ITextConvertible*> items = {&user, &order};
    vector<string> textList = toTextList(items);

    assert(textList.size() == 2);
    assert(textList[0] == "Nora");
    assert(textList[1] == "ORD-42");

    return 0;
}
