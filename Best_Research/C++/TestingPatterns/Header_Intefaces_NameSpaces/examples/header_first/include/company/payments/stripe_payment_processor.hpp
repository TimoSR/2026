#pragma once

#include "i_payment_processor.hpp"
#include <string>

namespace company::payments
{
    class StripePaymentProcessor final : public IPaymentProcessor
    {
        public:
            explicit StripePaymentProcessor(std::string api_key);

            bool charge(std::string_view account_id, int amount_cents) override;
            void refund(std::string_view transaction_id, int amount_cents) override;

        private:
            std::string api_key_;
    };
}
