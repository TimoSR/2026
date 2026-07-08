#include <cassert>
#include <vector>

using namespace std;

#include "modern_binary_format_patterns.cpp"

int main() {
    {
        FormatProfile json = jsonProfile();
        FormatProfile protobuf = protobufProfile();
        assert(json.getHumanReadable() == true);
        assert(protobuf.getHumanReadable() == false);
        assert(protobuf.getSchemaRequired() == true);
        assert(protobuf.getTypicalPayloadBytes() < json.getTypicalPayloadBytes());
    }

    {
        vector<FormatProfile> profiles = modernProfiles();
        assert(profiles.size() == 4);
        assert(profiles[0].getName() == "messagepack");
        assert(profiles[2].getName() == "protobuf");
    }

    return 0;
}
