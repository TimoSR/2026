#include "orders/order_service.hpp"
#include "payments/stripe_payment_processor.hpp"
#include <iostream>
#include <memory>

int main()
{
    auto processor = std::make_shared<app::payments::StripePaymentProcessor>("sk_test_123");
    app::orders::OrderService order_service(processor);

    const bool paid = order_service.checkout("order-1001", "acct-42", 1299);

    std::cout << "Source-divided checkout: " << (paid ? "success" : "failure") << '\n';
    return paid ? 0 : 1;
}