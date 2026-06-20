#include "stripe_payment_processor.hpp"
#include <iostream>
#include <utility>

namespace app::payments
{
    StripePaymentProcessor::StripePaymentProcessor(std::string api_key) : api_key_(std::move(api_key)) {}

    bool StripePaymentProcessor::charge(std::string_view account_id, int amount_cents)
    {
        const bool ok = !api_key_.empty() && !account_id.empty() && amount_cents > 0;
        
        std::cout
            << "[Stripe] charge account=" << account_id
            << " amount_cents=" << amount_cents
            << " result=" << (ok ? "ok" : "failed")
            << '\n';

        return ok;
    }

    void StripePaymentProcessor::refund(std::string_view transaction_id, int amount_cents)
    {
        std::cout
            << "[Stripe] refund tx=" << transaction_id
            << " amount_cents=" << amount_cents
            << '\n';
    }
}
