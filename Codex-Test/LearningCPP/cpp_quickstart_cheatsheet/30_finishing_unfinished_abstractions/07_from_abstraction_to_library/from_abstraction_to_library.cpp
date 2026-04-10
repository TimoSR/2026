#include <memory>
#include <string>
#include <vector>

using namespace std;

// Public API shape (what consumers should depend on).
class PacketLibraryResult {
private:
    bool success = false;
    string libraryVersion = "";
    string transportId = "";
    string wireFrame = "";

public:
    PacketLibraryResult(bool successValue, string versionValue, string transportIdValue, string wireFrameValue)
        : success(successValue), libraryVersion(versionValue), transportId(transportIdValue), wireFrame(wireFrameValue) {
    }

    bool isSuccess() const {
        return success;
    }

    string getLibraryVersion() const {
        return libraryVersion;
    }

    string getTransportId() const {
        return transportId;
    }

    string getWireFrame() const {
        return wireFrame;
    }
};

// Extension points (adapters) a library exposes for host projects.
class ILibrarySerializer {
public:
    virtual ~ILibrarySerializer() = default;
    virtual string serialize(string key, string value) = 0;
};

class ILibraryTransport {
public:
    virtual ~ILibraryTransport() = default;
    virtual bool send(string wireBytes) = 0;
    virtual string transportId() = 0;
};

class StableSerializerV1 : public ILibrarySerializer {
public:
    string serialize(string key, string value) override {
        return key + "=" + value;
    }
};

class InMemoryTransport : public ILibraryTransport {
private:
    string id = "in-memory";
    vector<string> sentFrames;

public:
    bool send(string wireBytes) override {
        sentFrames.push_back(wireBytes);
        return true;
    }

    string transportId() override {
        return id;
    }

    int sentCount() const {
        return static_cast<int>(sentFrames.size());
    }

    string lastFrame() const {
        if (sentFrames.empty()) {
            return "";
        }
        return sentFrames.back();
    }
};

// Library facade: small stable API, internal details hidden.
class PacketLibraryFacade {
private:
    shared_ptr<ILibrarySerializer> serializer;
    shared_ptr<ILibraryTransport> transport;
    string version = "1.0.0";

public:
    PacketLibraryFacade(
        shared_ptr<ILibrarySerializer> serializerValue,
        shared_ptr<ILibraryTransport> transportValue
    ) : serializer(serializerValue), transport(transportValue) {
    }

    PacketLibraryResult publishKeyValue(string key, string value) {
        string payload = serializer->serialize(key, value);
        string wire = "pkt.v1|" + to_string(payload.size()) + "|" + payload;
        bool ok = transport->send(wire);
        return PacketLibraryResult(ok, version, transport->transportId(), wire);
    }
};

bool startsWith(string value, string prefix) {
    if (value.size() < prefix.size()) {
        return false;
    }
    return value.substr(0, prefix.size()) == prefix;
}

bool isDigitsOnly(string value) {
    if (value.empty()) {
        return false;
    }

    for (char character : value) {
        if (character < '0' || character > '9') {
            return false;
        }
    }
    return true;
}

int payloadLengthFromWireFrame(string wireFrame) {
    string prefix = "pkt.v1|";
    if (!startsWith(wireFrame, prefix)) {
        return -1;
    }

    size_t lengthStart = prefix.size();
    size_t lengthEnd = wireFrame.find("|", lengthStart);
    if (lengthEnd == string::npos) {
        return -1;
    }

    string payloadLengthText = wireFrame.substr(lengthStart, lengthEnd - lengthStart);
    if (!isDigitsOnly(payloadLengthText)) {
        return -1;
    }

    try {
        return stoi(payloadLengthText);
    } catch (...) {
        return -1;
    }
}

string payloadFromWireFrame(string wireFrame) {
    string prefix = "pkt.v1|";
    if (!startsWith(wireFrame, prefix)) {
        return "";
    }

    size_t lengthStart = prefix.size();
    size_t lengthEnd = wireFrame.find("|", lengthStart);
    if (lengthEnd == string::npos) {
        return "";
    }

    return wireFrame.substr(lengthEnd + 1);
}

bool isWireFrameWellFormed(string wireFrame) {
    int declaredLength = payloadLengthFromWireFrame(wireFrame);
    if (declaredLength < 0) {
        return false;
    }

    string payload = payloadFromWireFrame(wireFrame);
    return static_cast<int>(payload.size()) == declaredLength;
}

bool isPublishResultConsistent(PacketLibraryResult result) {
    if (!result.isSuccess()) {
        return false;
    }
    if (result.getLibraryVersion() != "1.0.0") {
        return false;
    }
    if (result.getTransportId().empty()) {
        return false;
    }
    return isWireFrameWellFormed(result.getWireFrame());
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    shared_ptr<ILibrarySerializer> serializer = make_shared<StableSerializerV1>();
    shared_ptr<InMemoryTransport> transport = make_shared<InMemoryTransport>();

    PacketLibraryFacade library(serializer, transport);
    PacketLibraryResult result = library.publishKeyValue("temperature", "21");

    cout << "[PacketLibraryFacade publish]\n";
    cout << "success: " << result.isSuccess() << "\n";
    cout << "version: " << result.getLibraryVersion() << "\n";
    cout << "transport: " << result.getTransportId() << "\n";
    cout << "wire: " << result.getWireFrame() << "\n";
    cout << "payload size: " << payloadLengthFromWireFrame(result.getWireFrame()) << "\n";
    cout << "wire well formed: " << isWireFrameWellFormed(result.getWireFrame()) << "\n";
    cout << "result consistent: " << isPublishResultConsistent(result) << "\n";
    return 0;
}
#endif
