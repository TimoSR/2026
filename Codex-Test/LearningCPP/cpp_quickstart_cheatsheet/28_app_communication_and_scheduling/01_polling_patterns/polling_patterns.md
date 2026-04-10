# polling_patterns.cpp

## InDepth: When polling is useful

Polling is simple and predictable for:
- background job status checks
- external APIs without webhooks
- dashboards refreshing state

## InDepth: Tradeoff

Too frequent polling increases load and latency cost.  
Common pattern: backoff over time (`1s`, `2s`, `5s`, ...).
