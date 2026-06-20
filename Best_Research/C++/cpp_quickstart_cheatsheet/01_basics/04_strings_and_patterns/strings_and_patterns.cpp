#include <string>
#include <vector>

using namespace std;

string buildCsvLine(string firstValue, string secondValue) {
    return firstValue + ", " + secondValue;
}

string replaceAll(string text, string fromValue, string toValue) {
    if (fromValue.empty()) {
        return text;
    }

    string result = "";
    int index = 0;
    int textLength = static_cast<int>(text.length());
    int fromLength = static_cast<int>(fromValue.length());

    while (index < textLength) {
        bool canMatch = index + fromLength <= textLength;
        if (canMatch && text.substr(index, fromLength) == fromValue) {
            result += toValue;
            index += fromLength;
        } else {
            result += text[index];
            index += 1;
        }
    }

    return result;
}

string renderTemplate(string templateText, string userName, string productName) {
    string withName = replaceAll(templateText, "{name}", userName);
    string withProduct = replaceAll(withName, "{product}", productName);
    return withProduct;
}

vector<string> splitByCommaSpace(string text) {
    vector<string> result;
    string current = "";

    for (char letter : text) {
        if (letter == ',') {
            result.push_back(current);
            current = "";
            continue;
        }

        if (letter == ' ' && current.empty()) {
            continue;
        }

        current += letter;
    }

    result.push_back(current);
    return result;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    string csv = buildCsvLine("A", "B");
    cout << csv << "\n";

    string replaced = replaceAll("A, B, A", "A", "X");
    cout << replaced << "\n";

    string output = renderTemplate("Hello {name}, welcome to {product}!", "Nora", "CPP Quickstart");
    cout << output << "\n";

    vector<string> parts = splitByCommaSpace("A, B, C");
    cout << parts[0] << "|" << parts[1] << "|" << parts[2] << "\n";
    return 0;
}
#endif
