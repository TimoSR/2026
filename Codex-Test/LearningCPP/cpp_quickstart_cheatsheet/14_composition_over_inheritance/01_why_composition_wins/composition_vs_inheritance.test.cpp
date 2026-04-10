#include <cassert>
#include <cmath>

using namespace std;

#include "composition_vs_inheritance.cpp"

int main() {
    {
        VipDiscountInheritance vip;
        CheckoutWithInheritance checkout(&vip);
        double price = checkout.finalPrice(100.0);
        assert(abs(price - 80.0) < 0.000001);
    }

    {
        VipDiscount vip;
        NoDiscount noDiscount;
        CheckoutWithComposition checkout(&noDiscount);

        double normalPrice = checkout.finalPrice(100.0);
        assert(abs(normalPrice - 100.0) < 0.000001);

        checkout.setDiscountStrategy(&vip);
        double vipPrice = checkout.finalPrice(100.0);
        assert(abs(vipPrice - 80.0) < 0.000001);
    }

    return 0;
}
