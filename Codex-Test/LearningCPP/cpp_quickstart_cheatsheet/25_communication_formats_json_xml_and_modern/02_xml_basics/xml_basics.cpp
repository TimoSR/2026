#include <string>

using namespace std;

string xmlEscape(string raw) {
    string escaped = "";
    for (char character : raw) {
        if (character == '&') {
            escaped += "&amp;";
        } else if (character == '<') {
            escaped += "&lt;";
        } else if (character == '>') {
            escaped += "&gt;";
        } else {
            escaped += character;
        }
    }
    return escaped;
}

string buildUserXml(string id, string name, int score) {
    return "<user id=\"" + xmlEscape(id) + "\">" +
           "<name>" + xmlEscape(name) + "</name>" +
           "<score>" + to_string(score) + "</score>" +
           "</user>";
}

bool readBetweenTags(string xml, string tagName, string& value) {
    string open = "<" + tagName + ">";
    string close = "</" + tagName + ">";

    size_t start = xml.find(open);
    size_t end = xml.find(close);
    if (start == string::npos || end == string::npos || end < start) {
        return false;
    }

    size_t valueStart = start + open.size();
    value = xml.substr(valueStart, end - valueStart);
    return true;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    string xml = buildUserXml("u-1", "Nora", 800);
    cout << "[buildUserXml]\n" << xml << "\n\n";

    string name = "";
    bool ok = readBetweenTags(xml, "name", name);
    cout << "[readBetweenTags]\n";
    cout << "ok: " << ok << ", name: " << name << "\n";
    return 0;
}
#endif
