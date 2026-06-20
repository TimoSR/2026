# scheduled_jobs_and_cron_style.cpp

## InDepth: Cron-like thinking

A scheduler repeatedly asks: "is this job due now?"

This example uses interval rules (`every N minutes`) because it is easy to read.  
Real cron adds day/month/week matching.

## InDepth: Practical split

- scheduler decides *when* to run
- job handler decides *what* to run

Keeping those separate simplifies testing and operations.
