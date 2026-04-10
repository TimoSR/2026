#include <cassert>
#include <future>

using namespace std;

#include "multithreading_abstraction_wrapper.cpp"

int main() {
    {
        SingleThreadTaskRunner runner;
        future<string> a = runner.submit([]() { return string("A"); });
        future<string> b = runner.submit([]() { return string("B"); });

        assert(a.get() == "A");
        assert(b.get() == "B");
        runner.shutdown();
    }

    return 0;
}
