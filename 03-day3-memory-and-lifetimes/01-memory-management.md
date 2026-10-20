# Chapter 13: Memory Architecture & Approaches to Management

To master Rust, you must understand how computer memory operates at the hardware and operating system levels, and how Rust's ownership model compares to manual management and garbage collection.

---

## 1. Anatomy of Program Memory

When an operating system runs an executable, it allocates a virtual address space typically divided into distinct segments:

```
High Memory Address
+-------------------------------------------------------+
|  Stack (grows DOWNWARD)                               |
|  - Function call frames, local variables, parameters  |
|  - Fixed size at compile time                         |
|  - Extremely fast allocation (stack pointer bump)     |
+-------------------------------------------------------+
|                           |                           |
|                           v                           |
|                                                       |
|                           ^                           |
|                           |                           |
+-------------------------------------------------------+
|  Heap (grows UPWARD)                                  |
|  - Dynamic sizing, long-lived data                    |
|  - Managed via allocator (malloc / jemalloc / mimalloc)|
|  - Pointer indirection; slower cache access           |
+-------------------------------------------------------+
|  BSS & Data Segment                                   |
|  - Global variables, static data (`static`)           |
+-------------------------------------------------------+
|  Text / Code Segment (Read-Only)                      |
|  - Compiled machine instructions                      |
|  - String literals (`"Hello"` -> &'static str)        |
+-------------------------------------------------------+
Low Memory Address
```

### 1.1 The Stack vs. The Heap

| Attribute | Stack Memory | Heap Memory |
| :--- | :--- | :--- |
| **Allocation Speed** | Instantaneous (sub-nanosecond pointer arithmetic: `sub rsp, N`). | Slower (allocator searches free lists, system calls). |
| **Deallocation** | Automatic when function returns (stack pointer pops). | Must be explicitly freed or collected. |
| **Size Constraints** | Fixed size (e.g., 8 MB default thread stack). Risk of stack overflow. | Limited only by available physical RAM and virtual swap. |
| **Access Pattern** | Contiguous, highly cache-friendly (L1/L2 cache hits). | Pointer chasing across memory; possible cache misses. |
| **Data Types** | Primitives, arrays (`[T; N]`), structs with fixed sizes. | `Vec<T>`, `String`, `Box<T>`, `HashMap`. |

---

## 2. Paradigms of Memory Management

Throughout computer science history, three primary approaches have emerged for managing dynamic heap memory:

```
                               Approaches to Memory
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
Manual Management               Garbage Collection                Ownership & RAII
(C, C++)                        (Java, Go, Python, C#)           (Rust, Modern C++)
- Programmer calls              - Tracing GC scans heap          - Compiler tracks
  `malloc`/`free` or `new`/`delete` at runtime                    lifetimes statically
- Fast, zero overhead           - Memory safe; no dangling ptrs   - Free memory at scope end
- Bug-prone: segfaults,         - High memory overhead,           - Zero runtime overhead,
  use-after-free, leaks           pause times (GC pauses)          deterministic, safe
```

### 2.1 The Problem with Manual Management (C / C++)
In manual memory management, every allocation requires exactly one corresponding free call:
- Forget to free $\implies$ **Memory Leak**.
- Free too early $\implies$ **Dangling Pointer / Use-After-Free**.
- Free twice $\implies$ **Double Free** (critical security exploit).

### 2.2 The Problem with Tracing Garbage Collection (Go, Java)
Garbage collectors eliminate memory safety bugs by running a background collector:
- **GC Pauses ("Stop-the-World"):** Unacceptable for real-time systems, audio drivers, gaming engines, and kernels.
- **Memory Bloat:** A GC typically requires 2x to 3x the resident memory footprint to operate efficiently without constant collection cycles.

### 2.3 The Rust Solution: Static Ownership
Rust achieves the speed and low footprint of C with the safety of a garbage-collected language by enforcing **Ownership at compile time**.

---

## 3. The Three Rules of Ownership

Rust's memory management rests entirely upon three invariants enforced by the compiler:

1. **Each value in Rust has an owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is automatically dropped and its memory is reclaimed.**

---

## Summary & Next Step

You now understand program memory layout and how Rust eliminates both manual frees and garbage collection pauses. Next, we explore **Ownership Rules, Move Semantics, and the Drop Trait** in [02-ownership-and-moves.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/03-day3-memory-and-lifetimes/02-ownership-and-moves.md).
