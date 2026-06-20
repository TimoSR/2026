#include <string>
#include <vector>

using namespace std;

class Product {
private:
    string name;
    double price;

public:
    Product(string nameValue, double priceValue)
        : name(nameValue), price(priceValue) {
    }

    string getName() const {
        return name;
    }

    double getPrice() const {
        return price;
    }

    void setPrice(double newPrice) {
        if (newPrice > 0.0) {
            price = newPrice;
        }
    }
};

double averagePrice(const vector<Product>& products) {
    if (products.empty()) {
        return 0.0;
    }

    double totalPrice = 0.0;
    for (const Product& product : products) {
        totalPrice += product.getPrice();
    }

    return totalPrice / products.size();
}

string labelFromPrefix(const string& prefix, int number) {
    // const reference avoids copy and guarantees no mutation.
    return prefix + "-" + to_string(number);
}

int noisyConstStyleExample(int value) {
    // This works, but can be overcomplicated if used everywhere blindly.
    const int firstStep = value + 2;
    const int secondStep = firstStep * 3;
    return secondStep;
}

int simpleReadableStyleExample(int value) {
    int step = value + 2;
    step = step * 3;
    return step;
}

#ifdef RUN_DEMO
#include <iostream>

int main() {
    using namespace std;

    vector<Product> products = {
        Product("Mouse", 25.0),
        Product("Keyboard", 75.0)
    };

    cout << averagePrice(products) << "\n";
    cout << labelFromPrefix("INV", 12) << "\n";
    cout << noisyConstStyleExample(4) << "\n";
    cout << simpleReadableStyleExample(4) << "\n";
    return 0;
}
#endif
