#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "event_driven_patterns.cpp"

int main() {
    {
        EventBus bus;
        vector<string> calls;

        bus.subscribe("user_registered", [&calls](AppEvent eventValue) {
            calls.push_back("email:" + eventValue.getPayload());
        });
        bus.subscribe("user_registered", [&calls](AppEvent eventValue) {
            calls.push_back("analytics:" + eventValue.getPayload());
        });

        UserService users(bus);
        users.registerUser("nora@example.com");

        assert(calls.size() == 2);
        assert(calls[0] == "email:nora@example.com");
        assert(calls[1] == "analytics:nora@example.com");
    }

    return 0;
}
