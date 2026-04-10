#include <memory>
#include <string>

using namespace std;

class IByteTransport {
public:
    virtual ~IByteTransport() = default;
    virtual string sendAndReceive(string requestBytes) = 0;
};

class SocketApiResponse {
private:
    int statusCode = 0;
    string body = "";
    bool valid = false;

public:
    SocketApiResponse(int statusCodeValue, string bodyValue, bool validValue)
        : statusCode(statusCodeValue), body(bodyValue), valid(validValue) {
    }

    int getStatusCode() const {
        return statusCode;
    }

    string getBody() const {
        return body;
    }

    bool isValid() const {
        return valid;
    }
};

string buildWireRequest(string command, string payload) {
    return command + "\n" + payload;
}

SocketApiResponse parseWireResponse(string rawResponse) {
    size_t lineEnd = rawResponse.find("\n");
    if (lineEnd == string::npos) {
        return SocketApiResponse(0, "", false);
    }

    string codeLine = rawResponse.substr(0, lineEnd);
    string body = rawResponse.substr(lineEnd + 1);
    int statusCode = stoi(codeLine);
    return SocketApiResponse(statusCode, body, true);
}

class UserSocketClient {
private:
    shared_ptr<IByteTransport> transport;

public:
    UserSocketClient(shared_ptr<IByteTransport> transportValue) : transport(transportValue) {
    }

    bool isHealthy() {
        string request = buildWireRequest("HEALTH", "");
        string response = transport->sendAndReceive(request);
        SocketApiResponse parsed = parseWireResponse(response);
        return parsed.isValid() && parsed.getStatusCode() == 200;
    }

    string echo(string message) {
        string request = buildWireRequest("ECHO", message);
        string response = transport->sendAndReceive(request);
        SocketApiResponse parsed = parseWireResponse(response);
        if (!parsed.isValid() || parsed.getStatusCode() != 200) {
            return "";
        }
        return parsed.getBody();
    }
};

class FakeTransport : public IByteTransport {
public:
    string sendAndReceive(string requestBytes) override {
        if (requestBytes.rfind("HEALTH", 0) == 0) {
            return "200\nok";
        }
        if (requestBytes.rfind("ECHO\n", 0) == 0) {
            return "200\n" + requestBytes.substr(5);
        }
        return "500\nunsupported";
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    shared_ptr<IByteTransport> transport = make_shared<FakeTransport>();
    UserSocketClient client(transport);

    cout << "[isHealthy]\n" << client.isHealthy() << "\n\n";
    cout << "[echo]\n" << client.echo("hello socket abstraction") << "\n";
    return 0;
}
#endif
