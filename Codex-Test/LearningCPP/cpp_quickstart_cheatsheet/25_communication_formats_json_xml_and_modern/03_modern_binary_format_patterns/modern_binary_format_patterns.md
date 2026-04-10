# modern_binary_format_patterns.cpp

## InDepth: Modern alternatives to JSON/XML

Common modern options:
- MessagePack / CBOR: compact, schema-optional binary formats
- Protobuf / Avro: schema-driven compact formats

Typical reasons teams move:
- smaller payload size
- faster encode/decode
- stronger compatibility contracts with schemas

## InDepth: Selection hint

- need human-readable payloads: JSON
- need very compact payload + schema: Protobuf/Avro
- need compact payload + flexible schema-less flow: MessagePack/CBOR
