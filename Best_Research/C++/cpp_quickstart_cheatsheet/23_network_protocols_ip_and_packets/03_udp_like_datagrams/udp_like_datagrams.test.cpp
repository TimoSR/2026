#include <cassert>
#include <vector>

using namespace std;

#include "udp_like_datagrams.cpp"

int main() {
    {
        vector<uint8_t> datagram = buildUdpLikeDatagram(4000, 8080, "hello");
        UdpLikeDatagram parsed = parseUdpLikeDatagram(datagram);
        assert(parsed.isValid() == true);
        assert(parsed.getSourcePort() == 4000);
        assert(parsed.getDestinationPort() == 8080);
        assert(parsed.getPayload() == "hello");
    }

    {
        vector<uint8_t> datagram = buildUdpLikeDatagram(1000, 2000, "abc");
        datagram[7] = static_cast<uint8_t>(datagram[7] + 1); // corrupt checksum low byte
        UdpLikeDatagram parsed = parseUdpLikeDatagram(datagram);
        assert(parsed.isValid() == false);
    }

    {
        vector<uint8_t> datagram = buildUdpLikeDatagram(1000, 2000, "abc");
        datagram[5] = 0; // corrupt length low byte
        UdpLikeDatagram parsed = parseUdpLikeDatagram(datagram);
        assert(parsed.isValid() == false);
    }

    return 0;
}
