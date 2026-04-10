#include <cassert>
#include <map>
#include <tuple>
#include <vector>

using namespace std;

#include "interview_data_tasks.cpp"

int main() {
    map<string, int> frequency = frequencyByWord({"cpp", "docs", "cpp", " interview "});
    assert(frequency["cpp"] == 2);
    assert(frequency["docs"] == 1);
    assert(frequency["interview"] == 1);

    vector<int> filtered = valuesAbove({10, 30, 5, 42}, 20);
    assert(filtered.size() == 2);
    assert(filtered[0] == 30);
    assert(filtered[1] == 42);

    pair<int, int> evenOdd = evenOddCount({1, 2, 3, 4, 5, 6});
    assert(evenOdd.first == 3);
    assert(evenOdd.second == 3);

    tuple<int, int, int> top3 = top3OrZero({42, 7, 99, 18, 56});
    assert(get<0>(top3) == 99);
    assert(get<1>(top3) == 56);
    assert(get<2>(top3) == 42);
    return 0;
}
