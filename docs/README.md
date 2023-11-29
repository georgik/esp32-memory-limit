# esp32_memory_limit

Validate what happens in case of Out of Memory.

## Results

Method 1: Alloc without check
- Result: Panic with trace

Method 2: Check free space before allocation
- Result: Possibility to handle error

