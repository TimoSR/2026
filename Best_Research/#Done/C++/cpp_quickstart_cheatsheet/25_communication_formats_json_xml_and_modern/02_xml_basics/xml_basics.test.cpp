#include <cassert>

using namespace std;

#include "xml_basics.cpp"

int main() {
    {
        string xml = buildUserXml("u-1", "Nora", 700);
        assert(xml.find("<user id=\"u-1\">") != string::npos);
        assert(xml.find("<score>700</score>") != string::npos);
    }

    {
        string value = "";
        bool ok = readBetweenTags("<root><name>Nora</name></root>", "name", value);
        assert(ok == true);
        assert(value == "Nora");
    }

    {
        string value = "";
        bool ok = readBetweenTags("<root><name>Nora</name></root>", "score", value);
        assert(ok == false);
    }

    return 0;
}
