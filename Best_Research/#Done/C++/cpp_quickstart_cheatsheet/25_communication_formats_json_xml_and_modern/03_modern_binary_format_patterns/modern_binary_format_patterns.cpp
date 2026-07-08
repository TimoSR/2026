#include <string>
#include <vector>

using namespace std;

class FormatProfile {
private:
    string name = "";
    bool schemaRequired = false;
    bool humanReadable = false;
    bool strongCrossLanguageSupport = false;
    int typicalPayloadBytes = 0;

public:
    FormatProfile(
        string nameValue,
        bool schemaRequiredValue,
        bool humanReadableValue,
        bool crossLanguageValue,
        int typicalBytesValue
    ) : name(nameValue),
        schemaRequired(schemaRequiredValue),
        humanReadable(humanReadableValue),
        strongCrossLanguageSupport(crossLanguageValue),
        typicalPayloadBytes(typicalBytesValue) {
    }

    string getName() const {
        return name;
    }

    bool getSchemaRequired() const {
        return schemaRequired;
    }

    bool getHumanReadable() const {
        return humanReadable;
    }

    bool getStrongCrossLanguageSupport() const {
        return strongCrossLanguageSupport;
    }

    int getTypicalPayloadBytes() const {
        return typicalPayloadBytes;
    }
};

FormatProfile jsonProfile() {
    return FormatProfile("json", false, true, true, 220);
}

FormatProfile xmlProfile() {
    return FormatProfile("xml", false, true, true, 320);
}

FormatProfile messagePackProfile() {
    return FormatProfile("messagepack", false, false, true, 140);
}

FormatProfile cborProfile() {
    return FormatProfile("cbor", false, false, true, 150);
}

FormatProfile protobufProfile() {
    return FormatProfile("protobuf", true, false, true, 110);
}

FormatProfile avroProfile() {
    return FormatProfile("avro", true, false, true, 130);
}

vector<FormatProfile> modernProfiles() {
    vector<FormatProfile> profiles;
    profiles.push_back(messagePackProfile());
    profiles.push_back(cborProfile());
    profiles.push_back(protobufProfile());
    profiles.push_back(avroProfile());
    return profiles;
}

string describeProfile(FormatProfile profile) {
    return profile.getName() +
           ": bytes~" + to_string(profile.getTypicalPayloadBytes()) +
           ", schema=" + (profile.getSchemaRequired() ? "yes" : "no") +
           ", readable=" + (profile.getHumanReadable() ? "yes" : "no");
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[Classic formats]\n";
    cout << describeProfile(jsonProfile()) << "\n";
    cout << describeProfile(xmlProfile()) << "\n\n";

    cout << "[Modern binary alternatives]\n";
    vector<FormatProfile> profiles = modernProfiles();
    for (FormatProfile profile : profiles) {
        cout << describeProfile(profile) << "\n";
    }
    return 0;
}
#endif
