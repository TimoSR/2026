#include <string>
#include <vector>

using namespace std;

template <typename T>
class Box {
private:
    T value;

public:
    Box(T valueParam) : value(valueParam) {
    }

    T getValue() const {
        return value;
    }

    void setValue(T newValue) {
        value = newValue;
    }
};

template <typename T>
class SimpleStack {
private:
    vector<T> items;

public:
    void push(T value) {
        items.push_back(value);
    }

    T pop() {
        T lastItem = items.back();
        items.pop_back();
        return lastItem;
    }

    int size() const {
        return static_cast<int>(items.size());
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    Box<int> scoreBox(100);
    cout << scoreBox.getValue() << "\n";

    Box<string> nameBox("Nora");
    cout << nameBox.getValue() << "\n";

    SimpleStack<int> stack;
    stack.push(10);
    stack.push(20);
    cout << stack.pop() << "\n";
    return 0;
}
#endif
