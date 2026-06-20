#include <chrono>
#include <future>
#include <string>
#include <thread>

using namespace std;

int computeScoreSlowly(int baseScore) {
    this_thread::sleep_for(chrono::milliseconds(30));
    return baseScore * 2;
}

future<int> startScoreComputation(int baseScore) {
    return async(launch::async, [baseScore]() {
        return computeScoreSlowly(baseScore);
    });
}

string joinUserAndScoreAsync(string userName, int baseScore) {
    future<int> scoreFuture = startScoreComputation(baseScore);

    // Do other work while async task runs.
    string prefix = "User: " + userName + ", Final score: ";

    int finalScore = scoreFuture.get();
    return prefix + to_string(finalScore);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    future<int> scoreFuture = startScoreComputation(21);
    cout << "Working while score computes...\n";
    cout << "Score: " << scoreFuture.get() << "\n";

    cout << joinUserAndScoreAsync("Nora", 21) << "\n";
    return 0;
}
#endif
