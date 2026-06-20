#include <sstream>
#include <string>
#include <vector>

using namespace std;

class UserProfile {
private:
    string id = "";
    string name = "";
    int score = 0;

public:
    UserProfile(string idValue, string nameValue, int scoreValue) : id(idValue), name(nameValue), score(scoreValue) {
    }

    string getId() const {
        return id;
    }

    string getName() const {
        return name;
    }

    int getScore() const {
        return score;
    }
};

string escapeField(string rawText) {
    string escaped = "";
    for (char character : rawText) {
        if (character == '\\') {
            escaped += "\\\\";
        } else if (character == ';') {
            escaped += "\\;";
        } else if (character == '=') {
            escaped += "\\=";
        } else {
            escaped += character;
        }
    }
    return escaped;
}

string unescapeField(string escapedText) {
    string raw = "";
    bool isEscaping = false;

    for (char character : escapedText) {
        if (isEscaping) {
            raw += character;
            isEscaping = false;
        } else if (character == '\\') {
            isEscaping = true;
        } else {
            raw += character;
        }
    }

    return raw;
}

vector<string> splitByUnescapedSemicolon(string text) {
    vector<string> parts;
    string current = "";
    bool isEscaping = false;

    for (char character : text) {
        if (isEscaping) {
            current += character;
            isEscaping = false;
        } else if (character == '\\') {
            current += character;
            isEscaping = true;
        } else if (character == ';') {
            parts.push_back(current);
            current = "";
        } else {
            current += character;
        }
    }

    parts.push_back(current);
    return parts;
}

size_t findUnescapedEquals(string text) {
    bool isEscaping = false;
    for (size_t index = 0; index < text.size(); index += 1) {
        char character = text[index];
        if (isEscaping) {
            isEscaping = false;
            continue;
        }
        if (character == '\\') {
            isEscaping = true;
            continue;
        }
        if (character == '=') {
            return index;
        }
    }
    return string::npos;
}

string serializeUserProfile(UserProfile user) {
    // Practical wire format for learning:
    // id=<value>;name=<value>;score=<value>
    return "id=" + escapeField(user.getId()) +
           ";name=" + escapeField(user.getName()) +
           ";score=" + to_string(user.getScore());
}

bool tryParseInt(string text, int& value) {
    stringstream input(text);
    int number = 0;
    input >> number;
    if (input.fail() || !input.eof()) {
        return false;
    }
    value = number;
    return true;
}

bool tryDeserializeUserProfile(string text, UserProfile& result) {
    vector<string> fields = splitByUnescapedSemicolon(text);

    string id = "";
    string name = "";
    int score = 0;
    bool hasId = false;
    bool hasName = false;
    bool hasScore = false;

    for (string field : fields) {
        size_t equalsIndex = findUnescapedEquals(field);
        if (equalsIndex == string::npos) {
            continue;
        }

        string key = field.substr(0, equalsIndex);
        string value = unescapeField(field.substr(equalsIndex + 1));

        if (key == "id") {
            id = value;
            hasId = true;
        } else if (key == "name") {
            name = value;
            hasName = true;
        } else if (key == "score") {
            int parsedScore = 0;
            if (!tryParseInt(value, parsedScore)) {
                return false;
            }
            score = parsedScore;
            hasScore = true;
        }
    }

    if (!(hasId && hasName && hasScore)) {
        return false;
    }

    result = UserProfile(id, name, score);
    return true;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    UserProfile user("u-100", "Nora; Admin", 950);
    string serialized = serializeUserProfile(user);

    cout << "[serializeUserProfile]\n";
    cout << serialized << "\n\n";

    cout << "[tryDeserializeUserProfile]\n";
    UserProfile parsed("", "", 0);
    bool didParse = tryDeserializeUserProfile(serialized, parsed);
    cout << "didParse: " << didParse << "\n";
    cout << "id: " << parsed.getId() << "\n";
    cout << "name: " << parsed.getName() << "\n";
    cout << "score: " << parsed.getScore() << "\n";
    return 0;
}
#endif
