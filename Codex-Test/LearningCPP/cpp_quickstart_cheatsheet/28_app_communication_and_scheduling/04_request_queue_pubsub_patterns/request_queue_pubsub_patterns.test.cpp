#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "request_queue_pubsub_patterns.cpp"

int main() {
    {
        string response = requestResponsePing("users-api", "health");
        assert(response == "response from users-api: health");
    }

    {
        MessageQueue queue;
        queue.enqueue("a");
        queue.enqueue("b");

        string first = "";
        string second = "";
        bool firstOk = queue.tryDequeue(first);
        bool secondOk = queue.tryDequeue(second);

        assert(firstOk == true);
        assert(secondOk == true);
        assert(first == "a");
        assert(second == "b");
        assert(queue.size() == 0);
    }

    {
        PubSubBus bus;
        vector<string> received;
        bus.subscribe("topic", [&received](string message) { received.push_back("one:" + message); });
        bus.subscribe("topic", [&received](string message) { received.push_back("two:" + message); });
        bus.publish("topic", "hello");

        assert(received.size() == 2);
        assert(received[0] == "one:hello");
        assert(received[1] == "two:hello");
    }

    return 0;
}
