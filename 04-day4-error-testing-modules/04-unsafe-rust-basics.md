# Chapter 22: Unsafe Rust Basics & Safe FFI Wrappers

Rust's safe subset provides ironclad guarantees against memory corruption. However, computers are inherently unsafe at the lowest level: hardware registers, operating system system calls, and C libraries do not conform to Rust's borrow checker. **Unsafe Rust** provides an escape hatch to interface directly with hardware.

---

## 1. What is Unsafe Rust?

The `unsafe` keyword does **not** turn off the borrow checker or disable type checking. It merely unlocks **five specific superpowers** that the compiler cannot statically verify:

```
                            The 5 Unsafe Superpowers
                                       |
    +-----------------+----------------+----------------+-----------------+
    |                 |                |                |                 |
1. Dereference    2. Call unsafe    3. Implement     4. Mutate static  5. Access
   raw pointers      functions or      unsafe traits    variables         union
  (*const, *mut)     FFI symbols      (Send, Sync)     (`static mut`)     fields
```

### The Unsafe Philosophy: Safe Abstractions
Safe code should be built *on top of* unsafe primitives. Types like `Vec<T>`, `String`, `Box<T>`, and `Mutex<T>` are implemented using unsafe code internally, but expose a **100% safe, sound public API**.

---

## 2. Raw Pointers (`*const T` and `*mut T`)

Raw pointers are analogous to C pointers:

```rust
fn main() {
    let mut num = 42;

    // Creating raw pointers is completely SAFE:
    let r1: *const i32 = &num as *const i32;
    let r2: *mut i32 = &mut num as *mut i32;

    // Dereferencing raw pointers requires UNSAFE:
    unsafe {
        println!("r1 points to: {}", *r1);
        *r2 = 99;
        println!("r2 modified value to: {}", *r2);
    }
}
```

### Differences Between References and Raw Pointers:
| Feature | Safe Reference (`&T`, `&mut T`) | Raw Pointer (`*const T`, `*mut T`) |
| :--- | :--- | :--- |
| **Nullability** | Never null. | Can be null (`std::ptr::null()`). |
| **Aliasing** | Strict aliasing XOR mutability. | Can have multiple mutable pointers to the same address. |
| **Lifetime** | Statically verified by compiler. | May point to dangling/freed memory. |
| **Alignment** | Guaranteed properly aligned. | Can be unaligned. |

---

## 3. Calling External C Functions (FFI)

To call functions from C libraries, declare them inside an `extern "C"` block:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // Calling C functions is unsafe because the compiler cannot verify C code
    let result = unsafe { abs(-42) };
    println!("abs(-42) = {result}");
}
```

---

## 4. Building a Safe FFI Wrapper

The idiomatic way to use unsafe code is to encapsulate it behind a safe API:

```rust
/// Safe wrapper around libc `abs`
pub fn safe_abs(n: i32) -> Option<u32> {
    if n == i32::MIN {
        // abs(i32::MIN) overflows i32 in C!
        None
    } else {
        let val = unsafe { abs(n) };
        Some(val as u32)
    }
}

fn main() {
    assert_eq!(safe_abs(-50), Some(50));
    assert_eq!(safe_abs(i32::MIN), None);
    println!("Safe FFI wrapper verified successfully!");
}
```

---

## Summary & Next Step

You have completed **Core Rust (Days 1 to 4)**! You have mastered the foundational language: syntax, types, pattern matching, traits, generics, memory management, lifetimes, modules, testing, errors, and unsafe basics.

Now we enter the **Advanced Systems Engineering Tracks**:
Proceed to **Module 5: Concurrency in Depth** starting with [05-concurrency/01-threads-and-scoped-threads.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/05-concurrency/01-threads-and-scoped-threads.md).
