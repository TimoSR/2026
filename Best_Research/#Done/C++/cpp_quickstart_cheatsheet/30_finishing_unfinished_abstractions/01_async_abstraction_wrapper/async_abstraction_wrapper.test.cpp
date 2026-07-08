#include <cassert>
#include <chrono>
#include <thread>

using namespace std;

#include "async_abstraction_wrapper.cpp"

int main() {
    AsyncTextService service;

    {
        string result = service.runAndWait([]() { return string("ok"); });
        assert(result == "ok");
    }

    {
        AsyncResult result = service.runWithTimeout([]() { return string("quick"); }, 100);
        assert(result.isSuccess() == true);
        assert(result.isTimedOut() == false);
        assert(result.getValue() == "quick");
    }

    {
        AsyncResult result = service.runWithTimeout([]() {
            this_thread::sleep_for(chrono::milliseconds(150));
            return string("slow");
        }, 30);
        assert(result.isSuccess() == false);
        assert(result.isTimedOut() == true);
    }

    return 0;
}
