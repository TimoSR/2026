#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "html_parsing_basics.cpp"

int main() {
    {
        ParsedAnchor link = parseFirstAnchor("<a href=\"/users/nora\">Nora</a>");
        assert(link.isValid() == true);
        assert(link.getHref() == "/users/nora");
        assert(link.getText() == "Nora");
    }

    {
        ParsedAnchor link = parseFirstAnchor("<a>Nora</a>");
        assert(link.isValid() == false);
    }

    {
        vector<string> items = parseListItems("<ul><li>A</li><li>B</li><li>C</li></ul>");
        assert(items.size() == 3);
        assert(items[0] == "A");
        assert(items[1] == "B");
        assert(items[2] == "C");
    }

    return 0;
}
