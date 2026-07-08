#include <string>
#include <vector>

using namespace std;

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

string buildUserCardHtml(string userName, int score, bool isOnline) {
    string safeName = escapeHtmlText(userName);
    string statusClass = isOnline ? "status-online" : "status-offline";
    string statusText = isOnline ? "Online" : "Offline";

    string html = "";
    html += "<article class=\"user-card\">";
    html += "<h2>" + safeName + "</h2>";
    html += "<p>Score: " + to_string(score) + "</p>";
    html += "<p class=\"" + statusClass + "\">" + statusText + "</p>";
    html += "</article>";
    return html;
}

string buildUnorderedListHtml(vector<string> items) {
    string html = "<ul>";

    for (string item : items) {
        html += "<li>" + escapeHtmlText(item) + "</li>";
    }

    html += "</ul>";
    return html;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[buildUserCardHtml]\n";
    cout << buildUserCardHtml("Nora <Admin>", 950, true) << "\n\n";

    cout << "[buildUnorderedListHtml]\n";
    vector<string> tags = {"C++", "HTML & CSS", "Parsing <101>"};
    cout << buildUnorderedListHtml(tags) << "\n";
    return 0;
}
#endif
