#include "order_service.hpp"
#include <utility>

namespace app::orders
{
    OrderService::OrderService(std::shared_ptr<app::payments::IPaymentProcessor> processor) : processor_(std::move(processor)) {}

    bool OrderService::checkout(std::string order_id, std::string account_id, int amount_cents)
    {
        if (order_id.empty() || !processor_)
        {
            return false;
        }

        return processor_->charge(account_id, amount_cents);
    }
}
