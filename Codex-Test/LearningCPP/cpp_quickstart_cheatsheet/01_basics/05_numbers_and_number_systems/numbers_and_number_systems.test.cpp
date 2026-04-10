#include <cassert>
#include <cmath>

using namespace std;

#include "numbers_and_number_systems.cpp"

int main() {
    double average = calculateAverage(10.5, 14.5);
    assert(closeEnough(average, 12.5, 0.000001));

    long long yearly = yearlyRequestCount(120000LL);
    assert(yearly == 43800000LL);

    assert(areEquivalentNumberLiterals() == true);

    int decimalFromHex = parseAsDecimalFromHexLiteral();
    assert(decimalFromHex == 255);

    return 0;
}
