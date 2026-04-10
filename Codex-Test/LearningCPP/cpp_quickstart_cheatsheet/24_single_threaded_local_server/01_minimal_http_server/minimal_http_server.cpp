#include <sstream>
#include <string>

using namespace std;

class HttpRequest {
private:
    string method = "";
    string path = "";
    string httpVersion = "";
    bool valid = false;

public:
    HttpRequest(string methodValue, string pathValue, string versionValue, bool validValue)
        : method(methodValue), path(pathValue), httpVersion(versionValue), valid(validValue) {
    }

    string getMethod() const {
        return method;
    }

    string getPath() const {
        return path;
    }

    string getHttpVersion() const {
        return httpVersion;
    }

    bool isValid() const {
        return valid;
    }
};

HttpRequest parseHttpRequestLine(string rawRequest) {
    // Parse only first line: METHOD PATH HTTP/VERSION
    size_t lineEnd = rawRequest.find("\r\n");
    string firstLine = lineEnd == string::npos ? rawRequest : rawRequest.substr(0, lineEnd);

    stringstream input(firstLine);
    string method = "";
    string path = "";
    string version = "";
    input >> method >> path >> version;

    if (method.empty() || path.empty() || version.empty()) {
        return HttpRequest("", "", "", false);
    }

    bool knownMethod = method == "GET" || method == "POST" || method == "PUT" || method == "DELETE" || method == "PATCH";
    if (!knownMethod) {
        return HttpRequest("", "", "", false);
    }

    if (path[0] != '/') {
        return HttpRequest("", "", "", false);
    }

    if (version.rfind("HTTP/", 0) != 0) {
        return HttpRequest("", "", "", false);
    }

    return HttpRequest(method, path, version, true);
}

string buildHttpResponse(int statusCode, string statusText, string bodyText) {
    string response = "";
    response += "HTTP/1.1 " + to_string(statusCode) + " " + statusText + "\r\n";
    response += "Content-Type: text/plain; charset=utf-8\r\n";
    response += "Content-Length: " + to_string(bodyText.size()) + "\r\n";
    response += "Connection: close\r\n";
    response += "\r\n";
    response += bodyText;
    return response;
}

string handleHttpRequest(string rawRequest) {
    HttpRequest request = parseHttpRequestLine(rawRequest);
    if (!request.isValid()) {
        return buildHttpResponse(400, "Bad Request", "Invalid HTTP request.");
    }

    if (request.getMethod() != "GET") {
        return buildHttpResponse(405, "Method Not Allowed", "Only GET is supported.");
    }

    if (request.getPath() == "/health") {
        return buildHttpResponse(200, "OK", "ok");
    }

    if (request.getPath() == "/hello") {
        return buildHttpResponse(200, "OK", "hello from single-threaded server");
    }

    return buildHttpResponse(404, "Not Found", "Route not found.");
}

#ifdef RUN_DEMO
#include <iostream>

#include <winsock2.h>
#include <ws2tcpip.h>
#pragma comment(lib, "Ws2_32.lib")

bool runSingleThreadedServerOnce(int port) {
    WSADATA winsockData;
    int startupResult = WSAStartup(MAKEWORD(2, 2), &winsockData);
    if (startupResult != 0) {
        return false;
    }

    SOCKET listenSocket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (listenSocket == INVALID_SOCKET) {
        WSACleanup();
        return false;
    }

    sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_port = htons(static_cast<u_short>(port));
    inet_pton(AF_INET, "127.0.0.1", &address.sin_addr);

    if (bind(listenSocket, reinterpret_cast<sockaddr*>(&address), sizeof(address)) == SOCKET_ERROR) {
        closesocket(listenSocket);
        WSACleanup();
        return false;
    }

    if (listen(listenSocket, 1) == SOCKET_ERROR) {
        closesocket(listenSocket);
        WSACleanup();
        return false;
    }

    cout << "Listening on http://127.0.0.1:" << port << "\n";
    cout << "Single-threaded: handling one connection at a time.\n";

    SOCKET clientSocket = accept(listenSocket, nullptr, nullptr);
    if (clientSocket == INVALID_SOCKET) {
        closesocket(listenSocket);
        WSACleanup();
        return false;
    }

    char buffer[4096];
    int bytesReceived = recv(clientSocket, buffer, sizeof(buffer), 0);
    string requestText = bytesReceived > 0 ? string(buffer, buffer + bytesReceived) : "";
    string responseText = handleHttpRequest(requestText);

    send(clientSocket, responseText.c_str(), static_cast<int>(responseText.size()), 0);

    closesocket(clientSocket);
    closesocket(listenSocket);
    WSACleanup();
    return true;
}

int main() {
    bool ok = runSingleThreadedServerOnce(8080);
    if (!ok) {
        cerr << "Server startup or request handling failed.\n";
        return 1;
    }
    return 0;
}
#endif
