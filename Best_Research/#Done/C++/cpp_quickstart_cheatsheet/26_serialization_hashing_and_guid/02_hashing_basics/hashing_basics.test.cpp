#include <cassert>

using namespace std;

#include "hashing_basics.cpp"

int main() {
    {
        uint64_t hashA = fnv1a64("nora@example.com");
        uint64_t hashB = fnv1a64("nora@example.com");
        assert(hashA == hashB);
    }

    {
        uint64_t hashA = fnv1a64("nora@example.com");
        uint64_t hashB = fnv1a64("ava@example.com");
        assert(hashA != hashB);
    }

    {
        string hex = toHex64(0xA1B2C3u);
        assert(hex == "0000000000A1B2C3");
    }

    {
        uint64_t combined = combineHashes(10, 20);
        assert(combined != 10);
        assert(combined != 20);
    }

    return 0;
}
