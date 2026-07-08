#include <functional>
#include <map>
#include <string>
#include <vector>

using namespace std;

class AppEvent {
private:
    string name = "";
    string payload = "";

public:
    AppEvent(string nameValue, string payloadValue) : name(nameValue), payload(payloadValue) {
    }

    string getName() const {
        return name;
    }

    string getPayload() const {
        return payload;
    }
};

class EventBus {
private:
    map<string, vector<function<void(AppEvent)>>> handlersByEventName;

public:
    void subscribe(string eventName, function<void(AppEvent)> handler) {
        handlersByEventName[eventName].push_back(handler);
    }

    void publish(AppEvent eventValue) {
        map<string, vector<function<void(AppEvent)>>>::iterator handlers =
            handlersByEventName.find(eventValue.getName());
        if (handlers == handlersByEventName.end()) {
            return;
        }

        for (function<void(AppEvent)> handler : handlers->second) {
            handler(eventValue);
        }
    }
};

class UserService {
private:
    EventBus& bus;

public:
    UserService(EventBus& busValue) : bus(busValue) {
    }

    void registerUser(string email) {
        bus.publish(AppEvent("user_registered", email));
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    EventBus bus;

    bus.subscribe("user_registered", [](AppEvent eventValue) {
        cout << "[Email Service] send welcome to " << eventValue.getPayload() << "\n";
    });

    bus.subscribe("user_registered", [](AppEvent eventValue) {
        cout << "[Analytics Service] track registration " << eventValue.getPayload() << "\n";
    });

    UserService users(bus);
    users.registerUser("nora@example.com");
    return 0;
}
#endif
