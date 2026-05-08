# 0001. Use a BitCask Data Representation

Date: 2026-05-08

Status: Accepted

## Context

The current key-value store persists records in a single append-only text file. Each record is one line, using tab-delimited fields:

```text
set<TAB>key<TAB>value
delete<TAB>key
```

At startup, the store scans the file and rebuilds an in-memory `HashMap` from each live key to the byte offset of its latest record. Reads use the index to seek directly to the latest record, while inserts, updates, and deletes append new records.

This already has the basic shape of BitCask, but the current text representation has important limits:

- keys and values cannot contain tabs or newlines
- records do not include checksums, so corrupted or partial writes are difficult to detect
- the index only stores a record offset, so reads must parse the whole line to recover the value
- the file grows forever because overwritten values and delete records are never compacted
- future support for multiple data files would need more metadata than a single offset

## Decision

We will evolve the store toward a BitCask-style data representation.

Records will be append-only, length-prefixed entries containing enough metadata to locate and validate values without relying on line delimiters. The in-memory key directory will map each live key to the location and size of its latest value, rather than just a line offset.

A representative record layout for the first implementation is:

```text
kind | timestamp | key_size | value_size | key | value
```

Deletes will be represented by tombstone records. During startup, the store will scan records in file order and rebuild the key directory by keeping only the newest live entry for each key.

Checksums are intentionally deferred to a later change so the initial format migration can stay focused on length-prefixed records and key directory metadata.

## Consequences

This representation preserves the current store's strongest property: writes remain sequential appends. Inserts, updates, and deletes do not need to modify existing records in place.

Reads remain efficient. A lookup first checks the in-memory key directory, then seeks directly to the stored value. With value offset and value size in the key directory, the store can read only the value bytes it needs.

The store can support arbitrary byte content. Length-prefixed records do not reserve tabs or newlines as structural characters, so keys and values are no longer constrained by the current one-line text format.

The record format can detect incomplete records because each entry includes explicit key and value sizes. A future checksum will allow startup recovery to reject corrupted records rather than silently accepting malformed data.

The design creates a natural path to compaction. Older overwritten records and tombstones can be copied out or discarded during a merge process, reducing disk usage while preserving the latest value for each live key.

The tradeoff is additional implementation complexity. The store will need binary encoding and decoding, tombstone handling, and eventually checksum validation and compaction logic. It will also continue to require the full key directory to fit in memory.

## Alternatives Considered

### Keep the current text format

This is easy to inspect and simple to teach, but it prevents storing arbitrary data and gives weak protection against partial or corrupted writes.

### Use a structured text format

Formats such as JSON lines would improve extensibility, but they would still require escaping or encoding arbitrary bytes and would add parsing overhead without addressing compaction or direct value reads as cleanly.

### Use an embedded database engine

An existing engine would provide a more complete storage layer, but it would hide the storage mechanics this project is intended to explore.
