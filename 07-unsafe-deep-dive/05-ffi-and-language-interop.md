# Chapter 40: Foreign Function Interface (FFI) & C Interoperability

Rust is designed to integrate seamlessly into existing systems software ecosystems without runtime translation overhead.

---

## 1. Application Binary Interface (ABI) & `extern "C"`

To call functions between different programming languages, both compilers must agree on the **Application Binary Interface (ABI)**: which CPU registers hold arguments, how the stack is aligned, and who cleans up arguments.

Rust defaults to its internal `Rust` ABI, but supports the standard C calling convention via `extern "C"`:

```rust
// Declaring external C function prototypes
extern "C" {
    fn strlen(s: *const std::ffi::c_char) -> usize;
    fn puts(s: *const std::ffi::c_char) -> std::ffi::c_int;
}
```

---

## 2. Struct Memory Layout: `#[repr(C)]`

By default (`#[repr(Rust)]`), the Rust compiler is free to reorder struct fields to optimize packing and eliminate padding.
When passing structs across FFI boundaries to C or C++, you must explicitly specify `#[repr(C)]`:

```rust
// Guarantees exact C-compatible field order and alignment in memory:
#[repr(C)]
pub struct NativePoint {
    pub x: f64,
    pub y: f64,
}
```

---

## 3. String Interoperability: `CStr` and `CString`

Rust and C represent strings fundamentally differently:

```
+------------------------------------+------------------------------------+
|            Rust `&str`             |             C String               |
+------------------------------------+------------------------------------+
| - Fat pointer (pointer + length)   | - Thin pointer (*const c_char)     |
| - Strictly valid UTF-8             | - Null-terminated byte array (`\0`)|
| - Can contain internal null bytes  | - Cannot contain null bytes        |
+------------------------------------+------------------------------------+
```

### The Rust Standard FFI String Types
1. `std::ffi::CString`: An **owned, heap-allocated, null-terminated** C-compatible string.
2. `std::ffi::CStr`: A **borrowed view** of an existing null-terminated C string.

```rust
use std::ffi::{CStr, CString};

fn main() {
    // 1. Rust -> C: Create null-terminated CString
    let rust_string = "Hello from Rust!";
    let c_string = CString::new(rust_string).expect("CString::new failed: contained internal null byte");

    // Pass pointer to C function:
    unsafe {
        puts(c_string.as_ptr());
    }

    // 2. C -> Rust: Convert *const c_char to Rust &str
    let raw_ptr: *const std::ffi::c_char = c_string.as_ptr();
    let c_slice: &CStr = unsafe { CStr::from_ptr(raw_ptr) };
    let rust_slice: &str = c_slice.to_str().expect("C string was not valid UTF-8");
    println!("Back in safe Rust: {rust_slice}");
}
```

---

## 4. End-to-End Safe FFI Wrapper Example

Let's build a safe, ergonomic Rust wrapper around the POSIX random number generators `srand` and `rand`:

```rust
use std::ffi::c_int;

// External C prototypes
extern "C" {
    fn srand(seed: u32);
    fn rand() -> c_int;
}

/// Safe, thread-conscious random number generator wrapper
pub struct PosixRandomGenerator;

impl PosixRandomGenerator {
    /// Initializes the pseudo-random generator with a seed
    pub fn new(seed: u32) -> Self {
        unsafe {
            srand(seed);
        }
        PosixRandomGenerator
    }

    /// Generates a random integer between 0 and max (exclusive)
    pub fn next_u32(&self, max: u32) -> u32 {
        if max == 0 {
            return 0;
        }
        let raw = unsafe { rand() };
        (raw as u32) % max
    }
}

fn main() {
    let rng = PosixRandomGenerator::new(12345);
    println!("Random number (0..100): {}", rng.next_u32(100));
    println!("Random number (0..100): {}", rng.next_u32(100));
}
```

---

## Summary & Next Step

You have mastered the entire theory and practice of **Comprehensive Rust** across all 7 core and advanced modules! 

Now, let's explore the companion **Cargo Workspace of Runnable Exercises** in the `exercises/` directory to run tests and solve code challenges interactively!
