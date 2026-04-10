#include <string>
#include <vector>

using namespace std;

class ITextConvertible {
public:
    virtual ~ITextConvertible() = default;
    virtual string toText() const = 0;
};

class UserName : public ITextConvertible {
private:
    string value;

public:
    UserName(string valueParam) : value(valueParam) {
    }

    string toText() const override {
        return value;
    }
};

class OrderId : public ITextConvertible {
private:
    int value;

public:
    OrderId(int valueParam) : value(valueParam) {
    }

    string toText() const override {
        return "ORD-" + to_string(value);
    }
};

vector<string> toTextList(const vector<ITextConvertible*>& items) {
    vector<string> result;
    for (ITextConvertible* item : items) {
        result.push_back(item->toText());
    }
    return result;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    UserName user("Nora");
    OrderId order(42);

    vector<ITextConvertible*> items = {&user, &order};
    vector<string> textList = toTextList(items);

    cout << textList[0] << "\n";
    cout << textList[1] << "\n";
    return 0;
}
#endif
