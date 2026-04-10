using Payment.Abstractions;
using Payment.Exceptions;
using Payment.Models;

namespace Payment.Models
{
    public sealed record PaymentRequest(
        string CustomerId,
        decimal Amount,
        string Currency,
        string PaymentMethod,
        string Description);

    public sealed record PaymentResult(
        bool IsSuccess,
        string TransactionId,
        decimal OriginalAmount,
        decimal FeeAmount,
        decimal TotalChargedAmount,
        string Currency,
        string Receipt,
        string FailureReason)
    {
        public static PaymentResult Success(
            string transactionId,
            decimal originalAmount,
            decimal feeAmount,
            string currency,
            string receipt)
        {
            return new PaymentResult(
                IsSuccess: true,
                TransactionId: transactionId,
                OriginalAmount: originalAmount,
                FeeAmount: feeAmount,
                TotalChargedAmount: originalAmount + feeAmount,
                Currency: currency,
                Receipt: receipt,
                FailureReason: string.Empty);
        }

        public static PaymentResult Failure(
            decimal originalAmount,
            string currency,
            string failureReason)
        {
            return new PaymentResult(
                IsSuccess: false,
                TransactionId: string.Empty,
                OriginalAmount: originalAmount,
                FeeAmount: 0m,
                TotalChargedAmount: originalAmount,
                Currency: currency,
                Receipt: string.Empty,
                FailureReason: failureReason);
        }
    }

    public sealed record ChargeResult(
        bool IsSuccess,
        string TransactionId,
        string FailureReason)
    {
        public static ChargeResult Success(string transactionId)
        {
            return new ChargeResult(true, transactionId, string.Empty);
        }

        public static ChargeResult Failure(string failureReason)
        {
            return new ChargeResult(false, string.Empty, failureReason);
        }
    }

    public sealed record FraudDecision(bool IsAllowed, string Reason)
    {
        public static FraudDecision Allow()
        {
            return new FraudDecision(true, string.Empty);
        }

        public static FraudDecision Deny(string reason)
        {
            return new FraudDecision(false, reason);
        }
    }
}

namespace Payment.Abstractions
{
    public interface IFraudPolicy
    {
        ValueTask<FraudDecision> EvaluateAsync(
            PaymentRequest request,
            System.Threading.CancellationToken cancellationToken);
    }

    public interface IFeePolicy
    {
        decimal CalculateFee(PaymentRequest request);
    }

    public interface IRetryPolicy
    {
        ValueTask<T> ExecuteAsync<T>(
            System.Func<System.Threading.CancellationToken, ValueTask<T>> operation,
            System.Threading.CancellationToken cancellationToken);
    }

    public interface IReceiptPolicy
    {
        string FormatReceipt(
            PaymentRequest request,
            ChargeResult chargeResult,
            decimal feeAmount);
    }

    public interface IPaymentGateway
    {
        ValueTask<ChargeResult> ChargeAsync(
            PaymentRequest request,
            decimal totalAmount,
            System.Threading.CancellationToken cancellationToken);
    }
}

namespace Payment.Exceptions
{
    public sealed class TransientGatewayException : System.Exception
    {
        public TransientGatewayException(string message) : base(message)
        {
        }
    }
}

namespace Payment.Policies.Fraud
{
    public sealed class BasicFraudPolicy : IFraudPolicy
    {
        private static readonly System.Collections.Generic.HashSet<string> BlockedCustomers =
            new(System.StringComparer.OrdinalIgnoreCase)
            {
                "blocked-user",
                "fraudulent-account"
            };

        public ValueTask<FraudDecision> EvaluateAsync(
            PaymentRequest request,
            System.Threading.CancellationToken cancellationToken)
        {
            cancellationToken.ThrowIfCancellationRequested();

            if (BlockedCustomers.Contains(request.CustomerId))
            {
                return ValueTask.FromResult(
                    FraudDecision.Deny("Customer is blocked by fraud policy."));
            }

            if (request.Amount > 10_000m)
            {
                return ValueTask.FromResult(
                    FraudDecision.Deny("Amount exceeds manual review threshold."));
            }

            if (System.String.Equals(
                    request.PaymentMethod,
                    "crypto-prepaid",
                    System.StringComparison.OrdinalIgnoreCase))
            {
                return ValueTask.FromResult(
                    FraudDecision.Deny("Payment method is not allowed by fraud policy."));
            }

            return ValueTask.FromResult(FraudDecision.Allow());
        }
    }
}

namespace Payment.Policies.Fees
{
    public sealed class PercentageFeePolicy : IFeePolicy
    {
        private readonly decimal percentageRate;
        private readonly decimal fixedFee;

        public PercentageFeePolicy(decimal percentageRate, decimal fixedFee)
        {
            if (percentageRate < 0m)
            {
                throw new System.ArgumentOutOfRangeException(nameof(percentageRate));
            }

            if (fixedFee < 0m)
            {
                throw new System.ArgumentOutOfRangeException(nameof(fixedFee));
            }

            this.percentageRate = percentageRate;
            this.fixedFee = fixedFee;
        }

        public decimal CalculateFee(PaymentRequest request)
        {
            decimal percentageFee = decimal.Round(
                request.Amount * this.percentageRate,
                2,
                System.MidpointRounding.ToEven);

            decimal totalFee = percentageFee + this.fixedFee;

            return decimal.Round(totalFee, 2, System.MidpointRounding.ToEven);
        }
    }
}

namespace Payment.Policies.Retry
{
    public sealed class NoRetryPolicy : IRetryPolicy
    {
        public async ValueTask<T> ExecuteAsync<T>(
            System.Func<System.Threading.CancellationToken, ValueTask<T>> operation,
            System.Threading.CancellationToken cancellationToken)
        {
            if (operation is null)
            {
                throw new System.ArgumentNullException(nameof(operation));
            }

            return await operation(cancellationToken).ConfigureAwait(false);
        }
    }

    public sealed class ExponentialBackoffRetryPolicy : IRetryPolicy
    {
        private readonly int maxAttempts;
        private readonly System.TimeSpan initialDelay;
        private readonly System.Func<System.Exception, bool> shouldRetryOnException;

        public ExponentialBackoffRetryPolicy(
            int maxAttempts,
            System.TimeSpan initialDelay,
            System.Func<System.Exception, bool>? shouldRetryOnException = null)
        {
            if (maxAttempts <= 0)
            {
                throw new System.ArgumentOutOfRangeException(nameof(maxAttempts));
            }

            if (initialDelay < System.TimeSpan.Zero)
            {
                throw new System.ArgumentOutOfRangeException(nameof(initialDelay));
            }

            this.maxAttempts = maxAttempts;
            this.initialDelay = initialDelay;
            this.shouldRetryOnException = shouldRetryOnException ?? DefaultShouldRetry;
        }

        public async ValueTask<T> ExecuteAsync<T>(
            System.Func<System.Threading.CancellationToken, ValueTask<T>> operation,
            System.Threading.CancellationToken cancellationToken)
        {
            if (operation is null)
            {
                throw new System.ArgumentNullException(nameof(operation));
            }

            for (int attempt = 1; attempt <= this.maxAttempts; attempt++)
            {
                cancellationToken.ThrowIfCancellationRequested();

                try
                {
                    return await operation(cancellationToken).ConfigureAwait(false);
                }
                catch (System.Exception exception)
                    when (attempt < this.maxAttempts && this.shouldRetryOnException(exception))
                {
                    System.TimeSpan delay = CalculateDelay(attempt);
                    await System.Threading.Tasks.Task.Delay(delay, cancellationToken).ConfigureAwait(false);
                }
            }

            throw new System.InvalidOperationException(
                "Retry policy exhausted without returning or throwing as expected.");
        }

        private System.TimeSpan CalculateDelay(int attempt)
        {
            double multiplier = System.Math.Pow(2, attempt - 1);
            double milliseconds = this.initialDelay.TotalMilliseconds * multiplier;
            return System.TimeSpan.FromMilliseconds(milliseconds);
        }

        private static bool DefaultShouldRetry(System.Exception exception)
        {
            return exception is System.TimeoutException
                || exception is TransientGatewayException;
        }
    }
}

namespace Payment.Policies.Receipts
{
    public sealed class SimpleTextReceiptPolicy : IReceiptPolicy
    {
        public string FormatReceipt(
            PaymentRequest request,
            ChargeResult chargeResult,
            decimal feeAmount)
        {
            return string.Join(System.Environment.NewLine, new[]
            {
                "PAYMENT RECEIPT",
                $"Customer: {request.CustomerId}",
                $"Transaction: {chargeResult.TransactionId}",
                $"Description: {request.Description}",
                $"Payment Method: {request.PaymentMethod}",
                $"Amount: {request.Amount:0.00} {request.Currency}",
                $"Fee: {feeAmount:0.00} {request.Currency}",
                $"Total: {request.Amount + feeAmount:0.00} {request.Currency}",
                $"Processed At (UTC): {System.DateTime.UtcNow:O}"
            });
        }
    }

    public sealed class JsonReceiptPolicy : IReceiptPolicy
    {
        public string FormatReceipt(
            PaymentRequest request,
            ChargeResult chargeResult,
            decimal feeAmount)
        {
            string escapedDescription = EscapeJson(request.Description);
            string escapedPaymentMethod = EscapeJson(request.PaymentMethod);
            string escapedCustomerId = EscapeJson(request.CustomerId);
            string escapedCurrency = EscapeJson(request.Currency);
            string escapedTransactionId = EscapeJson(chargeResult.TransactionId);

            return
$@"{{
  ""customerId"": ""{escapedCustomerId}"",
  ""transactionId"": ""{escapedTransactionId}"",
  ""description"": ""{escapedDescription}"",
  ""paymentMethod"": ""{escapedPaymentMethod}"",
  ""amount"": {request.Amount:0.00},
  ""fee"": {feeAmount:0.00},
  ""total"": {request.Amount + feeAmount:0.00},
  ""currency"": ""{escapedCurrency}""
}}";
        }

        private static string EscapeJson(string value)
        {
            return value
                .Replace("\\", "\\\\", System.StringComparison.Ordinal)
                .Replace("\"", "\\\"", System.StringComparison.Ordinal)
                .Replace("\r", "\\r", System.StringComparison.Ordinal)
                .Replace("\n", "\\n", System.StringComparison.Ordinal)
                .Replace("\t", "\\t", System.StringComparison.Ordinal);
        }
    }
}

namespace Payment.Gateways
{
    public sealed class FakePaymentGateway : IPaymentGateway
    {
        private readonly System.Collections.Generic.Queue<System.Func<ChargeResult>> scriptedResponses;

        public FakePaymentGateway(
            System.Collections.Generic.IEnumerable<System.Func<ChargeResult>> scriptedResponses)
        {
            this.scriptedResponses =
                new System.Collections.Generic.Queue<System.Func<ChargeResult>>(scriptedResponses);
        }

        public ValueTask<ChargeResult> ChargeAsync(
            PaymentRequest request,
            decimal totalAmount,
            System.Threading.CancellationToken cancellationToken)
        {
            cancellationToken.ThrowIfCancellationRequested();

            if (this.scriptedResponses.Count == 0)
            {
                return ValueTask.FromResult(
                    ChargeResult.Success(System.Guid.NewGuid().ToString("N")));
            }

            System.Func<ChargeResult> next = this.scriptedResponses.Dequeue();

            try
            {
                ChargeResult result = next();
                return ValueTask.FromResult(result);
            }
            catch (System.Exception exception)
                when (exception is System.TimeoutException
                   || exception is TransientGatewayException)
            {
                throw;
            }
        }
    }
}

namespace Payment.Services
{
    public sealed class PaymentProcessor
    {
        private readonly IFraudPolicy fraudPolicy;
        private readonly IFeePolicy feePolicy;
        private readonly IRetryPolicy retryPolicy;
        private readonly IReceiptPolicy receiptPolicy;
        private readonly IPaymentGateway paymentGateway;

        public PaymentProcessor(
            IFraudPolicy fraudPolicy,
            IFeePolicy feePolicy,
            IRetryPolicy retryPolicy,
            IReceiptPolicy receiptPolicy,
            IPaymentGateway paymentGateway)
        {
            this.fraudPolicy = fraudPolicy ?? throw new System.ArgumentNullException(nameof(fraudPolicy));
            this.feePolicy = feePolicy ?? throw new System.ArgumentNullException(nameof(feePolicy));
            this.retryPolicy = retryPolicy ?? throw new System.ArgumentNullException(nameof(retryPolicy));
            this.receiptPolicy = receiptPolicy ?? throw new System.ArgumentNullException(nameof(receiptPolicy));
            this.paymentGateway = paymentGateway ?? throw new System.ArgumentNullException(nameof(paymentGateway));
        }

        public async ValueTask<PaymentResult> ProcessAsync(
            PaymentRequest request,
            System.Threading.CancellationToken cancellationToken = default)
        {
            ValidateRequest(request);

            FraudDecision fraudDecision =
                await this.fraudPolicy.EvaluateAsync(request, cancellationToken).ConfigureAwait(false);

            if (!fraudDecision.IsAllowed)
            {
                return PaymentResult.Failure(
                    request.Amount,
                    request.Currency,
                    fraudDecision.Reason);
            }

            decimal feeAmount = this.feePolicy.CalculateFee(request);
            decimal totalAmount = request.Amount + feeAmount;

            ChargeResult chargeResult = await this.retryPolicy.ExecuteAsync(
                async innerCancellationToken =>
                {
                    return await this.paymentGateway
                        .ChargeAsync(request, totalAmount, innerCancellationToken)
                        .ConfigureAwait(false);
                },
                cancellationToken).ConfigureAwait(false);

            if (!chargeResult.IsSuccess)
            {
                return PaymentResult.Failure(
                    request.Amount,
                    request.Currency,
                    chargeResult.FailureReason);
            }

            string receipt = this.receiptPolicy.FormatReceipt(request, chargeResult, feeAmount);

            return PaymentResult.Success(
                transactionId: chargeResult.TransactionId,
                originalAmount: request.Amount,
                feeAmount: feeAmount,
                currency: request.Currency,
                receipt: receipt);
        }

        private static void ValidateRequest(PaymentRequest request)
        {
            if (request is null)
            {
                throw new System.ArgumentNullException(nameof(request));
            }

            if (string.IsNullOrWhiteSpace(request.CustomerId))
            {
                throw new System.ArgumentException("CustomerId is required.", nameof(request));
            }

            if (request.Amount <= 0m)
            {
                throw new System.ArgumentOutOfRangeException(nameof(request), "Amount must be greater than zero.");
            }

            if (string.IsNullOrWhiteSpace(request.Currency))
            {
                throw new System.ArgumentException("Currency is required.", nameof(request));
            }

            if (string.IsNullOrWhiteSpace(request.PaymentMethod))
            {
                throw new System.ArgumentException("PaymentMethod is required.", nameof(request));
            }
        }
    }
}

namespace Demo
{
    public static class Program
    {
        public static async System.Threading.Tasks.Task Main()
        {   
            Payment.Models.PaymentRequest request = new(
                CustomerId: "customer-123",
                Amount: 1250.00m,
                Currency: "DKK",
                PaymentMethod: "card",
                Description: "Order #A-1024");

            Payment.Services.PaymentProcessor processor = new(
                fraudPolicy: new Payment.Policies.Fraud.BasicFraudPolicy(),
                feePolicy: new Payment.Policies.Fees.PercentageFeePolicy(percentageRate: 0.029m, fixedFee: 1.80m),
                retryPolicy: new Payment.Policies.Retry.ExponentialBackoffRetryPolicy(
                    maxAttempts: 3,
                    initialDelay: System.TimeSpan.FromMilliseconds(100)),
                receiptPolicy: new Payment.Policies.Receipts.SimpleTextReceiptPolicy(),
                paymentGateway: new Payment.Gateways.FakePaymentGateway(new System.Func<Payment.Models.ChargeResult>[]
                {
                    () => throw new System.TimeoutException("Gateway timeout."),
                    () => Payment.Models.ChargeResult.Success("tx-000001")
                }));

            Payment.Models.PaymentResult result = await processor.ProcessAsync(request);

            System.Console.WriteLine($"Success: {result.IsSuccess}");
            System.Console.WriteLine($"TransactionId: {result.TransactionId}");
            System.Console.WriteLine($"TotalCharged: {result.TotalChargedAmount:0.00} {result.Currency}");
            System.Console.WriteLine();
            System.Console.WriteLine(result.Receipt);
        }
    }
}