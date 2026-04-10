# hashing_basics.cpp

## InDepth: Hashing use cases

Practical uses:
- quick lookup keys (`unordered_map`)
- change detection
- bucketing/sharding
- cache keys

## InDepth: Important boundary

`std::hash` is good for in-process hashing, but not a cryptographic hash.

For passwords/signatures/security use dedicated cryptographic algorithms (SHA-256, BLAKE3, Argon2, bcrypt, scrypt) via trusted libraries.
