#include <string>

using namespace std;

class FormatNeed {
private:
    bool humanReadable = false;
    bool strictSchema = false;
    bool smallestPayloadPreferred = false;
    bool legacyEnterpriseCompatibility = false;

public:
    FormatNeed(bool readable, bool schema, bool smallest, bool legacy)
        : humanReadable(readable),
          strictSchema(schema),
          smallestPayloadPreferred(smallest),
          legacyEnterpriseCompatibility(legacy) {
    }

    bool wantsHumanReadable() const {
        return humanReadable;
    }

    bool wantsStrictSchema() const {
        return strictSchema;
    }

    bool wantsSmallestPayload() const {
        return smallestPayloadPreferred;
    }

    bool wantsLegacyEnterpriseCompatibility() const {
        return legacyEnterpriseCompatibility;
    }
};

FormatNeed defaultApiNeedProfile() {
    return FormatNeed(true, false, false, false);
}

FormatNeed highThroughputInternalNeedProfile() {
    return FormatNeed(false, true, true, false);
}

FormatNeed legacyEnterpriseNeedProfile() {
    return FormatNeed(true, false, false, true);
}

string chooseFormat(FormatNeed need) {
    if (need.wantsLegacyEnterpriseCompatibility()) {
        return "xml";
    }
    if (need.wantsStrictSchema() && need.wantsSmallestPayload()) {
        return "protobuf";
    }
    if (need.wantsHumanReadable()) {
        return "json";
    }
    if (need.wantsSmallestPayload()) {
        return "messagepack";
    }
    return "json";
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[defaultApiNeedProfile] => " << chooseFormat(defaultApiNeedProfile()) << "\n";
    cout << "[highThroughputInternalNeedProfile] => " << chooseFormat(highThroughputInternalNeedProfile()) << "\n";
    cout << "[legacyEnterpriseNeedProfile] => " << chooseFormat(legacyEnterpriseNeedProfile()) << "\n";
    return 0;
}
#endif
