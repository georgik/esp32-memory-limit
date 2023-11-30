# esp32_memory_limit

Validate what happens in case of Out of Memory.

## Scenarios

### 01-alloc
- Allocation of memory  without check
- Run: `cargo run --example 01-alloc`
- Debug and release results: Panic with trace
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
- Debug and release result: Possibility to handle error
- Output message: `Not enough memory to allocate`

### 03-try-reserve
- Use try_reserve to acquire memory
- Run: `cargo run --example 03-try-reserve`
- Debug and release result: Possibility to handle error
- Output message: `Not enough memory to allocate`

### 04-stack
- Allocated 1KB recursively
- Run: `cargo run --example 04-stack`
- Debug result: **WARN** Problem detected - chip freezes, board remains responsive
```
Depth: 219, Stack usage: 224256 bytes
```
- Release result: chip freezes, board remains responsive
```
INFO - Depth: 3998, Stack usage: 4093952 bytes
```


### 05-4kb-stack-overflow-protection-1kb-alloc
- 4 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 05-4kb-stack-overflow-protection-1kb-alloc`
- Debug result:
```
INFO - Depth: 214, Stack usage: 219136 bytes
INFO - Depth: 215, Stack usage: 220160 bytes
ERROR -

Possible Stack Overflow Detected
INFO - PC = 0x420266ec
0x420266ec - core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
```
- Release result:
```
INFO - Depth: 3932, Stack usage: 4026368 bytes
INFO - Depth: ERROR -

Possible Stack Overflow Detected
INFO - PC = 0x420041a4
0x420041a4 - _ZN4core3fmt9Formatter12pad_integral12write_prefix17h93e2f5ddd6e48c4aE
```

### 06-2kb-stack-overflow-protection-1kb-alloc
- 2 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 06-2kb-stack-overflow-protection-1kb-alloc`
- Debug result:
```
INFO - Depth: 216, Stack usage: 221184 bytes
INFO - Depth: 217ERROR -

Possible Stack Overflow Detected


!! A panic occured in 'examples/06-4kb-stack-overflow-protection-512b-alloc.rs', at line 71, column 25
```
- Release result:
```
INFO - Depth: 3964, Stack usage: 4059136 bytes
INFO - Depth: ERROR -

Possible Stack Overflow Detected
INFO - PC = 0x420041a2
0x420041a2 - _ZN4core3fmt9Formatter12pad_integral12write_prefix17h93e2f5ddd6e48c4aE
```

### 07-1kb-stack-overflow-protection-1kb-alloc
- 1 kB stack overflow protection with 1 kB allocation
- Run `cargo run --example 07-1kb-stack-overflow-protection-1kb-alloc`
- Debug result:
```
INFO - Depth: 217, Stack usage: 222208 bytes
INFO - Depth: 218Exception 'Load access fault' mepc=0x40380d22, mtval=0x00000125
0x40380d22 - esp_hal_common::interrupt::riscv::vectored::handle_interrupt
```
- Release result:
```
INFO - Depth: 3980, Stack usage: 4075520 bytes
INFO - Depth: ERROR -

Possible Stack Overflow Detected


!! A panic occured in 'examples/07-1kb-stack-overflow-protection-1kb-alloc.rs', at line 71, column 25
```

### 08-alloc-stack
- Allocator + Stack recursion
- Run `cargo run --example 08-alloc-stack`
- Debug result: unresponsive board, requires Boot + Reset button for the next flash
```
INFO - Stack depth: 218, usage: 223232 bytes
INFO - Stack depth: 219, usage: 224256 bytes
```
- Release result: unresponsive board, requires Boot + Reset button for the next flash, it even closed the connection
```
INFO - Stack depth: 3996, usage: 4091904 bytes
Error:   × Broken pipe
```

### 09-alloc-stack-recursion
- Allocator recursion with Stack recursion
- Run `cargo run --example 09-alloc-stack-recursion`
- Debug result:
```
INFO - Depth: 38, Stack usage: 38912 bytes, Heap allocation: 1024, Memory - used: 38912; free: 165888
INFO - Depth: 39, Stack usage: 39936 bytes, Heap allocation: 1024, Memory - used: 39936; free: 1070074805


!! A panic occured in '/Users/georgik/.cargo/registry/src/index.crates.io-6f17d22bba15001f/esp-alloc-0.3.0/src/lib.rs', at line 77, column 18

PanicInfo {
    payload: Any { .. },
    message: Some(
        already borrowed: BorrowMutError,
```
- Release result:
```
INFO - Depth: 200, Stack usage: 204800 bytes, Heap allocation: 1024, Memory - used: 204800; free: 0


!! A panic occured in '/Users/georgik/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs', at line 418, column 13

PanicInfo {
    payload: Any { .. },
    message: Some(
        memory allocation of 1024 bytes failed,
```

### 10-alloc-stack-with-watchdog
- Allocator + Stack recursion + 10 seconds watchdog
- Run `cargo run --example 10-alloc-stack-with-watchdog`
- Debug result: unresponsive board for short period of time until watchdog restarts it
```
INFO - Stack depth: 219, usage: 224256 bytes
Error:   × Broken pipe
```
- Release result: unresponsive board for short period of time until watchdog restarts it
```
INFO - Stack depth: 3996, usage: 4091904 bytes
Error:   × Broken pipe
```

### 11-300kb-alloc-stack-recursion
- Allocator recursion with Stack recursion
- Run `cargo run --example 09-alloc-stack-recursion`
- Debug result:
```
INFO - Depth: 109, Stack usage: 111616 bytes, Heap allocation: 1024, Memory - used: 111616; free: 195584
INFO - Depth: 110, Stack usage: 112640 bytes, Heap allocation: 1024, Memory - used: 112640; free: 194560


!! A panic occured in '.../.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/alloc.rs', at line 418, column 13

PanicInfo {
    payload: Any { .. },
    message: Some(
        memory allocation of 1024 bytes failed,
```
- Release result:
```
INFO - Depth: 222, Stack usage: 227328 bytes, Heap allocation: 1024, Memory - used: 227328; free: 79872
Exception 'Load access fault' mepc=0x403801aa, mtval=0x00000000
0x403801aa - _start_trap_rust_hal
    at ??:??
0x00000000 - _max_hart_id
    at ??:??
TrapFrame
PC=0x403801aa         RA/x1=0x40380060      SP/x2=0x3fc81e6c      GP/x3=0x3fcb8ec0      TP/x4=0x00000000
0x403801aa - _start_trap_rust_hal
```

