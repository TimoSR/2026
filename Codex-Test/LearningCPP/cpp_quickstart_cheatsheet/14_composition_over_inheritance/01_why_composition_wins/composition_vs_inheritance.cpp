#include <string>

using namespace std;

// Inheritance style: behavior is fixed by base class hierarchy.
class InheritanceDiscountStrategy {
public:
    virtual ~InheritanceDiscountStrategy() = default;
    virtual double apply(double price) = 0;
};

class VipDiscountInheritance : public InheritanceDiscountStrategy {
public:
    double apply(double price) override {
        return price * 0.80;
    }
};

class CheckoutWithInheritance {
private:
    InheritanceDiscountStrategy* strategy;

public:
    CheckoutWithInheritance(InheritanceDiscountStrategy* strategyParam)
        : strategy(strategyParam) {
    }

    double finalPrice(double basePrice) {
        return strategy->apply(basePrice);
    }
};

// Composition style: object receives behavior component explicitly.
class IDiscountStrategy {
public:
    virtual ~IDiscountStrategy() = default;
    virtual double apply(double price) = 0;
};

class VipDiscount : public IDiscountStrategy {
public:
    double apply(double price) override {
        return price * 0.80;
    }
};

class NoDiscount : public IDiscountStrategy {
public:
    double apply(double price) override {
        return price;
    }
};

class CheckoutWithComposition {
private:
    IDiscountStrategy* discountStrategy;

public:
    CheckoutWithComposition(IDiscountStrategy* discountStrategyParam)
        : discountStrategy(discountStrategyParam) {
    }

    void setDiscountStrategy(IDiscountStrategy* discountStrategyParam) {
        discountStrategy = discountStrategyParam;
    }

    double finalPrice(double basePrice) {
        return discountStrategy->apply(basePrice);
    }
};

#ifdef RUN_DEMO
#include <iostream>

int main() {
    VipDiscount vipDiscount;
    NoDiscount noDiscount;

    CheckoutWithComposition checkout(&noDiscount);
    cout << "No discount: " << checkout.finalPrice(100.0) << "\n";

    checkout.setDiscountStrategy(&vipDiscount);
    cout << "VIP discount: " << checkout.finalPrice(100.0) << "\n";
    return 0;
}
#endif
