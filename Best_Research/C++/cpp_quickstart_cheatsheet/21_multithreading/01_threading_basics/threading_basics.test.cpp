#include <cassert>

using namespace std;

#include "threading_basics.cpp"

int main() {
    int mutexResult = runThreadedCounterWithMutex(4, 1000);
    assert(mutexResult == 4000);

    int atomicResult = runThreadedCounterWithAtomic(4, 1000);
    assert(atomicResult == 4000);

    return 0;
}
