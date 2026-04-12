# C++ API Layout Comparison

This directory contains two equivalent examples showing different organization styles.

## 1) Header-first (`examples/header_first`)

- Public API headers live under `include/company/...`
- Implementations live under `src/...`
- Typical for reusable libraries consumed by other targets

Build and run:

```powershell
cmake -S examples/header_first -B examples/header_first/build
cmake --build examples/header_first/build
.\examples\header_first\build\header_first_demo.exe
```

If `cmake` is unavailable:

```powershell
& "C:\Program Files\LLVM\bin\clang++.exe" -std=c++20 `
  -Iexamples/header_first/include `
  examples/header_first/src/main.cpp `
  examples/header_first/src/orders/order_service.cpp `
  examples/header_first/src/payments/stripe_payment_processor.cpp `
  -o examples/header_first/header_first_demo.exe
.\examples\header_first\header_first_demo.exe
```

## 2) Source-divided (`examples/source_divided`)

- Headers and implementations are grouped by feature under `src/`
- Two folders demonstrate bounded contexts: `orders` and `payments`
- Typical for small apps/internal services where API export is not the primary concern

Build and run:

```powershell
cmake -S examples/source_divided -B examples/source_divided/build
cmake --build examples/source_divided/build
.\examples\source_divided\build\source_divided_demo.exe
```

If `cmake` is unavailable:

```powershell
& "C:\Program Files\LLVM\bin\clang++.exe" -std=c++20 `
  -Iexamples/source_divided/src `
  examples/source_divided/src/main.cpp `
  examples/source_divided/src/orders/order_service.cpp `
  examples/source_divided/src/payments/stripe_payment_processor.cpp `
  -o examples/source_divided/source_divided_demo.exe
.\examples\source_divided\source_divided_demo.exe
```

Both examples use:

- header declarations
- namespaces
- abstract interface (`IPaymentProcessor`)
- concrete implementation (`StripePaymentProcessor`)
