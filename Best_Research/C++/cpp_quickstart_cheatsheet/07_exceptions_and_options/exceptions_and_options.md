# Exceptions and Options

Two practical error-handling shapes in this section:

```cpp
// Exception style:
double result = divideNumbersOrThrow(10, 2);

// Optional style:
optional<double> result = tryDivideNumbers(10, 2);
```

## Pattern 1: Exceptions (`...OrThrow`)

Use when:
- failure is not part of normal control flow
- caller needs reason/details for logs or user messages
- you want a single success return type (`T`) and failure via `throw`

Example:
```cpp
double divideNumbersOrThrow(double firstNumber, double secondNumber) {
    if (secondNumber == 0.0) {
        throw invalid_argument("Cannot divide by zero");
    }
    return firstNumber / secondNumber;
}
```

## Pattern 2: Options (`try...` + `optional<T>`)

Use when:
- failure is expected and common
- caller only needs success/failure, not detailed reason
- you want branch-based flow without `try/catch`

Example:
```cpp
optional<double> tryDivideNumbers(double firstNumber, double secondNumber) {
    if (secondNumber == 0.0) {
        return nullopt;
    }
    return firstNumber / secondNumber;
}
```

## Choosing Quickly

- expected failure + no reason needed -> `optional`
- unexpected failure or reason needed -> `exceptions`

This section includes helper:
```cpp
string chooseErrorHandlingStyle(bool failureIsExpected, bool callerNeedsDetailedReason);
```

## Typical Scenarios

- Parsing user input in loops (`tryParsePort`) -> `optional`
- File I/O boundary (`readLinesFromFileOrThrow`) -> `exceptions`
- Business critical operation where failure details matter -> `exceptions`
- Fast validation pipeline where invalid cases are common -> `optional`

## InDepths

- `optional<T>` keeps the call site explicit (`has_value`, `value_or`) and avoids catch blocks.
- Exceptions preserve failure context better, especially for layered APIs.
- If you need no-exception flow and also rich error reasons, move from `optional<T>` to a result object (for example `struct { bool ok; string error; T value; }`).

