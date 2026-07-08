# udp_like_datagrams.cpp

## InDepth: What this demonstrates

This is a UDP-like teaching format:
- source port
- destination port
- total length
- checksum
- payload

It is not full UDP/IP stack code, but the same data-shaping pattern appears in real networking.

## InDepth: Why checksum first

Parsing should validate integrity before trusting payload.

Flow:
1. read fixed header fields
2. verify total length
3. recompute checksum
4. only then expose parsed payload
