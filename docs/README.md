# esp32_memory_limit

Validate what happens in case of Out of Memory.

## Scenarios

### 01-alloc
- Allocation of memory  without check
- Run: `cargo run --release --example 01-alloc`
- Result: Panic with trace
```
INFO - Used memory: 32768; Free memory: 0


!! A panic occured in '.../.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs', at line 418, column 13

PanicInfo {
    payload: Any { .. },
    message: Some(
        memory allocation of 33792 bytes failed,
    ),
    location: Location {
        file: ".../.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs",
        line: 418,
        col: 13,
    },
    can_unwind: false,
    force_no_backtrace: false,
}
```

### 02-check-before-alloc
- Check free space before allocation
- Run: `cargo run --example 02-check-before-alloc`
- Result: Possibility to handle error
- Output message: `Not enough memory to allocate`

### 03-try-reserve
- Use try_reserve to acquire memory
- Run: `cargo run --example 03-try-reserve`
- Result: **WARN** Problem detected - memory is not returned back when variable leaves the scope
- Output message: `Not enough memory to allocate`

### 04-stack
- Allocated 1KB recursively
- run: `cargo run --example 04-stack`
- Result: **WARN** Problem detected - chip freezes when reaching `Depth: 219, Stack usage: 224256 bytes`
