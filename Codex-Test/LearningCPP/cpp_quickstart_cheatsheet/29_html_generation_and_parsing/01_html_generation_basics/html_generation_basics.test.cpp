#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "html_generation_basics.cpp"

int main() {
    {
        string html = buildUserCardHtml("Nora", 500, true);
        assert(html.find("<h2>Nora</h2>") != string::npos);
        assert(html.find("Score: 500") != string::npos);
        assert(html.find("status-online") != string::npos);
    }

    {
        string html = buildUserCardHtml("Nora <Admin>", 500, false);
        assert(html.find("Nora &lt;Admin&gt;") != string::npos);
        assert(html.find("status-offline") != string::npos);
    }

    {
        vector<string> tags = {"A", "B & C", "<unsafe>"};
        string html = buildUnorderedListHtml(tags);
        assert(html == "<ul><li>A</li><li>B &amp; C</li><li>&lt;unsafe&gt;</li></ul>");
    }

    return 0;
}
