# Chapter 37: Safety Preconditions, Soundness, and Undefined Behavior

In systems programming, **Undefined Behavior (UB)** is not a simple runtime error or crash; it is the complete breakdown of compiler reasoning.

---

## 1. What is Undefined Behavior (UB)?

Under LLVM's optimization model, the compiler operates under the fundamental assumption that **Undefined Behavior will never happen**. 
- If a path of execution leads to UB, the optimizer can assume that path is dead code and delete it entirely.
- Optimizations can "travel backward in time", deleting bounds checks or security guards that preceded the UB!

### The Golden Rule of Rust:
> **Safe code must NEVER be able to cause Undefined Behavior. If safe code can trigger UB under any circumstance, the codebase is UNSOUND.**

---

## 2. The 3 Shapes of Sound Rust

Every function in a Rust codebase falls into one of three architectural shapes:

```
+------------------------------------+------------------------------------+
| Shape 1: Purely Safe               | Shape 2: Purely Unsafe             |
+------------------------------------+------------------------------------+
| `pub fn add(a: i32, b: i32) -> i32`| `pub unsafe fn get_unchecked(...)` |
| - Safe signature                   | - Unsafe signature                 |
| - Safe implementation              | - Caller must satisfy preconditions|
| - Guaranteed sound                 | - Explicit `# Safety` section      |
+------------------------------------+------------------------------------+
| Shape 3: Safe Abstraction Over Unsafe                                   |
+-------------------------------------------------------------------------+
| `pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])`   |
| - Safe public signature (no `unsafe` keyword on caller)                 |
| - Contains internal `unsafe { ... }` block                              |
| - Internal code rigorously guarantees all preconditions are satisfied   |
| - 100% SOUND safe abstraction!                                          |
+-------------------------------------------------------------------------+
```

---

## 3. Common Safety Preconditions

When calling or writing `unsafe fn`, you must document and verify these universal preconditions:

1. **Alignment:** The pointer address must be a multiple of `std::mem::align_of::<T>()`. Unaligned reads cause hardware faults on ARM and performance degradation on x86.
2. **Dereferenceability:** The memory region from `ptr` to `ptr + size_of::<T>()` must reside within an allocated virtual memory page owned by the process.
3. **Initialization:** The bytes must represent valid initialized memory for type `T`.
4. **Aliasing Rules:** You must never create a reference `&mut T` that overlaps with any other active reference (`&T` or `&mut T`).
5. **Non-Overlapping Memory (`copy_nonoverlapping`):** Source and destination pointers must not overlap (equivalent to C's `memcpy` vs `memmove`).

---

## 4. Writing a Soundness Proof

Whenever you write an `unsafe { ... }` block, you should mentally (or in comments) construct a formal **Soundness Proof**:

```rust
pub struct MySlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> MySlice<T> {
    /// Safe getter method
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SOUNDNESS PROOF:
            // 1. `index < self.len` ensures offset is strictly within the allocated bounds.
            // 2. `self.ptr` is guaranteed non-null, aligned, and points to `len` initialized `T` elements.
            // 3. The returned reference borrows from `&self`, ensuring the lifetime is tied to `MySlice`.
            // 4. Aliasing is preserved: only shared reference is created.
            // Therefore, this unsafe block is SOUND and can never trigger UB!
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
}
```

---

## Summary & Next Step

You have learned what Undefined Behavior is, the definition of Soundness, and how to write safety preconditions. Next, we master safe memory allocation and initialization in **Memory Lifecycle and MaybeUninit** in [03-memory-lifecycle-and-uninit.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/03-memory-lifecycle-and-uninit.md).
