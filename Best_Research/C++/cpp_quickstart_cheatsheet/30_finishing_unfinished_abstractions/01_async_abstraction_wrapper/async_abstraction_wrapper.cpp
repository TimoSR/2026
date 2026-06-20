#include <chrono>
#include <functional>
#include <future>
#include <string>

using namespace std;

class AsyncResult {
private:
    bool success = false;
    bool timedOut = false;
    string value = "";
    string error = "";

public:
    AsyncResult(bool successValue, bool timedOutValue, string valueText, string errorText)
        : success(successValue), timedOut(timedOutValue), value(valueText), error(errorText) {
    }

    bool isSuccess() const {
        return success;
    }

    bool isTimedOut() const {
        return timedOut;
    }

    string getValue() const {
        return value;
    }

    string getError() const {
        return error;
    }
};

class AsyncTextService {
public:
    string runAndWait(function<string()> work) {
        // Simple abstraction: hide future/promise from caller.
        future<string> pending = async(launch::async, work);
        return pending.get();
    }

    AsyncResult runWithTimeout(function<string()> work, int timeoutMilliseconds = 200) {
        future<string> pending = async(launch::async, work);
        future_status status = pending.wait_for(chrono::milliseconds(timeoutMilliseconds));

        if (status == future_status::ready) {
            return AsyncResult(true, false, pending.get(), "");
        }

        return AsyncResult(false, true, "", "Operation timed out.");
    }
};

#ifdef RUN_DEMO
#include <iostream>
#include <thread>

int main() {
    AsyncTextService service;

    cout << "[runAndWait]\n";
    string quickValue = service.runAndWait([]() { return string("quick result"); });
    cout << quickValue << "\n\n";

    cout << "[runWithTimeout]\n";
    AsyncResult timeoutResult = service.runWithTimeout([]() {
        this_thread::sleep_for(chrono::milliseconds(300));
        return string("slow");
    }, 100);
    cout << "success: " << timeoutResult.isSuccess() << ", timedOut: " << timeoutResult.isTimedOut() << "\n";
    return 0;
}
#endif
