#include <cassert>
#include <string>

using namespace std;

#include "class_templates.cpp"

int main() {
    Box<int> scoreBox(100);
    assert(scoreBox.getValue() == 100);
    scoreBox.setValue(250);
    assert(scoreBox.getValue() == 250);

    Box<string> nameBox("Nora");
    assert(nameBox.getValue() == "Nora");

    SimpleStack<int> stack;
    stack.push(10);
    stack.push(20);
    assert(stack.size() == 2);
    int popped = stack.pop();
    assert(popped == 20);
    assert(stack.size() == 1);

    return 0;
}
