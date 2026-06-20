#include <cassert>
#include <string>

using namespace std;

#include "serialization_basics.cpp"

int main() {
    {
        UserProfile user("u-1", "Nora", 500);
        string serialized = serializeUserProfile(user);
        assert(serialized == "id=u-1;name=Nora;score=500");
    }

    {
        UserProfile user("u=2", "Nora; Admin", 750);
        string serialized = serializeUserProfile(user);

        UserProfile parsed("", "", 0);
        bool didParse = tryDeserializeUserProfile(serialized, parsed);
        assert(didParse == true);
        assert(parsed.getId() == "u=2");
        assert(parsed.getName() == "Nora; Admin");
        assert(parsed.getScore() == 750);
    }

    {
        UserProfile parsed("", "", 0);
        bool didParse = tryDeserializeUserProfile("id=u-1;name=Nora;score=abc", parsed);
        assert(didParse == false);
    }

    return 0;
}
