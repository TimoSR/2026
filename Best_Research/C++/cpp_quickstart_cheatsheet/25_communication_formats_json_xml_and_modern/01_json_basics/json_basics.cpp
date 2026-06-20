#include <string>

using namespace std;

class UserDto {
private:
    string id = "";
    string name = "";
    int score = 0;

public:
    UserDto(string idValue, string nameValue, int scoreValue) : id(idValue), name(nameValue), score(scoreValue) {
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

string escapeJsonString(string raw) {
    string escaped = "";
    for (char character : raw) {
        if (character == '\\') {
            escaped += "\\\\";
        } else if (character == '"') {
            escaped += "\\\"";
        } else {
            escaped += character;
        }
    }
    return escaped;
}

string toJson(UserDto user) {
    return "{\"id\":\"" + escapeJsonString(user.getId()) +
           "\",\"name\":\"" + escapeJsonString(user.getName()) +
           "\",\"score\":" + to_string(user.getScore()) + "}";
}

bool readStringField(string json, string fieldName, string& value) {
    string marker = "\"" + fieldName + "\":\"";
    size_t start = json.find(marker);
    if (start == string::npos) {
        return false;
    }
    size_t valueStart = start + marker.size();
    size_t valueEnd = json.find("\"", valueStart);
    if (valueEnd == string::npos) {
        return false;
    }
    value = json.substr(valueStart, valueEnd - valueStart);
    return true;
}

bool readIntField(string json, string fieldName, int& value) {
    string marker = "\"" + fieldName + "\":";
    size_t start = json.find(marker);
    if (start == string::npos) {
        return false;
    }
    size_t valueStart = start + marker.size();
    size_t valueEnd = json.find_first_of(",}", valueStart);
    if (valueEnd == string::npos) {
        return false;
    }
    string numberText = json.substr(valueStart, valueEnd - valueStart);
    if (numberText.empty()) {
        return false;
    }
    value = stoi(numberText);
    return true;
}

bool tryFromJson(string json, UserDto& user) {
    string id = "";
    string name = "";
    int score = 0;

    if (!readStringField(json, "id", id)) {
        return false;
    }
    if (!readStringField(json, "name", name)) {
        return false;
    }
    if (!readIntField(json, "score", score)) {
        return false;
    }

    user = UserDto(id, name, score);
    return true;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    UserDto user("u-1", "Nora", 700);
    string json = toJson(user);
    cout << "[toJson]\n" << json << "\n\n";

    UserDto parsed("", "", 0);
    bool ok = tryFromJson(json, parsed);
    cout << "[tryFromJson]\n";
    cout << "ok: " << ok << ", name: " << parsed.getName() << ", score: " << parsed.getScore() << "\n";
    return 0;
}
#endif
