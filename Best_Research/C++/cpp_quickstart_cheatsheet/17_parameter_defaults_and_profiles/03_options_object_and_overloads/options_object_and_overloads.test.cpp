#include <cassert>
#include <string>

using namespace std;

#include "options_object_and_overloads.cpp"

int main() {
    {
        string result = renderReport("weekly_sales");
        assert(result.find("charts=yes") != string::npos);
        assert(result.find("summary=yes") != string::npos);
        assert(result.find("theme=light") != string::npos);
    }

    {
        string result = renderReport("weekly_sales", chartsOffDarkTheme());
        assert(result.find("charts=no") != string::npos);
        assert(result.find("summary=yes") != string::npos);
        assert(result.find("theme=dark") != string::npos);
    }

    return 0;
}
