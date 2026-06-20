#include <atomic>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

class CounterWithMutex {
private:
    int count = 0;
    mutex countMutex;

public:
    void increment() {
        lock_guard<mutex> lock(countMutex);
        count += 1;
    }

    int getValue() const {
        return count;
    }
};

int runThreadedCounterWithMutex(int threadCount, int incrementsPerThread) {
    CounterWithMutex counter;
    vector<thread> workers;

    for (int index = 0; index < threadCount; index++) {
        workers.push_back(thread([&counter, incrementsPerThread]() {
            for (int i = 0; i < incrementsPerThread; i++) {
                counter.increment();
            }
        }));
    }

    for (thread& worker : workers) {
        worker.join();
    }

    return counter.getValue();
}

int runThreadedCounterWithAtomic(int threadCount, int incrementsPerThread) {
    atomic<int> count{0};
    vector<thread> workers;

    for (int index = 0; index < threadCount; index++) {
        workers.push_back(thread([&count, incrementsPerThread]() {
            for (int i = 0; i < incrementsPerThread; i++) {
                count.fetch_add(1);
            }
        }));
    }

    for (thread& worker : workers) {
        worker.join();
    }

    return count.load();
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    int withMutex = runThreadedCounterWithMutex(4, 1000);
    int withAtomic = runThreadedCounterWithAtomic(4, 1000);

    cout << "Mutex count: " << withMutex << "\n";
    cout << "Atomic count: " << withAtomic << "\n";
    return 0;
}
#endif
