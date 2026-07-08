#pragma once

#include <string_view>

namespace app::payments
{
    class IPaymentProcessor
    {
      public:
        virtual ~IPaymentProcessor() = default;

        virtual bool charge(std::string_view account_id, int amount_cents) = 0;
        virtual void refund(std::string_view transaction_id, int amount_cents) = 0;
    };
} // namespace app::payments