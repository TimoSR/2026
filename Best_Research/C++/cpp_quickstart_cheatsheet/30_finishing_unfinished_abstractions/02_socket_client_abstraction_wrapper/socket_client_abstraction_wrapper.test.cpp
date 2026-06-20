#include <cassert>
#include <memory>

using namespace std;

#include "socket_client_abstraction_wrapper.cpp"

int main() {
    {
        shared_ptr<IByteTransport> transport = make_shared<FakeTransport>();
        UserSocketClient client(transport);
        assert(client.isHealthy() == true);
    }

    {
        shared_ptr<IByteTransport> transport = make_shared<FakeTransport>();
        UserSocketClient client(transport);
        assert(client.echo("nora") == "nora");
    }

    {
        SocketApiResponse parsed = parseWireResponse("200\nhello");
        assert(parsed.isValid() == true);
        assert(parsed.getStatusCode() == 200);
        assert(parsed.getBody() == "hello");
    }

    return 0;
}
