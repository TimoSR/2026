#include <cassert>

using namespace std;

#include "format_selection_profiles.cpp"

int main() {
    assert(chooseFormat(defaultApiNeedProfile()) == "json");
    assert(chooseFormat(highThroughputInternalNeedProfile()) == "protobuf");
    assert(chooseFormat(legacyEnterpriseNeedProfile()) == "xml");
    assert(chooseFormat(FormatNeed(false, false, true, false)) == "messagepack");
    return 0;
}
