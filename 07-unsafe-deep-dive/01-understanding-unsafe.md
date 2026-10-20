# Chapter 36: Understanding Unsafe: Contracts, Roles, and Invariants

Unsafe Rust is not a separate language; it is a formal partnership between the human engineer and the compiler.

---

## 1. Why Does Unsafe Exist?

The Rust compiler's type checker is **conservative**:
- It guarantees that **no unsafe program will ever compile**.
- As a consequence of the Halting Problem, it also **rejects some programs that are actually safe** because the compiler cannot mathematically prove their correctness.

```
+---------------------------------------------------------------+
|                      All Possible Programs                    |
|                                                               |
|   +-------------------------------------------------------+   |
|   |                 Actually Safe Programs                |   |
|   |                                                       |   |
|   |   +-----------------------------------------------+   |   |
|   |   |  Programs Accepted by Safe Rust Compiler      |   |   |
|   |   |  (Provably safe at compile time)              |   |   |
|   |   +-----------------------------------------------+   |   |
|   |                                                       |   |
|   |   Programs that are sound, but require `unsafe`       |   |
|   |   (e.g., Vec, OS kernels, lock-free queues)           |   |
|   +-------------------------------------------------------+   |
|                                                               |
|   Programs that trigger Undefined Behavior (Bugs, UB)         |
+---------------------------------------------------------------+
```

---

## 2. The Dual Roles of the `unsafe` Keyword

The `unsafe` keyword serves two completely distinct syntactic functions:

```
+------------------------------------+------------------------------------+
| 1. Author's Declaration of Risk    | 2. Caller's Proof of Compliance    |
|    (`unsafe fn`, `unsafe trait`)   |    (`unsafe { ... }`, `unsafe impl`|
+------------------------------------+------------------------------------+
| "I, the author, cannot have my     | "I, the caller, have read the      |
| safety requirements checked by the | documentation, checked all safety  |
| compiler. The caller must satisfy  | preconditions, and promise this    |
| my safety preconditions."          | invocation will never trigger UB." |
+------------------------------------+------------------------------------+
```

---

## 3. Safety Invariants vs. Validity Invariants

To avoid undefined behavior, systems engineers must understand the distinction between safety and validity:

| Invariant Class | Definition | Consequences of Violation |
| :--- | :--- | :--- |
| **Validity Invariant** | Language-level bit representation rules that must **never be broken, even for a single CPU cycle, inside or outside unsafe blocks**. | **Immediate Undefined Behavior (UB)**. LLVM optimizations assume validity invariants always hold. |
| **Safety Invariant** | Logical constraints enforced by a type or API (e.g., `Vec.len <= Vec.cap`). Can be temporarily violated *inside* an unsafe block as long as restored before returning. | Violating this in public safe APIs makes the API **unsound**. |

### Examples of Validity Violations (Instant UB!):
1. A `bool` having any value other than `0` (`false`) or `1` (`true`).
2. An `enum` containing a discriminant tag that does not correspond to a valid variant.
3. A reference (`&T` or `&mut T`) that is null or dangling.
4. An immutable reference `&T` that aliases a mutable reference `&mut T`.
5. A `char` with a value greater than `0x10FFFF` or in the surrogate range `[0xD800, 0xDFFF]`.
6. Reading uninitialized memory.

---

## Summary & Next Step

You have learned the philosophy of Unsafe Rust, its dual syntactic roles, and the distinction between safety and validity invariants. Next, we formalize safety proofs in **Safety Preconditions, Undefined Behavior, and Soundness** in [02-safety-preconditions.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/07-unsafe-deep-dive/02-safety-preconditions.md).
