# request_queue_pubsub_patterns.cpp

## InDepth: Choosing the communication style

- request/response: use when caller needs immediate answer
- queue: use when work can happen later
- pub/sub: use when many systems react to one event

## InDepth: Practical architecture mix

Most systems use all three:
- API calls for user interactions
- queues for background workloads
- pub/sub for cross-service reactions
