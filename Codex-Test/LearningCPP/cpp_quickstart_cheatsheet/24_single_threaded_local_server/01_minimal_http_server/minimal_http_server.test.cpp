#include <cassert>
#include <string>

using namespace std;

#include "minimal_http_server.cpp"

int main() {
    {
        HttpRequest request = parseHttpRequestLine("GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert(request.isValid() == true);
        assert(request.getMethod() == "GET");
        assert(request.getPath() == "/health");
    }

    {
        string response = handleHttpRequest("GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert(response.find("200 OK") != string::npos);
        assert(response.find("\r\n\r\nok") != string::npos);
    }

    {
        string response = handleHttpRequest("POST /health HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert(response.find("405 Method Not Allowed") != string::npos);
    }

    {
        string response = handleHttpRequest("GET /missing HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert(response.find("404 Not Found") != string::npos);
    }

    {
        string response = handleHttpRequest("invalid request text");
        assert(response.find("400 Bad Request") != string::npos);
    }

    return 0;
}
