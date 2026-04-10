#include <cstdint>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

class ParsedIpv4Address {
private:
    uint32_t value = 0;
    bool valid = false;

public:
    ParsedIpv4Address(uint32_t valueNumber, bool isValid) : value(valueNumber), valid(isValid) {
    }

    uint32_t getValue() const {
        return value;
    }

    bool isValid() const {
        return valid;
    }
};

ParsedIpv4Address parseIpv4(string text) {
    stringstream input(text);
    vector<int> octets;
    string segment = "";

    while (getline(input, segment, '.')) {
        if (segment.empty()) {
            return ParsedIpv4Address(0, false);
        }

        int number = -1;
        stringstream parser(segment);
        parser >> number;

        if (parser.fail() || !parser.eof() || number < 0 || number > 255) {
            return ParsedIpv4Address(0, false);
        }

        octets.push_back(number);
    }

    if (octets.size() != 4) {
        return ParsedIpv4Address(0, false);
    }

    uint32_t value = 0;
    value |= static_cast<uint32_t>(octets[0]) << 24;
    value |= static_cast<uint32_t>(octets[1]) << 16;
    value |= static_cast<uint32_t>(octets[2]) << 8;
    value |= static_cast<uint32_t>(octets[3]);
    return ParsedIpv4Address(value, true);
}

string ipv4ToString(uint32_t value) {
    int a = static_cast<int>((value >> 24) & 255);
    int b = static_cast<int>((value >> 16) & 255);
    int c = static_cast<int>((value >> 8) & 255);
    int d = static_cast<int>(value & 255);
    return to_string(a) + "." + to_string(b) + "." + to_string(c) + "." + to_string(d);
}

uint32_t subnetMaskFromPrefix(int prefixLength) {
    if (prefixLength < 0 || prefixLength > 32) {
        return 0;
    }

    if (prefixLength == 0) {
        return 0;
    }

    uint32_t allBits = 0xFFFFFFFFu;
    return allBits << (32 - prefixLength);
}

bool areInSameSubnet(uint32_t leftAddress, uint32_t rightAddress, int prefixLength) {
    uint32_t mask = subnetMaskFromPrefix(prefixLength);
    return (leftAddress & mask) == (rightAddress & mask);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    ParsedIpv4Address device = parseIpv4("192.168.1.10");
    ParsedIpv4Address gateway = parseIpv4("192.168.1.1");
    ParsedIpv4Address dns = parseIpv4("8.8.8.8");

    cout << "[parseIpv4]\n";
    cout << "device valid: " << device.isValid() << ", value: " << ipv4ToString(device.getValue()) << "\n";
    cout << "gateway valid: " << gateway.isValid() << ", value: " << ipv4ToString(gateway.getValue()) << "\n\n";

    cout << "[areInSameSubnet /24]\n";
    cout << "device + gateway: " << areInSameSubnet(device.getValue(), gateway.getValue(), 24) << "\n";
    cout << "device + dns: " << areInSameSubnet(device.getValue(), dns.getValue(), 24) << "\n";
    return 0;
}
#endif
