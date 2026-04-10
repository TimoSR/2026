#include <string>

using namespace std;

namespace billing {
    class TaxCalculator {
    public:
        int calculateTaxPercent(string countryCode) {
            if (countryCode == "DK") {
                return 25;
            }
            if (countryCode == "US") {
                return 8;
            }
            return 0;
        }
    };
}

namespace shipping {
    class TaxCalculator {
    public:
        int calculateTaxPercent(string countryCode) {
            // Shipping surcharge style tax, just for example.
            if (countryCode == "DK") {
                return 5;
            }
            if (countryCode == "US") {
                return 3;
            }
            return 1;
        }
    };
}

int totalTaxPercentForOrder(string countryCode) {
    billing::TaxCalculator billingTax;
    shipping::TaxCalculator shippingTax;
    return billingTax.calculateTaxPercent(countryCode) + shippingTax.calculateTaxPercent(countryCode);
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    cout << "[namespace_basics]\n";
    cout << "DK total tax: " << totalTaxPercentForOrder("DK") << "\n";
    cout << "US total tax: " << totalTaxPercentForOrder("US") << "\n";
    return 0;
}
#endif
