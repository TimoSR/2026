#include <cassert>
#include <vector>

using namespace std;

#include "guid_basics.cpp"

int main() {
    {
        vector<uint8_t> bytes = {
            0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
            0x88, 0x99, 0xaa, 0xbb,
            0xcc, 0xdd, 0xee, 0xff
        };
        string guid = guidFrom16Bytes(bytes);
        assert(guid.size() == 36);
        assert(isValidGuidFormat(guid) == true);
        assert(guid[14] == '4');
    }

    {
        string guid = generateGuidV4();
        assert(isValidGuidFormat(guid) == true);
    }

    {
        assert(isValidGuidFormat("not-a-guid") == false);
        assert(isValidGuidFormat("00112233-4455-6677-8899-aabbccddeeff") == false); // version is not v4
    }

    return 0;
}
