#include <memory>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

// Layer 1: domain message (business data)
class TelemetryMessage {
private:
    string deviceId = "";
    int temperature = 0;
    int batteryPercent = 0;

public:
    TelemetryMessage(string deviceIdValue, int temperatureValue, int batteryPercentValue)
        : deviceId(deviceIdValue), temperature(temperatureValue), batteryPercent(batteryPercentValue) {
    }

    string getDeviceId() const {
        return deviceId;
    }

    int getTemperature() const {
        return temperature;
    }

    int getBatteryPercent() const {
        return batteryPercent;
    }
};

// Layer 2: serialization abstraction
class IMessageSerializer {
public:
    virtual ~IMessageSerializer() = default;
    virtual string serializeTelemetry(TelemetryMessage message) = 0;
};

class CsvTelemetrySerializer : public IMessageSerializer {
public:
    string serializeTelemetry(TelemetryMessage message) override {
        return message.getDeviceId() + "," +
               to_string(message.getTemperature()) + "," +
               to_string(message.getBatteryPercent());
    }
};

// Layer 3: packet abstraction
class PacketFrame {
private:
    string protocol = "";
    string payload = "";

public:
    PacketFrame(string protocolValue, string payloadValue) : protocol(protocolValue), payload(payloadValue) {
    }

    string getProtocol() const {
        return protocol;
    }

    string getPayload() const {
        return payload;
    }
};

class IPacketBuilder {
public:
    virtual ~IPacketBuilder() = default;
    virtual PacketFrame build(string payload) = 0;
};

class SimplePacketBuilder : public IPacketBuilder {
public:
    PacketFrame build(string payload) override {
        return PacketFrame("telemetry.v1", payload);
    }
};

string toWireBytes(PacketFrame frame) {
    // Wire format: protocol|length|payload
    return frame.getProtocol() + "|" + to_string(frame.getPayload().size()) + "|" + frame.getPayload();
}

// Layer 4: transport abstraction
class IByteTransportSender {
public:
    virtual ~IByteTransportSender() = default;
    virtual bool send(string wireBytes) = 0;
};

class SpyTransportSender : public IByteTransportSender {
private:
    vector<string> sentFrames;

public:
    bool send(string wireBytes) override {
        sentFrames.push_back(wireBytes);
        return true;
    }

    int getSentCount() const {
        return static_cast<int>(sentFrames.size());
    }

    string getLastFrame() const {
        if (sentFrames.empty()) {
            return "";
        }
        return sentFrames.back();
    }
};

// Layer 5: facade over all layers for app usage
class TelemetrySenderFacade {
private:
    shared_ptr<IMessageSerializer> serializer;
    shared_ptr<IPacketBuilder> packetBuilder;
    shared_ptr<IByteTransportSender> transportSender;

public:
    TelemetrySenderFacade(
        shared_ptr<IMessageSerializer> serializerValue,
        shared_ptr<IPacketBuilder> packetBuilderValue,
        shared_ptr<IByteTransportSender> transportSenderValue
    ) : serializer(serializerValue), packetBuilder(packetBuilderValue), transportSender(transportSenderValue) {
    }

    bool sendTelemetry(TelemetryMessage message) {
        string serialized = serializer->serializeTelemetry(message);
        PacketFrame frame = packetBuilder->build(serialized);
        string wireBytes = toWireBytes(frame);
        return transportSender->send(wireBytes);
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    shared_ptr<IMessageSerializer> serializer = make_shared<CsvTelemetrySerializer>();
    shared_ptr<IPacketBuilder> packetBuilder = make_shared<SimplePacketBuilder>();
    shared_ptr<SpyTransportSender> transport = make_shared<SpyTransportSender>();

    TelemetrySenderFacade sender(serializer, packetBuilder, transport);
    sender.sendTelemetry(TelemetryMessage("device-42", 23, 87));

    cout << "[Layered send result]\n";
    cout << "sent frames: " << transport->getSentCount() << "\n";
    cout << "last frame: " << transport->getLastFrame() << "\n";
    return 0;
}
#endif
