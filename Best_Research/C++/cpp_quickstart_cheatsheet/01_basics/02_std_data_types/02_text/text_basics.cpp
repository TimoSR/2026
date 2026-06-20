#include <string>
#include <vector>

using namespace std;

string buildFullName(string firstName, string lastName) {
    return firstName + " " + lastName;
}

string buildEmailAddress(string userName, string companyDomain) {
    return userName + "@" + companyDomain;
}

string trimSpaces(string text) {
    size_t start = text.find_first_not_of(" \t");
    if (start == string::npos) {
        return "";
    }
    size_t end = text.find_last_not_of(" \t");
    return text.substr(start, end - start + 1);
}

vector<string> splitCommaSeparated(string text) {
    vector<string> values;
    string current = "";

    for (char character : text) {
        if (character == ',') {
            values.push_back(current);
            current = "";
            continue;
        }
        current.push_back(character);
    }
    values.push_back(current);
    return values;
}

vector<string> splitCsvLine(string csvLine) {
    // Simple CSV split for beginner scenarios (no quoted comma handling).
    vector<string> parts;
    string currentPart = "";

    for (char letter : csvLine) {
        if (letter == ',') {
            parts.push_back(currentPart);
            currentPart = "";
        } else {
            currentPart += letter;
        }
    }

    parts.push_back(currentPart);
    return parts;
}

string joinWithComma(vector<string> values) {
    if (values.empty()) {
        return "";
    }

    string result = trimSpaces(values[0]);
    for (size_t index = 1; index < values.size(); index += 1) {
        result += ", " + trimSpaces(values[index]);
    }
    return result;
}

string renderTemplateAB(string templateText, string valueA, string valueB) {
    vector<string> values = splitCommaSeparated(templateText);
    for (size_t index = 0; index < values.size(); index += 1) {
        string key = trimSpaces(values[index]);
        if (key == "A") {
            values[index] = valueA;
        } else if (key == "B") {
            values[index] = valueB;
        }
    }
    return joinWithComma(values);
}

string repeatText(string text, int repeatCount, string separator) {
    string result = "";

    for (int index = 0; index < repeatCount; index++) {
        result += text;
        if (index < repeatCount - 1) {
            result += separator;
        }
    }

    return result;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << buildFullName("Nora", "Jensen") << "\n";
    cout << buildEmailAddress("nora", "example.com") << "\n";
    cout << repeatText("C++", 3, " | ") << "\n";
    cout << renderTemplateAB("A, B", "House", "Number") << "\n";
    vector<string> colors = splitCsvLine("red,green,blue");
    cout << colors[0] << " - " << colors[1] << " - " << colors[2] << "\n";
    return 0;
}
#endif
