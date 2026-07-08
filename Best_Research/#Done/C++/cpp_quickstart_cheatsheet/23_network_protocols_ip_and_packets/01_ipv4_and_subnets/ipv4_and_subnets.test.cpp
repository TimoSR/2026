#include <cassert>

using namespace std;

#include "ipv4_and_subnets.cpp"

int main() {
    {
        ParsedIpv4Address parsed = parseIpv4("10.20.30.40");
        assert(parsed.isValid() == true);
        assert(ipv4ToString(parsed.getValue()) == "10.20.30.40");
    }

    {
        ParsedIpv4Address parsed = parseIpv4("10.20.300.40");
        assert(parsed.isValid() == false);
    }

    {
        ParsedIpv4Address left = parseIpv4("192.168.1.10");
        ParsedIpv4Address right = parseIpv4("192.168.1.77");
        ParsedIpv4Address other = parseIpv4("192.168.2.1");
        assert(areInSameSubnet(left.getValue(), right.getValue(), 24) == true);
        assert(areInSameSubnet(left.getValue(), other.getValue(), 24) == false);
    }

    return 0;
}
