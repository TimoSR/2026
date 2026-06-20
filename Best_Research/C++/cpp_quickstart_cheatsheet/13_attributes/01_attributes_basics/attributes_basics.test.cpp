#include <cassert>
#include <string>
#include <vector>

using namespace std;

#include "attributes_basics.cpp"

int main() {
    string fullNameV2 = createDisplayNameV2("Nora", "Jensen");
    assert(fullNameV2 == "Jensen, Nora");

    int positiveSum = sumPositiveValues({3, -1, 8, 0, 2});
    assert(positiveSum == 13);

    string gradeA = categoryFromScore(95);
    string gradeB = categoryFromScore(85);
    string gradeF = categoryFromScore(50);
    assert(gradeA == "A");
    assert(gradeB == "B");
    assert(gradeF == "F");

    touchForLogging("test");
    return 0;
}
