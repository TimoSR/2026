#include <cassert>
#include <map>
#include <string>

using namespace std;

#include "template_rendering_and_escaping.cpp"

int main() {
    {
        string line = replaceAll("A, B, C", ", ", " | ");
        assert(line == "A | B | C");
    }

    {
        map<string, string> values;
        values["name"] = "Nora";
        values["score"] = "500";
        string output = renderTemplate("User={{name}}, Score={{score}}", values);
        assert(output == "User=Nora, Score=500");
    }

    {
        string html = buildSafeWelcomePageHtml("Nora <Admin>", "Hello & welcome");
        assert(html.find("Nora &lt;Admin&gt;") != string::npos);
        assert(html.find("Hello &amp; welcome") != string::npos);
    }

    return 0;
}
