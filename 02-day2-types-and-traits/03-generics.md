# Chapter 9: Generics, Trait Bounds, and Dispatch

Generics allow you to write algorithms and data structures that operate across multiple types without sacrificing type safety or runtime performance.

---

## 1. Generic Functions & Monomorphization

A generic function uses type parameters instead of concrete types:

```rust
fn pick_first<T>(a: T, _b: T) -> T {
    a
}

fn main() {
    let int_choice = pick_first(10, 20);       // T inferred as i32
    let str_choice = pick_first("foo", "bar"); // T inferred as &str
}
```

### 1.1 How Rust Generics Work: Monomorphization
Unlike languages like Java that use *type erasure* (converting objects to `Object` with runtime casting), Rust uses **monomorphization**:

```
Source Code:
fn pick_first<T>(a: T, b: T) -> T { ... }

Call sites in binary:
pick_first(10, 20);
pick_first("foo", "bar");
             |
             v [Compiler Monomorphization Step]
Generated Native Code:
fn pick_first_i32(a: i32, b: i32) -> i32 { ... }
fn pick_first_str(a: &str, b: &str) -> &str { ... }
```

#### Trade-offs:
- **Pros:** Zero runtime cost! Functions are compiled to dedicated native instructions, allowing LLVM to perform aggressive inlining and loop vectorization.
- **Cons:** Can increase binary size ("code bloat") and compilation time if a generic function is called with many distinct types.

---

## 2. Trait Bounds & Where Clauses

By default, a generic parameter `T` has no capabilities. To perform operations on `T`, you constrain it with **trait bounds**:

```rust
use std::fmt::Display;

// 1. Inline trait bound
fn print_duplicate<T: Display + Clone>(item: &T) {
    let clone = item.clone();
    println!("Duplicate: {clone}");
}

// 2. Where clause (preferred for complex signatures)
fn complex_comparison<T, U>(first: &T, second: &U) -> bool
where
    T: Display + PartialOrd<U>,
    U: Display,
{
    first > second
}
```

---

## 3. Generic Data Structures

Structs and enums can be parameterized with types:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Specialized implementation for f64 only:
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

---

## 4. Static vs. Dynamic Dispatch (`impl Trait` vs. `dyn Trait`)

Rust gives you explicit control over how polymorphism is executed at runtime:

```
+------------------------------------+------------------------------------+
|   Static Dispatch (`impl Trait`)   |    Dynamic Dispatch (`dyn Trait`)  |
+------------------------------------+------------------------------------+
| - Resolved at COMPILE TIME         | - Resolved at RUNTIME via VTABLE   |
| - Generates specialized code       | - Single function in binary        |
| - Zero runtime overhead            | - Pointer indirection penalty      |
| - Types must be known at compile   | - Can store heterogeneous items in |
|   time; cannot mix types in a Vec  |   a single collection (`Vec<Box>`) |
+------------------------------------+------------------------------------+
```

### Trait Object Memory Layout (`dyn Trait`)
A trait object is a **fat pointer** (16 bytes on 64-bit systems):

```
Trait Object: `&dyn Draw` or `Box<dyn Draw>`
+-------------------------+
| [ptr]  --> Data instance| (Points to the concrete struct on heap/stack)
| [vptr] --> VTable       | (Points to virtual method table in memory)
+-------------------------+
             |
             v
         VTable in Read-Only Data Section:
         +-----------------------+
         | Type destructor (drop)|
         | Type size & align     |
         | fn draw(&self) ptr    |
         +-----------------------+
```

### When to Use Which:
- Use `impl Trait` or generics by default for maximum performance.
- Use `Box<dyn Trait>` when you must hold a heterogeneous collection of different structs that implement the same trait (e.g., `Vec<Box<dyn Widget>>`).

---

---

## 5. Object Safety Rules

Not every trait can be used as a `dyn Trait` object. A trait is **object-safe** if:
1. All associated functions have a receiver (`&self`, `&mut self`, `self`). (No static constructors like `fn new()`).
2. Method return types do not involve `Self`.
3. Methods do not have generic type parameters.

---

## 5.1 Principal Engineer Deep Dive: Rust Generics & Dispatch vs. The JVM

For a senior Java engineer, understanding how Rust handles generics and polymorphism at the hardware level is transformative:

### 1. Monomorphization vs. JVM Type Erasure

```
Java (Type Erasure):
  List<Integer> ───┐
                   ├───> Bytecode: List (raw type, holds Object references)
  List<String>  ───┘        Runtime: Checkcast instructions + Heap Boxing

Rust (Monomorphization):
  Vec<i32>    ─────────> Machine Code: Vec_i32 (flat [i32; N] in contiguous memory)
  Vec<String> ─────────> Machine Code: Vec_String (flat [ptr, cap, len] structs)
```

| Dimension | Java Generics (JVM) | Rust Generics (`rustc` / LLVM) |
| :--- | :--- | :--- |
| **Compilation** | Erased to `Object` (or first bound) | Fully monomorphized to concrete types |
| **Primitives** | Requires boxed types (`Integer`, `Double`) | Primitives are first-class, unboxed (`i32`, `f64`) |
| **Memory Overhead** | 16–24 byte object header per boxed element | **0 bytes overhead**; flat contiguous array |
| **Cache Locality** | Pointer chasing across heap memory | Direct L1/L2 cache line utilization (64 bytes/line) |
| **Hardware Inlining** | JIT compiler speculative devirtualization | LLVM compiles direct branch instructions, aggressively inlining |
| **Code Size** | Single bytecode representation | Duplicate machine code per type specialization |

### 2. Trait Objects (`dyn Trait`) vs. Java Virtual Method Tables (`invokevirtual`)

In Java, polymorphism is **intrinsic to every heap object**:
```
Java Heap Object Layout:
┌───────────────────────────────────────────────┐
│ Mark Word (8 bytes: GC age, lock state, hash) │
├───────────────────────────────────────────────┤
│ Klass Pointer (8 bytes: points to Class metadata & VTable) │
├───────────────────────────────────────────────┤
│ Instance Fields...                            │
└───────────────────────────────────────────────┘
```
Every non-final method invocation in Java is executed via `invokevirtual` or `invokeinterface`.

In Rust, polymorphism is **opt-in and external to the struct**:
```
Rust Fat Pointer (16 bytes on stack):
┌───────────────────────────┬───────────────────────────┐
│ Data Pointer (8 bytes)    │ VTable Pointer (8 bytes)  │
└─────────────┬─────────────┴─────────────┬─────────────┘
              │                           │
              v                           v
     Struct Data on Stack/Heap       Shared Read-Only VTable:
     ┌───────────────────────┐       ┌───────────────────────┐
     │ Field 1: u32 (4 bytes)│       │ Drop glue fn pointer  │
     │ Field 2: u32 (4 bytes)│       │ Size & alignment (8B) │
     └───────────────────────┘       │ Method fn pointers... │
                                     └───────────────────────┘
```

**Key Takeaways for Principle Engineers:**
1. **Clean Struct Layout**: A Rust struct has **no** hidden object header, **no** vtable pointer, and **no** monitor lock. A struct with two `u32` fields is exactly **8 bytes** in memory.
2. **Polymorphic Reference, Not Polymorphic Object**: In Rust, the struct doesn't know it's being used polymorphically. The fat pointer `&dyn Trait` holds the vtable pointer, not the data.
3. **Mechanical Sympathy**: Because generic Rust code is monomorphized, LLVM can apply auto-vectorization (SIMD) and constant folding that JIT compilers struggle to guarantee.

---

## 6. Exercise: Generic `min` Function

### Problem Description
Write a generic function `min` that accepts two values of the same type `T` and returns the smaller of the two.
- What trait bound is required on `T` to support ordering comparisons?

### Starter Code
```rust
fn min<T>(a: T, b: T) -> T {
    todo!("Implement generic min")
}

fn main() {
    assert_eq!(min(10, 20), 10);
    assert_eq!(min("apple", "banana"), "apple");
}
```

---

### Solution & Deep-Dive

```rust
fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

fn main() {
    assert_eq!(min(10, 20), 10);
    assert_eq!(min(42.5 as u64, 18 as u64), 18);
    assert_eq!(min("rust", "c++"), "c++");
    println!("Generic min verified successfully!");
}
```

### Deep Dive: Why `Ord` and not `PartialOrd`?
- Rust separates total ordering (`Ord`) from partial ordering (`PartialOrd`).
- `f32` and `f64` do **not** implement `Ord` because of IEEE-754 `NaN` (Not a Number), where `NaN <= NaN` is false and `NaN >= NaN` is false!
- If you need to support floats, you can bound on `PartialOrd`, but must decide how to handle `None` comparisons (`a.partial_cmp(&b)`).

---

## Summary & Next Step

You have mastered generics, monomorphization, trait bounds, and static vs. dynamic dispatch (`dyn Trait`). Next, we explore functional programming: **Closures and the Fn Traits** in [04-closures.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/02-day2-types-and-traits/04-closures.md).
