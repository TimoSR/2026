#include <cassert>

using namespace std;

#include "namespace_basics.cpp"

int main() {
    assert(totalTaxPercentForOrder("DK") == 30);
    assert(totalTaxPercentForOrder("US") == 11);
    assert(totalTaxPercentForOrder("NO") == 1);
    return 0;
}
