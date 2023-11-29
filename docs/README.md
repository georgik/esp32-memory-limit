# esp32_memory_limit

Validate what happens in case of Out of Memory.

## Scenarios

Method 1: Alloc without check
- Run: `cargo run --release --example 01-alloc`
- Result: Panic with trace

- Run: `cargo run --example 01-alloc`
- Result: 

Method 2: Check free space before allocation
- Run: `cargo run --example 02-check-before-alloc`
- Result: Possibility to handle error
- Output message: `Not enough memory to allocate`

