# socket_client_abstraction_wrapper.cpp

## InDepth: What got improved

Raw socket APIs expose bytes, framing, and parsing details at every call site.

`UserSocketClient` finishes abstraction by offering domain methods:
- `isHealthy()`
- `echo(message)`

Callers no longer manage wire protocol details directly.

## InDepth: Why transport interface helps

`IByteTransport` allows:
- fake transport in tests
- real socket transport in production
- no project-wide dependency on low-level API shape
