#include <string>
#include <vector>

using namespace std;

class ParsedAnchor {
private:
    string href = "";
    string text = "";
    bool valid = false;

public:
    ParsedAnchor(string hrefValue, string textValue, bool isValid)
        : href(hrefValue), text(textValue), valid(isValid) {
    }

    string getHref() const {
        return href;
    }

    string getText() const {
        return text;
    }

    bool isValid() const {
        return valid;
    }
};

string getAttributeValue(string tagHtml, string attributeName) {
    string marker = attributeName + "=\"";
    size_t markerStart = tagHtml.find(marker);
    if (markerStart == string::npos) {
        return "";
    }

    size_t valueStart = markerStart + marker.size();
    size_t valueEnd = tagHtml.find("\"", valueStart);
    if (valueEnd == string::npos) {
        return "";
    }

    return tagHtml.substr(valueStart, valueEnd - valueStart);
}

ParsedAnchor parseFirstAnchor(string html) {
    size_t openStart = html.find("<a ");
    if (openStart == string::npos) {
        return ParsedAnchor("", "", false);
    }

    size_t openEnd = html.find(">", openStart);
    if (openEnd == string::npos) {
        return ParsedAnchor("", "", false);
    }

    size_t closeStart = html.find("</a>", openEnd);
    if (closeStart == string::npos) {
        return ParsedAnchor("", "", false);
    }

    string openingTag = html.substr(openStart, openEnd - openStart + 1);
    string href = getAttributeValue(openingTag, "href");
    string text = html.substr(openEnd + 1, closeStart - (openEnd + 1));

    if (href.empty()) {
        return ParsedAnchor("", "", false);
    }

    return ParsedAnchor(href, text, true);
}

vector<string> parseListItems(string html) {
    vector<string> items;
    size_t searchStart = 0;

    while (true) {
        size_t openStart = html.find("<li>", searchStart);
        if (openStart == string::npos) {
            break;
        }

        size_t contentStart = openStart + 4;
        size_t closeStart = html.find("</li>", contentStart);
        if (closeStart == string::npos) {
            break;
        }

        items.push_back(html.substr(contentStart, closeStart - contentStart));
        searchStart = closeStart + 5;
    }

    return items;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    string page = "<a href=\"/users/nora\">View Profile</a>";
    ParsedAnchor link = parseFirstAnchor(page);

    cout << "[parseFirstAnchor]\n";
    cout << "Valid: " << link.isValid() << "\n";
    cout << "Href: " << link.getHref() << "\n";
    cout << "Text: " << link.getText() << "\n\n";

    cout << "[parseListItems]\n";
    vector<string> items = parseListItems("<ul><li>C++</li><li>HTML</li></ul>");
    for (string item : items) {
        cout << item << "\n";
    }
    return 0;
}
#endif
