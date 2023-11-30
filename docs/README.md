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
- Result: Possibility to handle error
- Output message: `Not enough memory to allocate`

### 04-stack
- Allocated 1KB recursively
- Run: `cargo run --example 04-stack`
- Result: **WARN** Problem detected - chip freezes when reaching `Depth: 219, Stack usage: 224256 bytes`


### 05-4kb-stack-overflow-protection-1kb-alloc
- 4 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 05-4kb-stack-overflow-protection-1kb-alloc`
- Result:
```
INFO - Depth: 214, Stack usage: 219136 bytes
INFO - Depth: 215, Stack usage: 220160 bytes
ERROR -

Possible Stack Overflow Detected
INFO - PC = 0x420266ec
0x420266ec - core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
```

### 06-2kb-stack-overflow-protection-1kb-alloc
- 2 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 06-2kb-stack-overflow-protection-1kb-alloc.rs`
- Result:
```
INFO - Depth: 216, Stack usage: 221184 bytes
INFO - Depth: 217ERROR -

Possible Stack Overflow Detected


!! A panic occured in 'examples/06-4kb-stack-overflow-protection-512b-alloc.rs', at line 71, column 25
```

### 07-1kb-stack-overflow-protection-1kb-alloc
- 1 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 07-1kb-stack-overflow-protection-1kb-alloc`
- Result:
```
INFO - Depth: 217, Stack usage: 222208 bytes
INFO - Depth: 218Exception 'Load access fault' mepc=0x40380d22, mtval=0x00000125
0x40380d22 - esp_hal_common::interrupt::riscv::vectored::handle_interrupt
```
