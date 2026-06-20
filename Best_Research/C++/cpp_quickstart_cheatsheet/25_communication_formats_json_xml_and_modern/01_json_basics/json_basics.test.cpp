#include <cassert>

using namespace std;

#include "json_basics.cpp"

int main() {
    {
        UserDto user("u-1", "Nora", 500);
        string json = toJson(user);
        assert(json.find("\"id\":\"u-1\"") != string::npos);
        assert(json.find("\"score\":500") != string::npos);
    }

    {
        UserDto parsed("", "", 0);
        bool ok = tryFromJson("{\"id\":\"u-2\",\"name\":\"Ava\",\"score\":900}", parsed);
        assert(ok == true);
        assert(parsed.getId() == "u-2");
        assert(parsed.getName() == "Ava");
        assert(parsed.getScore() == 900);
    }

    {
        UserDto parsed("", "", 0);
        bool ok = tryFromJson("{\"id\":\"u-2\",\"name\":\"Ava\"}", parsed);
        assert(ok == false);
    }

    return 0;
}
