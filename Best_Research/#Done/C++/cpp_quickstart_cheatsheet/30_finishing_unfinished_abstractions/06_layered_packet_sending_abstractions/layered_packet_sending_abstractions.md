# layered_packet_sending_abstractions.cpp

## InDepth: Layered abstraction flow

This example intentionally stacks abstractions:
1. domain model (`TelemetryMessage`)
2. serializer (`IMessageSerializer`)
3. packet builder (`IPacketBuilder`)
4. byte transport (`IByteTransportSender`)
5. facade (`TelemetrySenderFacade`)

Each layer has one responsibility and can be replaced independently.

## InDepth: Why this helps projects

- easier testing (fake each layer)
- easier protocol evolution (swap serializer or packet format)
- easier transport migration (socket, UDP, queue, etc.)
