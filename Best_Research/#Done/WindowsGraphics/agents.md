# AGENTS.md

## Purpose

This file defines the coding, architecture, API-design, and refactoring rules for AI agents working in this repository.

The agent must optimize for production-quality code, explicit architecture, readable staged operations, strong domain naming, and maintainable modular systems.

Do not optimize for minimal examples, academic style, cleverness, or overly idiomatic code when that reduces clarity.

---

## Core Operating Rules

1. Produce complete, production-ready code.
2. Do not use placeholder comments such as:

   * `// existing logic`
   * `// TODO: implement`
   * `// ... rest of code`
   * `// your validation here`
3. Preserve and build on existing code whenever code is provided.
4. Do not replace user-provided architecture unless there is a clear correctness, safety, or maintainability reason.
5. Prefer explicit, staged operations over compact chained expressions.
6. Prefer readable domain language over generic technical shorthand.
7. Prefer bottom-up modular architecture over large framework-first designs.
8. Keep abstractions inspectable, composable, and justified by product needs.
9. Let the compiler do its job before adding unnecessary annotations, lifetimes, wrappers, or complexity.
10. Tests belong close to the feature module and should cover both public behavior and important internal behavior.
11. Before changing code in a module, read the Markdown documentation that belongs to that module. If no module-specific Markdown file exists, read the nearest parent-level documentation before making the change.

---

## Architecture Philosophy

Use a bottom-up modular architecture.

Build systems in this order:

```text
Primitives
  → Capabilities
    → Tools
      → Systems
        → Human-friendly APIs
          → Product
```

Start with low-level primitives. Compose them into reusable capabilities. Turn those capabilities into tools. Compose tools into systems. Expose those systems through simple, human-friendly APIs.

The goal is not to avoid abstraction.

The goal is to build the right abstraction at the right layer.

Each layer must have clear ownership of its complexity:

* Primitives provide raw mechanisms.
* Capabilities provide foundational behavior.
* Tools make repeated work reusable.
* Systems coordinate behavior.
* APIs make product development fast.
* Product code should not directly depend on low-level machinery unless there is a specific reason.

Do not create a large batteries-included framework by default.

Do not force product code into assumptions that do not come from the product’s actual requirements.

---

## Code Organization

Prefer module-first organization.

A module should usually be structured like this:

```text
module
{
    imports

    public types

    private types

    semantic type aliases

    domain constants

    predicate/delegate constants

    public API

    internal implementation

    tests
}
```

Use labeled sections where they improve readability.

Example section labels:

```rust
// public types

// private types

// domain constants

// private domain language

// public API

// internal implementation

// tests
```

Do not over-fragment modules into many files too early. Keep highly related domain logic together until the boundaries are obvious.

---

## Syntax and Formatting Preferences

Prefer C++ / C# style readability where the language allows it.

Use Allman braces when formatting is under our control:

```csharp
if (condition)
{
    DoWork();
}
```

Prefer explicit `return` when it improves readability or matches the surrounding style.

Avoid compressed one-liners when they hide meaningful intermediate state.

Prefer this:

```rust
let remainder = grade % ROUNDING_BASE;
let delta = ROUNDING_BASE - remainder;

if delta < ROUNDING_DELTA_LIMIT
{
    result += delta;
}

return result;
```

Over this:

```rust
result + ((5 - result % 5) % 5)
```

---

## Naming Rules

Use clear, correct, technical names.

Avoid overly compact names when they require the reader to infer meaning.

Do not use generic names like:

```rust
T
U
V
x
y
data
item
thing
```

Use names like:

```rust
TRequest
TResponse
TError
TState
TMessage
UserRequest
ParsedCommand
RenderPipelineId
```

Use semantic type aliases when they improve domain readability:

```rust
type Grade = i32;
type RoundedGrades = Vec<Grade>;
type Remainder = Grade;
type Delta = Grade;
type GradeRule = fn(Grade) -> bool;
```

Domain constants should be named explicitly:

```rust
const MIN_PASSING_GRADE: Grade = 38;
const ROUNDING_BASE: Grade = 5;
const ROUNDING_DELTA_LIMIT: Delta = 3;
```

Predicate names should read like domain language:

```rust
const IS_FAILING: GradeRule = |grade: Grade| grade < MIN_PASSING_GRADE;
const IS_PASSING: GradeRule = |grade: Grade| !IS_FAILING(grade);
const IS_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| grade % ROUNDING_BASE == 0;
const IS_NOT_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| !IS_MULTIPLE_OF_FIVE(grade);
```

---

## Iteration Style

Prefer `for` loops over iterator chains.

Avoid unnecessary use of:

```rust
.iter()
.map()
.filter()
.fold()
.collect()
```

Use imperative loops when intermediate state matters, when debugging clarity matters, or when the business logic is easier to read step by step.

Prefer this:

```rust
let mut rounded_grades = Vec::new();

for grade in grades
{
    let mut result = *grade;

    if GRADE_SHOULD_BE_ROUNDED(result)
    {
        let remainder: Remainder = result % ROUNDING_BASE;
        let delta: Delta = ROUNDING_BASE - remainder;

        if delta < ROUNDING_DELTA_LIMIT
        {
            result += delta;
        }
    }

    rounded_grades.push(result);
}

return rounded_grades;
```

Over this:

```rust
grades.iter().map(|grade| round_grade(*grade)).collect()
```

Iterator chains are allowed only when they are clearly simpler, do not hide important intermediate state, and improve readability.

---

## Rust-Specific Rules

### Compiler First

Let the Rust compiler do its job.

Do not add explicit lifetimes, type annotations, clones, boxes, reference counting, or unsafe code unless the compiler or architecture requires it.

Write the simple version first. Add precision only when necessary.

### Lifetimes

Do not use explicit lifetimes unless required.

If explicit lifetimes are required, do not use names like:

```rust
'a
'b
'c
```

Use meaningful names:

```rust
'request
'response
'context
'resource
```

### Generics

Do not use single-letter generic names unless the type is genuinely mathematical or conventional in a tiny local scope.

Prefer:

```rust
struct Repository<TRecord, TRecordId>
{
    records: Vec<TRecord>,
}
```

Over:

```rust
struct Repository<T, U>
{
    records: Vec<T>,
}
```

### Strings

Do not force callers to care whether an API stores, borrows, clones, or allocates a string.

Avoid APIs that require this from callers:

```rust
Printer::new(String::from("Hello"));
```

Prefer accepting natural caller input:

```rust
let printer = Printer::new("Hello");

let message = "Hello".to_string();
let printer = Printer::new(message);
```

Use `impl Into<String>`, `impl AsRef<str>`, or equivalent patterns when they improve caller-side ergonomics without making the API harder to understand.

### Ownership

Make ownership boundaries explicit in implementation, but do not leak internal ownership machinery into the public API without cause.

Avoid exposing types like `Box`, `Rc`, `Arc`, `Cow`, or complex generic wrappers in public APIs unless they are part of the user’s actual mental model.

Use clear domain names around ownership concepts when wrappers are necessary.

### Error Handling

Do not use `unwrap()` in production code.

Avoid:

```rust
let value = result.unwrap();
```

Prefer:

```rust
let value = result?;
```

Or map the error into a domain-specific error:

```rust
let value = result.map_err(ApplicationError::from)?;
```

Use `panic!` only for true invariant violations or unrecoverable programmer errors.

Do not use `panic!` for ordinary control flow, validation failure, parsing failure, missing input, user error, network failure, file-system failure, or recoverable application state.

Fail gracefully:

* Do not pretend everything is fine.
* Do not crash unpredictably.
* Return explicit errors.
* Preserve useful diagnostic context.
* Keep the system in a valid state.

### Macros

Do not introduce macros as a first solution.

When repeated code might later benefit from a macro, add a comment suggesting the possible macro direction only when useful.

Acceptable comment:

```rust
// This repetition may become a declarative macro if additional grade policies are added.
```

Do not use procedural macros unless the complexity is clearly justified.

---

## C#-Specific Rules

Use clear domain-oriented C#.

Prefer explicit classes, methods, delegates, and staged operations.

Use delegates and predicates to express reusable domain rules.

Prefer:

```csharp
private static readonly Func<int, bool> IsFailing = grade => grade < MinimumPassingGrade;
private static readonly Func<int, bool> IsPassing = grade => !IsFailing(grade);
private static readonly Func<int, bool> GradeShouldBeRounded = grade => IsPassing(grade) && IsNotMultipleOfFive(grade);
```

Over repeated inline conditionals.

Do not use exceptions as ordinary control flow.

Use exceptions for exceptional conditions, invalid programmer assumptions, or unrecoverable states.

For expected failure, prefer explicit result types, validation results, nullable handling, or domain-specific error values where appropriate.

Remember that passing a `List<T>` into a method or constructor does not move ownership. It passes a copy of the reference to the object on the managed heap.

When mutation is not intended, prefer read-only abstractions:

```csharp
IReadOnlyList<T>
IReadOnlyCollection<T>
IEnumerable<T>
```

Use mutable collections only when mutation is part of the API contract.

---

## API Design Rules

### Prefer Human-Friendly APIs

Public APIs should be designed around how callers naturally provide data.

The caller should not need to understand internal storage, allocation, ownership, retry layers, runtimes, or configuration machinery for basic use.

Prefer simple defaults:

```rust
let client = Client::new(token);
let response = client.get("/users").await?;
```

Then provide explicit escape hatches for advanced users:

```rust
let options = ClientOptions {
    timeout: Duration::from_secs(10),
    retries: 5,
    transport: Transport::Custom(transport),
};

let client = Client::new_with_options(token, options);
```

### Separate Configuration, Construction, and Execution

Prefer APIs that separate stages:

```rust
let config = ServerConfig {
    host: "localhost".into(),
    port: 8080,
    tls: Some(tls),
};

let server = Server::new(config);

server.start_async().await?;
```

This makes available operations obvious at each stage.

Avoid APIs that require discovering a long fluent chain through documentation.

### Avoid Overused Fluent APIs

Do not use builder or fluent APIs by default.

Avoid this shape unless it is clearly justified:

```rust
ClientBuilder::new()
    .with_runtime(runtime)
    .with_connector(connector)
    .with_tls(tls)
    .with_retry_layer(retry_layer)
    .build()
```

Prefer explicit configuration structs when the system has meaningful configuration.

Builders are acceptable when:

* many fields are optional,
* construction has staged validation,
* the type would otherwise require many overloaded constructors,
* the builder materially improves correctness or usability.

### Constructors and Defaults

Prioritize good defaults.

Use optional parameters, overloads, or configuration structs depending on the language.

Provide multiple constructors only when they represent meaningfully different construction paths.

Do not make the simplest caller path pay for advanced configuration complexity.

---

## Domain Rule Composition

Prefer composing small named rules over repeating procedural condition checks.

Example:

```rust
type Grade = i32;
type GradeRule = fn(Grade) -> bool;
type Delta = Grade;
type Remainder = Grade;

const MIN_PASSING_GRADE: Grade = 38;
const ROUNDING_BASE: Grade = 5;
const ROUNDING_DELTA_LIMIT: Delta = 3;

const IS_FAILING: GradeRule = |grade: Grade| grade < MIN_PASSING_GRADE;
const IS_PASSING: GradeRule = |grade: Grade| !IS_FAILING(grade);
const IS_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| grade % ROUNDING_BASE == 0;
const IS_NOT_MULTIPLE_OF_FIVE: GradeRule = |grade: Grade| !IS_MULTIPLE_OF_FIVE(grade);
const GRADE_SHOULD_BE_ROUNDED: GradeRule = |grade: Grade| IS_PASSING(grade) && IS_NOT_MULTIPLE_OF_FIVE(grade);
```

This style is preferred because it gives:

* clearer intent,
* smaller units of logic,
* better testability,
* easier changes,
* less duplicated conditional logic,
* a path toward policy-driven design.

---

## Testing Rules

Tests should live inside the feature module when possible.

Test public behavior.

Also test important internal behavior when internal domain rules carry meaningful logic.

Rust example structure:

```rust
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn public_api_rounds_grades_correctly()
    {
        let grades = vec![73, 67, 38, 33];

        let rounded_grades = GradingStudents::grading_students(&grades);

        assert_eq!(rounded_grades, vec![75, 67, 40, 33]);
    }

    #[test]
    fn failing_rule_works()
    {
        assert!(IS_FAILING(37));
        assert!(!IS_FAILING(38));
    }

    #[test]
    fn passing_rule_is_composed_from_failing_rule()
    {
        assert!(!IS_PASSING(37));
        assert!(IS_PASSING(38));
    }
}
```

Tests should verify:

* public API behavior,
* domain constants,
* predicate/delegate rules,
* important private/internal implementation behavior,
* boundary cases,
* error cases,
* graceful failure behavior.

Do not test only the happy path.

---

## Refactoring Rules

When refactoring existing code:

1. Preserve behavior first.
2. Add or update tests before structural changes when possible.
3. Extract semantic type aliases before introducing larger abstractions.
4. Extract domain constants before extracting services.
5. Extract predicate/delegate rules before duplicating conditionals.
6. Prefer module-level private helpers when methods would require unnecessary `self`.
7. Avoid introducing traits, interfaces, generics, async runtimes, dependency injection, builders, or macros unless they solve a real problem.
8. Keep the public API simpler than the internal implementation.
9. Do not expose machinery just because the implementation uses it.

---

## Agent Response Rules

When generating code:

* Provide full code.
* Do not omit imports.
* Do not omit tests.
* Do not omit error types.
* Do not omit configuration structs.
* Do not use placeholder comments.
* Do not collapse important logic into pseudocode.
* Do not discard user-provided code.
* Explain architectural tradeoffs when changing structure.
* Prefer concrete, production-ready implementation over abstract explanation.

When reviewing code:

* Identify correctness risks first.
* Then identify architecture risks.
* Then identify API ergonomics issues.
* Then identify style violations.
* Then provide the corrected code.

When uncertain:

* Make the smallest reasonable assumption.
* State the assumption.
* Continue with a complete best-effort implementation.

---

## Anti-Patterns

Avoid:

```rust
.unwrap()
.expect("works")
panic!("normal validation failed")
.map(...).filter(...).collect()
T
'a
String::from(...) required at call sites
large builder chains
framework-first architecture
public APIs exposing internal ownership machinery
placeholder comments
pseudo-implementations
```

Avoid:

```csharp
throw new Exception()
catch (Exception)
fluent chains for ordinary configuration
mutable List<T> parameters when read-only access is enough
repeated inline boolean logic
```

Prefer:

```text
explicit staged operations
semantic type aliases
domain constants
named predicate/delegate rules
for loops
clear ownership boundaries
configuration structs
simple default constructors
advanced escape hatches
feature-local tests
graceful failure
```

---

## Final Standard

The final code should feel explicit, composable, inspectable, and adaptable.

Low-level details should exist where they belong.

Product-facing APIs should remain simple.

The agent should produce code that an experienced engineer can maintain, extend, debug, and ship.
