# minimal_http_server.cpp

## InDepth: Why this is single-threaded

The server handles one connection, processes it, responds, then closes it.

That means:
- very simple flow
- easy debugging
- limited concurrency

It is ideal for learning core request/response mechanics.

## InDepth: Production progression

Typical next steps after this pattern:
1. loop `accept` continuously
2. keep-alive support
3. multiple worker threads or async I/O
4. routing + middleware layers
