# guid_basics.cpp

## InDepth: GUID vs hash

GUID/UUID is for unique identifiers, not content fingerprinting.

- GUID: identity value (`user_id`, `order_id`)
- hash: derived value from content (`cache key`, checksum)

## InDepth: Version and variant bits

v4 GUID includes fixed bit patterns:
- version nibble `4`
- variant nibble one of `8,9,a,b`

Setting those bits makes generated IDs follow standard UUID v4 format.
