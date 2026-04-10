#include <cstdint>
#include <string>
#include <vector>

using namespace std;

class UdpLikeDatagram {
private:
    uint16_t sourcePort = 0;
    uint16_t destinationPort = 0;
    string payload = "";
    bool valid = false;

public:
    UdpLikeDatagram(uint16_t sourcePortNumber, uint16_t destinationPortNumber, string payloadText, bool isValid)
        : sourcePort(sourcePortNumber), destinationPort(destinationPortNumber), payload(payloadText), valid(isValid) {
    }

    uint16_t getSourcePort() const {
        return sourcePort;
    }

    uint16_t getDestinationPort() const {
        return destinationPort;
    }

    string getPayload() const {
        return payload;
    }

    bool isValid() const {
        return valid;
    }
};

void appendUInt16BigEndian(vector<uint8_t>& bytes, uint16_t value) {
    bytes.push_back(static_cast<uint8_t>((value >> 8) & 255));
    bytes.push_back(static_cast<uint8_t>(value & 255));
}

uint16_t readUInt16BigEndian(vector<uint8_t> bytes, size_t startIndex) {
    uint16_t value = static_cast<uint16_t>(bytes[startIndex]) << 8;
    value |= static_cast<uint16_t>(bytes[startIndex + 1]);
    return value;
}

uint16_t udpLikeChecksum(vector<uint8_t> bytes) {
    uint32_t sum = 0;
    for (uint8_t value : bytes) {
        sum += value;
    }
    return static_cast<uint16_t>(sum % 65536);
}

vector<uint8_t> buildUdpLikeDatagram(uint16_t sourcePort, uint16_t destinationPort, string payload) {
    vector<uint8_t> bytes;
    appendUInt16BigEndian(bytes, sourcePort);
    appendUInt16BigEndian(bytes, destinationPort);

    uint16_t totalLength = static_cast<uint16_t>(8 + payload.size()); // 8 bytes header, then payload
    appendUInt16BigEndian(bytes, totalLength);

    appendUInt16BigEndian(bytes, 0); // checksum placeholder

    for (char character : payload) {
        bytes.push_back(static_cast<uint8_t>(character));
    }

    uint16_t checksum = udpLikeChecksum(bytes);
    bytes[6] = static_cast<uint8_t>((checksum >> 8) & 255);
    bytes[7] = static_cast<uint8_t>(checksum & 255);
    return bytes;
}

UdpLikeDatagram parseUdpLikeDatagram(vector<uint8_t> bytes) {
    if (bytes.size() < 8) {
        return UdpLikeDatagram(0, 0, "", false);
    }

    uint16_t sourcePort = readUInt16BigEndian(bytes, 0);
    uint16_t destinationPort = readUInt16BigEndian(bytes, 2);
    uint16_t totalLength = readUInt16BigEndian(bytes, 4);
    uint16_t receivedChecksum = readUInt16BigEndian(bytes, 6);

    if (bytes.size() != totalLength) {
        return UdpLikeDatagram(0, 0, "", false);
    }

    vector<uint8_t> checksumBytes = bytes;
    checksumBytes[6] = 0;
    checksumBytes[7] = 0;
    uint16_t expectedChecksum = udpLikeChecksum(checksumBytes);
    if (expectedChecksum != receivedChecksum) {
        return UdpLikeDatagram(0, 0, "", false);
    }

    string payload = "";
    for (size_t index = 8; index < bytes.size(); index += 1) {
        payload += static_cast<char>(bytes[index]);
    }

    return UdpLikeDatagram(sourcePort, destinationPort, payload, true);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<uint8_t> datagram = buildUdpLikeDatagram(50000, 53, "query=example.com");
    UdpLikeDatagram parsed = parseUdpLikeDatagram(datagram);

    cout << "[parseUdpLikeDatagram]\n";
    cout << "valid: " << parsed.isValid() << "\n";
    cout << "sourcePort: " << parsed.getSourcePort() << "\n";
    cout << "destinationPort: " << parsed.getDestinationPort() << "\n";
    cout << "payload: " << parsed.getPayload() << "\n";
    return 0;
}
#endif
