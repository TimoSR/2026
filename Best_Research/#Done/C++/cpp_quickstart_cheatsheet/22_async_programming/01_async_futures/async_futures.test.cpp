#include <cassert>
#include <future>
#include <string>

using namespace std;

#include "async_futures.cpp"

int main() {
    future<int> scoreFuture = startScoreComputation(21);
    int score = scoreFuture.get();
    assert(score == 42);

    string message = joinUserAndScoreAsync("Nora", 21);
    assert(message == "User: Nora, Final score: 42");

    return 0;
}
