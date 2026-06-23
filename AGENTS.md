## Compiler

Let the compiler do the work.

## General Syntax Preference

Program in a C++ / C# style syntax, no matter the language.

## Aliases

Rust has a tendency to make overly compact names, such as `Box`, `Rc`, and
`Arc`. Make things clear instead of requiring callers to guess or consult
documentation.

## Iteration Style

I despise `map` and `iter`.

## Lifetimes

There is no reason to use explicit lifetimes unless required.

If explicit lifetimes are required, use a higher-standard naming convention
than `'a`.

Readability is a priority.

## Generics

When making generics, never just write `T`.

Use a higher-standard naming convention that makes the code easier to read.

## Error Handling

I’m not a huge fan of the overuse of `.unwrap()`.

A mistake is using exceptions as ordinary control-flow signals.

Exceptions should only be used when something is really exceptional.

## Failing Gracefully

Failing gracefully means:

The system does not pretend everything is fine, but it also does not explode
unpredictably.

## Macros

Add comment suggestions where something could maybe be simplified with:

- Declarative macros
- Procedural macros

## Methods and Constructors

In methods and constructors, prioritize:

- Good defaults
- Optional parameters

Sometimes different constructors are required.

## Rust API Design

Rust APIs often expose too much machinery.

Rust APIs often overexpose correctness machinery because the ecosystem
culturally rewards:

- Type-level precision
- Generic composability
- Zero-cost configurability

More than caller-side simplicity.

## Fluent APIs

There is a trend in software where fluent APIs / interfaces are misused.

They often make local reasoning and API comprehension worse.

A fluent API frequently requires reading docs to discover the correct chain.

Interestingly, Rust’s own standard library tends to avoid excessively fluent
designs.

Example:

```rust
ClientBuilder::new()
    .with_runtime(...)
    .with_connector(...)
    .with_tls(...)
    .with_retry_layer(...)
    .build()
```

## Preferred API Structure

A lot of experienced engineers eventually prefer APIs that separate:

### Configuration

```rust
let config = ServerConfig {
    host: "localhost".into(),
    port: 8080,
    tls: Some(tls),
};
```

### Construction

```rust
let server = Server::new(config);
```

### Execution

```rust
server.startAsync().await?;
```

This scales better as systems grow because the available operations at each
stage are obvious.

## Simpler Caller-Side APIs

A JavaScript-style API is more likely to say something like this.

Rust could use more of that mindset:

```rust
let client = Client::new(token);
let response = client.get("/users").await?;
```

Then advanced users get explicit escape hatches:

```rust
let options = ClientOptions {
    timeout: Duration::from_secs(10),
    retries: 5,
    transport: Transport::Custom(transport),
};

let client = Client::new(token, options);
```
