# Chapter 2: Types, Variables, and Arithmetic

In Rust, every value has a concrete type determined at compile time. Rust pairs static typing with local type inference, giving you both the safety of strong types and the readability of clean code.

---

## 1. Variables and Immutability

Variable bindings are declared using the `let` keyword:

```rust
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    
    // x = 20; // COMPILE ERROR: cannot assign twice to immutable variable `x`
}
```

### Why Immutable by Default?
In Rust, **variables are immutable by default**. 
- **Predictability:** When reading code, you can be certain that a variable's value never changes once bound.
- **Thread Safety:** Immutability makes concurrent reads completely free of data races.
- **Optimization:** The compiler can aggressively optimize constants and register allocations.

To allow mutation, explicitly mark the binding with `mut`:

```rust
fn main() {
    let mut x: i32 = 10;
    println!("Initial x: {x}");
    x = 20;
    println!("Mutated x: {x}");
}
```

> [!NOTE]
> If you declare `let mut x = 10;` but never mutate `x`, the compiler will emit a helpful warning: `variable does not need to be mutable`. The compiler encourages minimizing mutability.

---

## 2. Scalar Types

Scalar types represent a single primitive value. Rust has four primary categories: **integers**, **floating-point numbers**, **booleans**, and **characters**.

### 2.1 Integers

Integers are numbers without a fractional component:

| Width | Signed (`iN`) | Unsigned (`uN`) | Range (Signed) | Range (Unsigned) |
| :--- | :--- | :--- | :--- | :--- |
| **8-bit** | `i8` | `u8` | $-128$ to $127$ | $0$ to $255$ |
| **16-bit** | `i16` | `u16` | $-32,768$ to $32,767$ | $0$ to $65,535$ |
| **32-bit** | `i32` | `u32` | $\approx -2.14 \times 10^9$ to $2.14 \times 10^9$ | $0$ to $\approx 4.29 \times 10^9$ |
| **64-bit** | `i64` | `u64` | $\approx -9.22 \times 10^{18}$ to $9.22 \times 10^{18}$ | $0$ to $\approx 1.84 \times 10^{19}$ |
| **128-bit** | `i128` | `u128` | $\approx -1.7 \times 10^{38}$ to $1.7 \times 10^{38}$ | $0$ to $\approx 3.4 \times 10^{38}$ |
| **Arch** | `isize` | `usize` | Target pointer width (64-bit on x86_64, 32-bit on ARM Cortex-M) | Target pointer width |

- `isize` and `usize` depend on the architecture of the target platform (32 bits on 32-bit CPUs, 64 bits on 64-bit CPUs).
- `usize` is the mandatory type used for **indexing collections** (e.g., `array[index]`) and representing memory sizes.

### 2.2 Integer Literals

Rust provides flexible syntax for writing integer literals:

```rust
fn main() {
    let decimal = 98_222;        // Decimal with visual underscore separator
    let hex = 0xff;              // Hexadecimal (255)
    let octal = 0o77;            // Octal (63)
    let binary = 0b1111_0000;    // Binary (240)
    let byte = b'A';             // u8 byte literal (ASCII 65)
    
    // Explicit type suffixes
    let big_num = 1_000_000_u64;
    let small_num = 42_i8;
}
```

### 2.3 Floating-Point Numbers

Rust provides two IEEE-754 standard floating-point types:
- `f32`: Single-precision (32 bits).
- `f64`: Double-precision (64 bits). **Default float type in Rust** due to modern CPU performance.

```rust
let f1: f64 = 3.1415926535;
let f2: f32 = 2.5_f32;
let exp = 1.0e-5; // Exponential notation
```

### 2.4 Booleans

Represented by the `bool` type:
- Values: `true` or `false`.
- Size: Exactly **1 byte (8 bits)** in memory.

```rust
let is_active: bool = true;
let is_greater: bool = 10 > 5;
```

### 2.5 Characters

Represented by `char`:
- Delimited by single quotes `'a'`.
- Size: Exactly **4 bytes (32 bits)** wide.
- Represents a **Unicode Scalar Value** (can represent Roman, Cyrillic, Chinese, Kanji, Emoji, accented glyphs, etc.).

```rust
let c1: char = 'z';
let c2: char = '🦀'; // Unicode emoji
let c3: char = 'α';  // Greek letter
```

> [!IMPORTANT]
> In C/C++, `char` is an 8-bit ASCII byte. In Rust, an 8-bit byte is `u8`, while `char` is a 4-byte Unicode scalar value.

---

## 3. Arithmetic Operations & Integer Overflow

Rust supports standard arithmetic operators: `+`, `-`, `*`, `/`, `%`.

```rust
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}
```

### The Overflow Dilemma
In C/C++, signed integer overflow is **Undefined Behavior (UB)**—compilers can delete code assuming overflow never happens.
In Rust, integer behavior is strictly defined:

```
                      Integer Overflow Event
                                |
               +----------------+----------------+
               |                                 |
        Debug Build                      Release Build
     (`cargo run`)                   (`cargo run --release`)
               |                                 |
         Panics with:                   Wraps around silently
    "attempt to add with            using two's complement
         overflow"                      (modular arithmetic)
```

### Explicit Overflow Strategies
If you intentionally require specific behavior for arithmetic that might overflow, use standard library methods:

```rust
fn main() {
    let x: u8 = 250;
    
    // 1. Wrapping: Modular arithmetic (250 + 10 mod 256 = 4)
    let wrap = x.wrapping_add(10);
    println!("wrapping_add: {wrap}"); // 4

    // 2. Checked: Returns Option<T> (None if overflow occurs)
    let checked = x.checked_add(10);
    println!("checked_add: {checked:?}"); // None

    // 3. Saturating: Clamps at minimum or maximum boundary
    let sat = x.saturating_add(10);
    println!("saturating_add: {sat}"); // 255 (u8::MAX)

    // 4. Overflowing: Returns (value, bool_did_overflow)
    let (val, did_overflow) = x.overflowing_add(10);
    println!("overflowing_add: {val}, overflowed? {did_overflow}"); // 4, true
}
```

---

## 4. Type Inference and Strong Typing

Rust is strictly statically typed, but you don't have to annotate every variable:

```rust
fn takes_u32(x: u32) { println!("u32: {x}"); }
fn takes_i8(y: i8)   { println!("i8: {y}"); }

fn main() {
    let x = 10; // Compiler infers u32 because of its usage below
    let y = 20; // Compiler infers i8 because of its usage below

    takes_u32(x);
    takes_i8(y);
}
```

### Type Defaults
When no context constrains the type of an unannotated numeric literal:
- Integers default to `i32`.
- Floating-point numbers default to `f64`.

### No Implicit Conversions
Rust **never** performs implicit type coercion. You cannot assign an `i16` to an `i32`, or add an integer to a float:

```rust
let a: i16 = 10;
// let b: i32 = a; // COMPILE ERROR: mismatched types

// Explicit casting with `as`:
let b: i32 = a as i32; 

let float_val = 3.14;
let int_val = 20;
// let sum = float_val + int_val; // COMPILE ERROR: cannot add {float} to {integer}
let sum = float_val + (int_val as f64); // OK
```

---

## 5. Exercise: Fibonacci Sequence

### Problem Description
The Fibonacci sequence begins with $[0, 1]$. For $n \ge 2$, each term is the sum of the two preceding terms:
$$F_0 = 0, \quad F_1 = 1, \quad F_n = F_{n-1} + F_{n-2}$$

Write a function `fib(n: u32) -> u32` that calculates the $n$-th Fibonacci number.
- At what value of $n$ will this function panic in debug mode?

### Starter Code
```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case: F(0) = 0, F(1) = 1
        todo!("Implement base case")
    } else {
        // The recursive case: F(n) = F(n-1) + F(n-2)
        todo!("Implement recursive step")
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```

---

### Solution & Deep-Dive

#### Recursive Solution
```rust
fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
```

#### Iterative Solution (More Efficient)
```rust
fn fib_iterative(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
```

### Deep Dive: When Does it Panic?
`u32` has an upper limit of $2^{32} - 1 = 4,294,967,295$.
- $F_{46} = 1,836,311,903$ (fits in `u32`)
- $F_{47} = 2,971,215,073$ (fits in `u32`)
- $F_{48} = 4,807,526,976 > 2^{32} - 1$ ($\implies$ **overflows `u32`**)

In Debug mode (`cargo run`), calculating `fib(48)` immediately panics with:
```
thread 'main' panicked at 'attempt to add with overflow', src/main.rs:6:9
```
In Release mode (`cargo run --release`), it wraps around to $512,559,680$ producing an incorrect answer.

---

## Summary & Next Step

You have mastered Rust's primitive types, immutability, integer overflow semantics, and type inference. Next, we explore **Blocks, Expressions, and Control Flow** in [03-control-flow-basics.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/01-day1-basics/03-control-flow-basics.md).
