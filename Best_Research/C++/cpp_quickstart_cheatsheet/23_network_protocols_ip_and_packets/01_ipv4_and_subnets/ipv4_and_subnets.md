# ipv4_and_subnets.cpp

## InDepth: Why integer form matters

Network protocols treat IPv4 as 32-bit numbers.  
String form (`192.168.1.10`) is only for humans.

Pattern in code:
- parse string into 32-bit value
- apply subnet mask
- compare masked values

## InDepth: Prefix basics

`/24` means first 24 bits are network bits.  
`/16` means first 16 bits are network bits.

Same subnet check:
`(ipA & mask) == (ipB & mask)`
