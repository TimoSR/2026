#include <functional>
#include <map>
#include <string>
#include <vector>

using namespace std;

// 1) Request/response: caller waits for immediate result.
string requestResponsePing(string endpoint, string requestBody) {
    return "response from " + endpoint + ": " + requestBody;
}

// 2) Queue: producer pushes, consumer pulls later.
class MessageQueue {
private:
    vector<string> messages;

public:
    void enqueue(string message) {
        messages.push_back(message);
    }

    bool tryDequeue(string& message) {
        if (messages.empty()) {
            return false;
        }
        message = messages.front();
        messages.erase(messages.begin());
        return true;
    }

    int size() const {
        return static_cast<int>(messages.size());
    }
};

// 3) Pub/Sub: publisher broadcasts to all subscribers on topic.
class PubSubBus {
private:
    map<string, vector<function<void(string)>>> subscribersByTopic;

public:
    void subscribe(string topic, function<void(string)> handler) {
        subscribersByTopic[topic].push_back(handler);
    }

    void publish(string topic, string message) {
        map<string, vector<function<void(string)>>>::iterator found = subscribersByTopic.find(topic);
        if (found == subscribersByTopic.end()) {
            return;
        }

        for (function<void(string)> handler : found->second) {
            handler(message);
        }
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[request/response]\n";
    cout << requestResponsePing("orders-api", "ping") << "\n\n";

    cout << "[queue]\n";
    MessageQueue queue;
    queue.enqueue("send_receipt:order-1");
    queue.enqueue("send_receipt:order-2");
    string work = "";
    while (queue.tryDequeue(work)) {
        cout << "worker handled " << work << "\n";
    }
    cout << "\n";

    cout << "[pub/sub]\n";
    PubSubBus bus;
    bus.subscribe("order_created", [](string message) { cout << "email: " << message << "\n"; });
    bus.subscribe("order_created", [](string message) { cout << "analytics: " << message << "\n"; });
    bus.publish("order_created", "order-200");
    return 0;
}
#endif
