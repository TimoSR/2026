# event_driven_patterns.cpp

## InDepth: Why event-driven communication

Events decouple producers from consumers.

`UserService` only emits `user_registered`.  
Email and analytics subscribe independently.

## InDepth: Common team pattern

- publish domain events in business services
- subscribe in integration services
- keep event payloads stable and versioned over time
