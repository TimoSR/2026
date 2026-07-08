#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "redis_style_patterns.cpp"

int main() {
    {
        RedisStyleStore store;
        store.set("app:status", "healthy");
        string status = "";
        bool found = store.tryGet("app:status", status);
        assert(found == true);
        assert(status == "healthy");
    }

    {
        RedisStyleStore store;
        int first = store.increment("counter:orders");
        int second = store.increment("counter:orders");
        assert(first == 1);
        assert(second == 2);
        store.expireInSeconds("counter:orders", 60);
        assert(store.hasExpiration("counter:orders") == true);
    }

    {
        RedisStyleStore store;
        vector<string> events;
        store.subscribe("events", [&events](string payload) { events.push_back("a:" + payload); });
        store.subscribe("events", [&events](string payload) { events.push_back("b:" + payload); });
        store.publish("events", "order-created");

        assert(events.size() == 2);
        assert(events[0] == "a:order-created");
        assert(events[1] == "b:order-created");
    }

    return 0;
}
