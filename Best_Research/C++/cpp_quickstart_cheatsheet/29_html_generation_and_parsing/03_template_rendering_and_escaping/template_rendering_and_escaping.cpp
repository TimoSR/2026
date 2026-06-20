#include <map>
#include <string>

using namespace std;

string replaceAll(string sourceText, string findText, string replacementText) {
    if (findText.empty()) {
        return sourceText;
    }

    size_t searchStart = 0;
    while (true) {
        size_t position = sourceText.find(findText, searchStart);
        if (position == string::npos) {
            break;
        }

        sourceText.replace(position, findText.size(), replacementText);
        searchStart = position + replacementText.size();
    }

    return sourceText;
}

string escapeHtmlText(string rawText) {
    string escapedText = "";

    for (char character : rawText) {
        if (character == '&') {
            escapedText += "&amp;";
        } else if (character == '<') {
            escapedText += "&lt;";
        } else if (character == '>') {
            escapedText += "&gt;";
        } else if (character == '"') {
            escapedText += "&quot;";
        } else if (character == '\'') {
            escapedText += "&#39;";
        } else {
            escapedText += character;
        }
    }

    return escapedText;
}

string renderTemplate(string templateText, map<string, string> values) {
    for (pair<string, string> entry : values) {
        string placeholder = "{{" + entry.first + "}}";
        templateText = replaceAll(templateText, placeholder, entry.second);
    }

    return templateText;
}

string buildSafeWelcomePageHtml(string userName, string message) {
    string templateText =
        "<section>"
        "<h1>Welcome {{name}}</h1>"
        "<p>{{message}}</p>"
        "</section>";

    map<string, string> values;
    values["name"] = escapeHtmlText(userName);
    values["message"] = escapeHtmlText(message);

    return renderTemplate(templateText, values);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[replaceAll]\n";
    cout << replaceAll("A, B, C", ", ", " | ") << "\n\n";

    cout << "[renderTemplate]\n";
    map<string, string> values;
    values["name"] = "Nora";
    values["level"] = "Beginner";
    cout << renderTemplate("User: {{name}}, Level: {{level}}", values) << "\n\n";

    cout << "[buildSafeWelcomePageHtml]\n";
    cout << buildSafeWelcomePageHtml("Nora <Admin>", "You have 3 <new> messages.") << "\n";
    return 0;
}
#endif
