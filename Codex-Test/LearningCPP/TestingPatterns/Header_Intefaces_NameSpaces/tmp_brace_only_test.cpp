namespace app::payments {
OrderService::OrderService(std::shared_ptr<app::payments::IPaymentProcessor> processor)
        : processor_(std::move(processor)) {}

void f(){
    std::cout << "A" << x << "B" << y << '\n';
}
}