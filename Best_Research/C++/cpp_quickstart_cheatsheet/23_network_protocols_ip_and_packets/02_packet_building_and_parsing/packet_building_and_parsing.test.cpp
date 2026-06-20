#include <cassert>
#include <vector>

using namespace std;

#include "packet_building_and_parsing.cpp"

int main() {
    {
        vector<uint8_t> packet = buildPacketBytes(1, 7, "HELLO");
        ParsedPacket parsed = parsePacketBytes(packet);
        assert(parsed.isValid() == true);
        assert(parsed.getVersion() == 1);
        assert(parsed.getMessageType() == 7);
        assert(parsed.getPayload() == "HELLO");
    }

    {
        vector<uint8_t> packet = buildPacketBytes(1, 7, "HELLO");
        packet[packet.size() - 1] = 0; // break checksum
        ParsedPacket parsed = parsePacketBytes(packet);
        assert(parsed.isValid() == false);
    }

    {
        vector<uint8_t> packet = buildPacketBytes(1, 3, "X");
        string hex = toHexString(packet);
        assert(hex.size() > 0);
    }

    return 0;
}
