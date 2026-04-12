#pragma once

#include "../payments/i_payment_processor.hpp"
#include <memory>
#include <string>

namespace company::orders
{
    class OrderService
    {
        public:
            explicit OrderService(std::shared_ptr<company::payments::IPaymentProcessor> processor);

            bool checkout(std::string order_id, std::string account_id, int amount_cents);

        private:
            std::shared_ptr<company::payments::IPaymentProcessor> processor_;
    };
}
