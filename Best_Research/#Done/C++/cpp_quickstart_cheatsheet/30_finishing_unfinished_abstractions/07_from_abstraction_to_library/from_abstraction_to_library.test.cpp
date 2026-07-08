#include <cassert>
#include <memory>

using namespace std;

#include "from_abstraction_to_library.cpp"

int main() {
    {
        shared_ptr<ILibrarySerializer> serializer = make_shared<StableSerializerV1>();
        shared_ptr<InMemoryTransport> transport = make_shared<InMemoryTransport>();

        PacketLibraryFacade library(serializer, transport);
        PacketLibraryResult result = library.publishKeyValue("device", "nora-1");

        assert(result.isSuccess() == true);
        assert(result.getLibraryVersion() == "1.0.0");
        assert(result.getTransportId() == "in-memory");
        assert(result.getWireFrame().find("pkt.v1|") == 0);
        assert(transport->sentCount() == 1);
        assert(payloadLengthFromWireFrame(result.getWireFrame()) == 13);
        assert(payloadFromWireFrame(result.getWireFrame()) == "device=nora-1");
        assert(isWireFrameWellFormed(result.getWireFrame()) == true);
        assert(isPublishResultConsistent(result) == true);
    }

    {
        assert(payloadLengthFromWireFrame("invalid") == -1);
        assert(isWireFrameWellFormed("pkt.v1|9|abc") == false);
    }

    return 0;
}
