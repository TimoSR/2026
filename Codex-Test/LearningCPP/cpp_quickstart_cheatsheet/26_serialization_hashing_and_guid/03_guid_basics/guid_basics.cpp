#include <cstdint>
#include <iomanip>
#include <random>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

string byteToHex(uint8_t value) {
    stringstream output;
    output << hex << nouppercase << setw(2) << setfill('0') << static_cast<int>(value);
    return output.str();
}

string guidFrom16Bytes(vector<uint8_t> bytes) {
    if (bytes.size() != 16) {
        return "";
    }

    // RFC 4122 v4 bits:
    // byte 6 high nibble = 0100 (version 4)
    // byte 8 high bits = 10xxxxxx (variant)
    bytes[6] = static_cast<uint8_t>((bytes[6] & 0x0F) | 0x40);
    bytes[8] = static_cast<uint8_t>((bytes[8] & 0x3F) | 0x80);

    string text = "";
    for (size_t index = 0; index < bytes.size(); index += 1) {
        if (index == 4 || index == 6 || index == 8 || index == 10) {
            text += "-";
        }
        text += byteToHex(bytes[index]);
    }
    return text;
}

vector<uint8_t> random16Bytes() {
    random_device randomSeed;
    mt19937 generator(randomSeed());
    uniform_int_distribution<int> distribution(0, 255);

    vector<uint8_t> bytes(16);
    for (int index = 0; index < 16; index += 1) {
        bytes[index] = static_cast<uint8_t>(distribution(generator));
    }
    return bytes;
}

string generateGuidV4() {
    return guidFrom16Bytes(random16Bytes());
}

bool isHexLower(char character) {
    return (character >= '0' && character <= '9') || (character >= 'a' && character <= 'f');
}

bool isValidGuidFormat(string text) {
    if (text.size() != 36) {
        return false;
    }

    for (size_t index = 0; index < text.size(); index += 1) {
        if (index == 8 || index == 13 || index == 18 || index == 23) {
            if (text[index] != '-') {
                return false;
            }
            continue;
        }

        if (!isHexLower(text[index])) {
            return false;
        }
    }

    // Version nibble at index 14 must be 4 for v4 GUID.
    if (text[14] != '4') {
        return false;
    }

    // Variant nibble at index 19 must be 8,9,a,b.
    char variant = text[19];
    if (!(variant == '8' || variant == '9' || variant == 'a' || variant == 'b')) {
        return false;
    }

    return true;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[generateGuidV4]\n";
    string guid = generateGuidV4();
    cout << guid << "\n\n";

    cout << "[isValidGuidFormat]\n";
    cout << isValidGuidFormat(guid) << "\n";
    return 0;
}
#endif
