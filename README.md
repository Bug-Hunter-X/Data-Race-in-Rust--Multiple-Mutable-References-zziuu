# Data Race Bug in Rust

This repository demonstrates a common error in Rust: creating a data race by having multiple mutable references to the same variable.  This is undefined behavior and can lead to unpredictable program crashes or incorrect results. The `bug.rs` file showcases this error, while `bugSolution.rs` offers a corrected version.

## How to reproduce

1. Clone the repository.
2. Run `rustc bug.rs` and then `./bug`.
3. Observe the unpredictable output, which varies depending on compiler and runtime optimization decisions (it might print 7, 6, or something else entirely).

## Solution

The solution involves using techniques like cloning, interior mutability (e.g., using `RefCell` or `Mutex`), or refactoring to avoid mutable borrows simultaneously.
