#include <cassert>
#include <vector>

using namespace std;

#include "numbers_basics.cpp"

int main() {
    assert(cappedAge(-5) == 0);
    assert(cappedAge(24) == 24);
    assert(cappedAge(500) == 130);

    vector<int> values = {10, 20, 30};
    assert(sumValues(values) == 60);
    assert(averageValues(values) == 20.0);

    pair<int, int> minMax = minMaxValues({95, 88, 77, 100});
    assert(minMax.first == 77);
    assert(minMax.second == 100);

    pair<int, int> emptyMinMax = minMaxValues({});
    assert(emptyMinMax.first == 0);
    assert(emptyMinMax.second == 0);
    return 0;
}
