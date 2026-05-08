# 0002. Use Parity Bit Checking

Date: 2026-05-08

Status: Accepted

## Context

A well-worn path to overcome disk corruption is to use a technique called a checksum.

Before data is written to disk, a checking function is applied to those bytes. The result of that checking function, the checksum, is written alongside the original data. No checksum is calculated for the bytes of the checksum. If something breaks while writing the checksum's own bytes to disk, this will be noticed later as an error.

When data is read from disk, the data and the saved checksum are read together. The checking function is applied to the data again, and the newly calculated checksum is compared with the saved checksum. If the two results do not match, an error has occurred and the data should be considered corrupted.

The checking function should:

- return the same result for the same input
- return a different result for different inputs as often as possible
- be fast
- be easy to implement

This project is an educational key-value store. The immediate goal is to make record corruption visible while keeping the record format and implementation easy to inspect.

## Decision

We will add parity bit checking to each persisted record.

Each record will store one checksum byte before the existing record fields:

```text
parity | kind | timestamp | key_size | value_size | key | value
```

The parity byte stores even parity for every byte after the checksum byte. The checksum byte itself is excluded from the calculation. On decode, the store recalculates parity for the record data and rejects the record if the saved parity does not match.

## Alternatives Considered

| Option | Example | Strengths | Weaknesses for this use case |
| --- | --- | --- | --- |
| Parity bit check | one even-parity byte over the record payload | Very small, fast, dependency-free, and easy to explain | Detects only odd numbers of flipped bits; not suitable for strong corruption detection |
| CRC32 | `crc32fast` style 32-bit cyclic redundancy check | Fast and much better at detecting common accidental corruption | Adds format width and implementation complexity beyond the first checksum lesson |
| Cryptographic hash | SHA-256 | Strong collision resistance and tamper evidence | Slower, larger on disk, and solves an adversarial integrity problem this store does not have |

We chose parity bit checking because it is the smallest checksum that demonstrates the full write-and-verify workflow without hiding the mechanics behind a library.

## Consequences

The store can now detect records whose bytes no longer match the saved parity value. Startup will reject corrupted records while rebuilding the key directory.

The record format changes by one byte per record. Value offsets shift by one byte because values now start after the parity byte and the existing record header.

Parity checking is intentionally weak. It will not detect every corruption pattern, especially an even number of bit flips that preserve parity. A later ADR can replace the parity byte with CRC32 when stronger accidental corruption detection becomes the next learning goal.
