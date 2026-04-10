#include <functional>
#include <map>
#include <string>
#include <vector>

using namespace std;

class RedisStyleStore {
private:
    map<string, string> keyValues;
    map<string, int> keyExpirations;
    map<string, vector<function<void(string)>>> subscribersByChannel;

public:
    void set(string key, string value) {
        keyValues[key] = value;
    }

    bool tryGet(string key, string& value) const {
        map<string, string>::const_iterator found = keyValues.find(key);
        if (found == keyValues.end()) {
            return false;
        }
        value = found->second;
        return true;
    }

    int increment(string key) {
        string currentText = "0";
        tryGet(key, currentText);

        int current = 0;
        for (char character : currentText) {
            if (character < '0' || character > '9') {
                current = 0;
                break;
            }
            current = (current * 10) + (character - '0');
        }

        current += 1;
        keyValues[key] = to_string(current);
        return current;
    }

    void expireInSeconds(string key, int seconds) {
        keyExpirations[key] = seconds;
    }

    bool hasExpiration(string key) const {
        return keyExpirations.find(key) != keyExpirations.end();
    }

    void subscribe(string channel, function<void(string)> handler) {
        subscribersByChannel[channel].push_back(handler);
    }

    void publish(string channel, string payload) {
        map<string, vector<function<void(string)>>>::iterator found = subscribersByChannel.find(channel);
        if (found == subscribersByChannel.end()) {
            return;
        }

        for (function<void(string)> handler : found->second) {
            handler(payload);
        }
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    RedisStyleStore redis;

    cout << "[cache]\n";
    redis.set("user:100:name", "Nora");
    string name = "";
    bool hasName = redis.tryGet("user:100:name", name);
    cout << "hasName: " << hasName << ", value: " << name << "\n\n";

    cout << "[counter + ttl]\n";
    int count1 = redis.increment("rate:user:100");
    int count2 = redis.increment("rate:user:100");
    redis.expireInSeconds("rate:user:100", 60);
    cout << "count1: " << count1 << ", count2: " << count2 << "\n";
    cout << "has ttl: " << redis.hasExpiration("rate:user:100") << "\n\n";

    cout << "[pub/sub]\n";
    redis.subscribe("orders", [](string message) { cout << "worker-a: " << message << "\n"; });
    redis.subscribe("orders", [](string message) { cout << "worker-b: " << message << "\n"; });
    redis.publish("orders", "order-300-created");
    return 0;
}
#endif
