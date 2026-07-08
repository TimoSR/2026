#include <cassert>
#include <memory>

using namespace std;

#include "layered_packet_sending_abstractions.cpp"

int main() {
    {
        shared_ptr<IMessageSerializer> serializer = make_shared<CsvTelemetrySerializer>();
        shared_ptr<IPacketBuilder> packetBuilder = make_shared<SimplePacketBuilder>();
        shared_ptr<SpyTransportSender> transport = make_shared<SpyTransportSender>();

        TelemetrySenderFacade sender(serializer, packetBuilder, transport);
        bool sent = sender.sendTelemetry(TelemetryMessage("device-7", 21, 64));
        assert(sent == true);
        assert(transport->getSentCount() == 1);

        string frame = transport->getLastFrame();
        assert(frame.find("telemetry.v1|") == 0);
        assert(frame.find("device-7,21,64") != string::npos);
    }

    {
        PacketFrame frame("x", "abc");
        string wire = toWireBytes(frame);
        assert(wire == "x|3|abc");
    }

    return 0;
}
