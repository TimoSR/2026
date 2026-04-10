#include <cassert>
#include <cmath>
#include <string>
#include <vector>

using namespace std;

#include "const_usage_tradeoffs.cpp"

int main() {
    vector<Product> products = {
        Product("Mouse", 25.0),
        Product("Keyboard", 75.0),
        Product("Monitor", 200.0)
    };

    double avg = averagePrice(products);
    assert(abs(avg - 100.0) < 0.000001);

    string label = labelFromPrefix("INV", 12);
    assert(label == "INV-12");

    int noisyResult = noisyConstStyleExample(4);
    int simpleResult = simpleReadableStyleExample(4);
    assert(noisyResult == 18);
    assert(simpleResult == 18);

    Product product("Desk", 120.0);
    product.setPrice(150.0);
    assert(abs(product.getPrice() - 150.0) < 0.000001);

    return 0;
}
