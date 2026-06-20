# redis_style_patterns.cpp

## InDepth: Why Redis appears everywhere

Redis-style usage patterns:
- cache hot values (`SET`, `GET`)
- counters and rate limits (`INCR`)
- expiring keys (`EXPIRE`)
- pub/sub notifications (`PUBLISH`, `SUBSCRIBE`)

## InDepth: Production note

This file is an in-memory teaching model, not a real Redis client.  
In production, use an actual Redis server + C++ client library and keep command patterns the same.
