#include <cstdint>
#include <string>
#include <vector>

using namespace std;

class ParsedPacket {
private:
    uint8_t version = 0;
    uint8_t messageType = 0;
    string payload = "";
    bool valid = false;

public:
    ParsedPacket(uint8_t versionNumber, uint8_t typeNumber, string payloadText, bool isValid)
        : version(versionNumber), messageType(typeNumber), payload(payloadText), valid(isValid) {
    }

    uint8_t getVersion() const {
        return version;
    }

    uint8_t getMessageType() const {
        return messageType;
    }

    string getPayload() const {
        return payload;
    }

    bool isValid() const {
        return valid;
    }
};

uint8_t checksumOf(vector<uint8_t> bytes) {
    uint32_t sum = 0;
    for (uint8_t value : bytes) {
        sum += value;
    }
    return static_cast<uint8_t>(sum % 256);
}

vector<uint8_t> buildPacketBytes(uint8_t version, uint8_t messageType, string payload) {
    vector<uint8_t> bytes;
    bytes.push_back(version);
    bytes.push_back(messageType);

    uint16_t payloadLength = static_cast<uint16_t>(payload.size());
    bytes.push_back(static_cast<uint8_t>((payloadLength >> 8) & 255));
    bytes.push_back(static_cast<uint8_t>(payloadLength & 255));

    for (char character : payload) {
        bytes.push_back(static_cast<uint8_t>(character));
    }

    uint8_t checksum = checksumOf(bytes);
    bytes.push_back(checksum);
    return bytes;
}

ParsedPacket parsePacketBytes(vector<uint8_t> bytes) {
    if (bytes.size() < 5) {
        return ParsedPacket(0, 0, "", false);
    }

    vector<uint8_t> headerAndPayload(bytes.begin(), bytes.end() - 1);
    uint8_t expectedChecksum = checksumOf(headerAndPayload);
    uint8_t actualChecksum = bytes[bytes.size() - 1];
    if (expectedChecksum != actualChecksum) {
        return ParsedPacket(0, 0, "", false);
    }

    uint8_t version = bytes[0];
    uint8_t type = bytes[1];
    uint16_t payloadLength = static_cast<uint16_t>(bytes[2]) << 8;
    payloadLength |= static_cast<uint16_t>(bytes[3]);

    if (bytes.size() != static_cast<size_t>(4 + payloadLength + 1)) {
        return ParsedPacket(0, 0, "", false);
    }

    string payload = "";
    for (size_t index = 4; index < 4 + payloadLength; index += 1) {
        payload += static_cast<char>(bytes[index]);
    }

    return ParsedPacket(version, type, payload, true);
}

string toHexString(vector<uint8_t> bytes) {
    const string digits = "0123456789ABCDEF";
    string text = "";

    for (size_t index = 0; index < bytes.size(); index += 1) {
        uint8_t value = bytes[index];
        text += digits[(value >> 4) & 15];
        text += digits[value & 15];
        if (index + 1 != bytes.size()) {
            text += " ";
        }
    }

    return text;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    vector<uint8_t> packet = buildPacketBytes(1, 2, "PING");
    ParsedPacket parsed = parsePacketBytes(packet);

    cout << "[buildPacketBytes]\n";
    cout << toHexString(packet) << "\n\n";

    cout << "[parsePacketBytes]\n";
    cout << "valid: " << parsed.isValid() << "\n";
    cout << "version: " << static_cast<int>(parsed.getVersion()) << "\n";
    cout << "type: " << static_cast<int>(parsed.getMessageType()) << "\n";
    cout << "payload: " << parsed.getPayload() << "\n";
    return 0;
}
#endif
