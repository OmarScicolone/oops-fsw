# OOPS Flight Software Parser

A Rust parser for serial packets from the Optical Operations Pointing System (OOPS), written as part of a flight software engineering challenge.

## What it does

Parses a binary stream of hex bytes (`data/fsw-output.txt`) sniffed from a shared communications bus and answers four questions:

1. How many valid packets are in the data?
2. Of the invalid packets, why are they invalid?
3. How many bytes are padding (not part of any packet)?
4. What is the checksum of a constructed telecommand packet?

## How to run

```
cargo run
```

## Project structure

- `src/main.rs` — entry point, prints answers to Q1–Q4
- `src/parser.rs` — scans the byte stream and classifies packets
- `src/packet.rs` — protocol constants and data structures
- `ASSUMPTIONS.md` — design decisions and ambiguities in the datasheet
- `data/` — raw hex byte stream to parse
- `docs/` — original challenge brief and datasheet
