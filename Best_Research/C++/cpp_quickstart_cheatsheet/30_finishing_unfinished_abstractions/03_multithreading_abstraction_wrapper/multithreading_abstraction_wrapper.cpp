#include <condition_variable>
#include <functional>
#include <future>
#include <mutex>
#include <queue>
#include <string>
#include <thread>

using namespace std;

class SingleThreadTaskRunner {
private:
    queue<function<void()>> pendingTasks;
    mutex lock;
    condition_variable hasWork;
    bool stopRequested = false;
    thread worker;

    void workerLoop() {
        while (true) {
            function<void()> nextTask;
            {
                unique_lock<mutex> guard(lock);
                hasWork.wait(guard, [this]() {
                    return stopRequested || !pendingTasks.empty();
                });

                if (stopRequested && pendingTasks.empty()) {
                    return;
                }

                nextTask = pendingTasks.front();
                pendingTasks.pop();
            }
            nextTask();
        }
    }

public:
    SingleThreadTaskRunner() : worker(&SingleThreadTaskRunner::workerLoop, this) {
    }

    ~SingleThreadTaskRunner() {
        shutdown();
    }

    future<string> submit(function<string()> work) {
        shared_ptr<promise<string>> completion = make_shared<promise<string>>();
        future<string> result = completion->get_future();

        {
            lock_guard<mutex> guard(lock);
            pendingTasks.push([work, completion]() {
                completion->set_value(work());
            });
        }
        hasWork.notify_one();
        return result;
    }

    void shutdown() {
        {
            lock_guard<mutex> guard(lock);
            if (stopRequested) {
                return;
            }
            stopRequested = true;
        }
        hasWork.notify_one();
        if (worker.joinable()) {
            worker.join();
        }
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    SingleThreadTaskRunner runner;

    future<string> first = runner.submit([]() { return string("task-1"); });
    future<string> second = runner.submit([]() { return string("task-2"); });

    cout << "[SingleThreadTaskRunner]\n";
    cout << first.get() << "\n";
    cout << second.get() << "\n";
    runner.shutdown();
    return 0;
}
#endif
