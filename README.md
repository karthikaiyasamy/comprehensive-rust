# Comprehensive Rust 🦀

Welcome to **Comprehensive Rust**, an extensive, deeply structured reference and interactive curriculum based on Google's renowned Rust training program, now enhanced with **comprehensive Java-to-Rust comparisons**, **visual memory diagrams**, **deep type system coverage**, and **debugging guides** specifically for Java developers.

This repository contains searchable, comprehensive Markdown chapters covering **foundational theory**, **under-the-hood memory diagrams**, **common compiler traps**, and a **companion Cargo workspace of 21 runnable exercise packages**.

---

## 📚 Curriculum Table of Contents

### [01. Day 1: Rust Fundamentals](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics)
1. [01. Welcome to Rust & The Cargo Ecosystem](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/01-welcome-and-cargo.md)
   - Why Rust, zero-cost abstractions, memory safety without GC.
   - Cargo toolchain (`check`, `build`, `test`, `clippy`, `fmt`), Debug vs. Release profiles.
2. [02. Types, Variables, and Arithmetic](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/02-types-and-values.md)
   - Immutability by default, scalar types (`iN`, `uN`, `fN`, `bool`, `char`).
   - Integer overflow semantics (Debug panic vs. Release wrap, `.checked_add`, `.saturating_add`).
   - Type inference and strict type casting.
   - *Exercise: Fibonacci Sequence*
3. [03. Control Flow Basics & Functions](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/03-control-flow-basics.md)
   - Scopes, variable shadowing, expressions vs. statements, the unit type `()`.
   - `if` expressions, exhaustive `match` expressions.
   - Loops (`loop`, `while`, `for`), loop labels, `break` with return values.
   - Function signatures, macros (`println!`, `dbg!`, `assert!`).
   - *Exercise: Collatz Sequence*
4. [04. Tuples, Arrays, and Destructuring](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/04-tuples-and-arrays.md)
   - Fixed-size stack arrays `[T; N]`, bounds checking, repeat initialization syntax `[0; 1024]`.
   - Heterogeneous tuples `(T1, T2, ...)`, pattern destructuring.
   - *Exercise: Nested Arrays (Matrix Transpose)*
5. [05. References, Slices, and Strings](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/05-references-and-slices.md)
   - Shared references `&T` vs. Exclusive references `&mut T`.
   - The Fundamental Rule: Aliasing XOR Mutability.
   - Slice fat pointers `&[T]` (pointer + length).
   - `&str` vs. `String`, UTF-8 encoding rules, dangling pointer prevention.
   - *Exercise: 3D Geometry Utilities*
6. [06. User-Defined Types: Structs, Enums, Const, and Static](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/06-user-defined-types.md)
   - Named structs, field init shorthand, struct update syntax.
   - Tuple structs and the Newtype pattern.
   - Algebraic enums, tagged union memory layout, Null Pointer Optimization (Niche filling).
   - Type aliases, `const` vs. `static`.
   - *Exercise: Elevator Events Modeling*

### [Bonus: Java Developer Guides]
7. [07. Java to Rust Comparison Guide](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/07-java-to-rust-comparison.md)
   - Side-by-side comparisons for Java developers
   - Mental model mapping between Java and Rust
   - Common patterns and anti-patterns
   - Debugging tips specific to Java developers

---

### [02. Day 2: Types, Traits, and the Standard Library](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits)
8. [01. Advanced Pattern Matching & Control Flow](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/01-pattern-matching.md)
   - Refutable vs. irrefutable patterns, matching literals, ranges, struct/enum destructuring.
   - Match guards, variable binding with `@`.
   - Let control flow: `if let`, `while let`, and `let else` guard clauses.
   - *Exercise: Arithmetic Expression Evaluation*
9. [02. Methods, Traits, and Deriving](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/02-methods-and-traits.md)
   - `impl` blocks, receiver types (`&self`, `&mut self`, `self`), associated functions.
   - Defining and implementing traits, default methods, supertraits, associated types.
   - Trait coherence and the Orphan Rule.
   - Automatic trait derivation with `#[derive(...)]`.
   - *Exercise: Modular Logger Trait & Decorator*
10. [03. Generics, Trait Bounds, and Dispatch](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/03-generics.md)
    - Generic functions, structs, and enums.
    - Compile-time monomorphization and zero runtime cost.
    - Trait bounds and `where` clauses.
    - Static dispatch (`impl Trait`) vs. Dynamic dispatch (`dyn Trait` trait objects and vtables).
    - Object safety requirements.
    - *Exercise: Generic min Function*
11. [04. Closures and Environment Capturing](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/04-closures.md)
    - Closure syntax and type inference.
    - Capturing modes: `Fn` (borrow), `FnMut` (mutable borrow), `FnOnce` (move/consume).
    - The `move` keyword for thread and lifecycle management.
    - *Exercise: Log Filter with Closures*
12. [05. Standard Library Types: Option, Result, Vec, String, and HashMap](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/05-standard-library-types.md)
    - `Option<T>` combinators (`unwrap_or`, `map`, `and_then`).
    - `Result<T, E>` and the `?` try operator.
    - Dynamic array heap memory layout for `Vec<T>`.
    - Key-value mapping with `HashMap<K, V>` and the `.entry()` API.
    - Lazy composable iterators (`.iter()`, `.iter_mut()`, `.into_iter()`, `.map()`, `.filter()`).
    - *Exercise: Frequency Counter with Entry API*
13. [06. Standard Library Traits: Comparisons, Conversions, and I/O](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/06-standard-library-traits.md)
    - Comparison traits: `PartialEq`, `Eq`, `PartialOrd`, `Ord` (NaN nuances).
    - Operator overloading with `std::ops`.
    - Conversions: `From` & `Into`, `TryFrom` & `TryInto` (why `TryFrom` is safer than `as`).
    - Standard I/O streaming traits: `std::io::Read` and `std::io::Write`.
    - *Exercise: ROT13 Streaming Decoder*

### [Bonus: Enhanced Type System Coverage for Java Developers]
14. [07. Deep Dive: Type System Mastery for Java Developers](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/07-deep-dive-types-java-developers.md)
    - Algebra of Types: Product vs Sum types
    - Newtype pattern: Zero-cost type safety
    - Phantom types: Type-level programming
    - Type state pattern: Compile-time state machines
    - Zero-sized types: Compile-time guarantees
    - **Why this knowledge transfers beyond Rust**

---

### [03. Day 3: Memory Management, Ownership, and Lifetimes](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes)
15. [01. Memory Architecture & Approaches to Management](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/01-memory-management.md)
    - Stack vs. Heap layout, pointer arithmetic, allocation speeds.
    - Comparison: Manual Free (C/C++) vs. Tracing GC (Go/Java) vs. Static Ownership (Rust).
    - The Three Rules of Ownership.
16. [02. Ownership, Move Semantics, and Drop](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/02-ownership-and-moves.md)
    - Move semantics: shallow metadata copy + source invalidation. Double-free prevention.
    - `Copy` (implicit stack bitwise copy) vs. `Clone` (explicit deep heap copy).
    - The `Drop` trait, destruction order (reverse declaration for locals, declaration order for fields), `std::mem::drop`.
    - *Exercise: Type-Safe Builder Pattern*
17. [03. Smart Pointers: Box, Rc, and Interior Mutability](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/03-smart-pointers.md)
    - `Box<T>`: heap allocation and recursive data structures.
    - `Rc<T>`: reference-counted shared ownership for single-threaded graphs.
    - `RefCell<T>`: interior mutability and runtime borrow checking.
    - The `Rc<RefCell<T>>` shared-mutable architecture.
    - *Exercise: Generic Binary Search Tree*
18. [04. The Borrow Checker: Aliasing, NLL, and Invariants](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/04-borrowing-and-references.md)
    - Freezing and invalidation: why iterator invalidation is mathematically impossible in Rust.
    - Non-Lexical Lifetimes (NLL): borrows end at last use.
    - Reborrowing mechanics (`&mut *ref`).
    - Diagnostic guide to solving borrow checker errors.
    - *Exercise: The Wizard's Inventory*

### [Bonus: Visual Ownership Guide for Java Developers]
19. [07. Visual Ownership & Borrowing Guide for Java Developers](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/07-java-developers-visual-guide.md)
    - Memory models: Java vs Rust comparison
    - Ownership moves: Step-by-step visual diagrams
    - Borrowing: References in action
    - Borrow checker scenarios: Visual analysis
    - Lifetimes visualized
    - Common Java-to-Rust ownership misconceptions
    - Memory diagram practice exercises

20. [05. Lifetimes: Annotations, Structs, and Elision Rules](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/05-lifetimes.md)
    - Why lifetime parameters exist: annotating relationships between inputs and outputs.
    - The Three Lifetime Elision Rules.
    - Structs holding references.
    - The `'static` lifetime (string literals vs. owned type bounds).
    - *Exercise: Zero-Copy Protocol Decoding (Protobuf Parsing)*
21. [06. Advanced Slices, Subtyping, and Lifetime Bounds](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/06-slices-and-lifetimes.md)
    - Decoupling struct fields with multiple lifetime parameters (`'a`, `'b`).
    - Lifetime subtyping: `'a: 'b` ("`'a` outlives `'b`").
    - Type lifetime bounds `T: 'a`.
    - UTF-8 safe slicing with `char_indices()`.

---

### [04. Day 4: Errors, Testing, and Modules](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules)
22. [01. Packages, Crates, and Modules](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/01-modules-and-packages.md)
    - Package layout, crates (library vs. binary), module tree hierarchy.
    - Privacy boundaries: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`.
    - Path resolution: `use`, `super`, `self`, and re-exporting with `pub use`.
    - *Exercise: Modules for a GUI Library*
23. [02. Testing: Unit Tests, Integration Tests, and Clippy](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/02-testing.md)
    - Unit tests (`#[cfg(test)]`, `#[test]`, `#[should_panic]`).
    - Integration tests in `tests/` directory.
    - Executable documentation tests (doctests).
    - Compiler lints and Clippy static analysis.
    - *Exercise: Validating Numbers with the Luhn Algorithm*
24. [03. Error Handling: Panics, Result, thiserror, and anyhow](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/03-error-handling.md)
    - Panics: unrecoverable bugs, stack unwinding vs. abort (`panic = "abort"`).
    - Recoverable errors with `Result<T, E>` and `?`.
    - Reusable library error enums with `thiserror`.
    - Ergonomic application error reporting with `anyhow` and `.context()`.
    - *Exercise: Rewriting Panics to Result*

### [Bonus: Error Handling Guide for Java Developers]
25. [05. Error Handling: Java vs Rust - Complete Guide](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/05-java-developers-error-handling-guide.md)
    - Philosophy: Exceptions vs Result Types
    - Compile-time guarantees: exhaustiveness
    - The `?` Operator: Composable Error Propagation
    - Custom Error Types: thiserror
    - anyhow: Ergonomic Application Errors
    - Real-World Example: API Client
    - Best practices for Java developers
    - Migration guide: Converting Java code

---

### [05. Concurrency in Depth](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency)
26. [01. Threads & Scoped Threads](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/01-threads-and-scoped-threads.md)
    - Native OS threads with `thread::spawn` and `JoinHandle`.
    - Scoped threads with `thread::scope`: safely borrowing local stack frames concurrently without `Arc`.
27. [02. Message Passing & MPSC Channels](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/02-channels-and-message-passing.md)
    - `std::sync::mpsc`: Multiple Producer, Single Consumer.
    - Unbounded channels vs. bounded channels (`sync_channel`) with backpressure.
    - Rendezvous synchronization (`sync_channel(0)`).
    - Channel disconnection and ownership recovery semantics.
28. [03. The Thread-Safety Contract: Send and Sync](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/03-send-and-sync.md)
    - Formal definitions: `Send` (ownership transfer) and `Sync` (shared reference transfer).
    - The Fundamental Equivalence: $T \text{ is } Sync \iff \&T \text{ is } Send$.
    - Why `Rc` and `RefCell` are forbidden across threads.
    - Upgrading to `Arc` and `Mutex`.
29. [04. Shared State Concurrency: Arc, Mutex, and RwLock](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/04-shared-state-concurrency.md)
    - The `Arc<Mutex<T>>` synchronization pattern.
    - RAII `MutexGuard` automatic unlocking.
    - Lock poisoning and recovery.
    - Reader-writer locks (`RwLock<T>`).
    - *Exercise: The Dining Philosophers Problem (Deadlock Prevention)*
30. [05. Asynchronous Programming: Futures, Poll, and Runtimes](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/05-async-basics.md)
    - Cooperative multitasking vs. OS threads.
    - Why Futures are lazy.
    - The `Future` trait, `poll()`, `Poll::Pending` vs. `Poll::Ready`, and `Waker`.
    - Async state machine enum transformation.
    - The Tokio runtime and executor.

### [Bonus: Concurrency & Async Guide for Java Developers]
31. [08. Concurrency & Async: Java to Rust Transition Guide](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/08-java-developers-concurrency-guide.md)
    - Threads: std::thread vs Java Thread Pool
    - Shared State: Mutex & Arc
    - The Send and Sync Traits: Thread Safety as Types
    - Channels: MPSC vs Java BlockingQueue
    - Async/Await: Tokio vs CompletableFuture
    - Real-World Example: Concurrent Web Crawler
    - Common Pitfalls for Java Developers
    - Debugging Concurrency Issues
    - Best practices for Java developers

32. [06. Async Control Flow: Channels, Join, and Select](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/06-async-channels-and-control-flow.md)
    - Tokio channel architectures: `mpsc`, `oneshot`, `broadcast`, and `watch`.
    - Concurrent execution with `tokio::join!`.
    - Multiplexing and racing with `tokio::select!`.
33. [07. Async Pitfalls, Pinning, and Cancellation Safety](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/07-async-pitfalls-and-cancellation.md)
    - Pitfall 1: Blocking the executor thread pool (and solving with `spawn_blocking`).
    - Pitfall 2: Pinning and self-referential future structures.
    - Pitfall 3: Cancellation safety (handling dropped futures in `select!`).
    - *Exercise: Multi-Client Broadcast Chat Server*

---

### [06. Idiomatic Rust & Advanced API Design](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust)
34. [01. Foundations of API Design & Documentation](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/01-api-design-and-documentation.md)
    - Markdown doc comments (`///`, `//!`) with standard `# Examples`, `# Panics`, `# Safety` sections.
    - Rust API Guidelines: `as_` (borrowed), `to_` (borrowed to owned), `into_` (owned).
    - Field getters: omitting the `get_` prefix.
    - Strong domain types vs. primitive obsession.
35. [02. Leveraging the Type System: Newtypes, RAII, and Extension Traits](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/02-leveraging-the-type-system.md)
    - The Newtype pattern for zero-cost type safety and bypassing the Orphan Rule.
    - RAII Scope Guards and transactional rollback.
    - Extension traits for adding methods to standard types.
36. [03. The Typestate Pattern: Compile-Time State Machines](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/03-typestate-pattern.md)
    - Eliminating runtime assertion checks using Zero-Sized Types (ZSTs).
    - Compile-time prevention of invalid method invocations.
    - Invalidating previous states through value moves (`self`).
37. [04. Borrow Checker Invariants: Split Borrows & PhantomData](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/04-borrow-checker-invariants.md)
    - Struct field-level split borrows vs. method receiver whole-struct borrows.
    - Slicing splitting with `split_at_mut`.
    - `PhantomData<T>`: communicating ownership, lifetimes, and variance to the compiler.
38. [05. Token Types & Capability-Based Security](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/05-token-types-and-capabilities.md)
    - Unforgeable zero-sized capability tokens.
    - Enforcing hardware initialization order and authorization at compile time with zero runtime cost.
39. [06. Polymorphism: Composition Over Inheritance](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/06-polymorphism-composition.md)
    - Avoiding fragile base classes and deep OOP inheritance trees.
    - Struct composition and trait contracts.
    - Strategy pattern and trade-off comparison between Static and Dynamic dispatch.

---

### [07. The Unsafe Rust Deep Dive](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive)
40. [01. Understanding Unsafe: Contracts, Roles, and Invariants](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/01-understanding-unsafe.md)
    - Why unsafe exists: the conservative nature of static analysis.
    - The dual roles of `unsafe`: author's risk declaration vs. caller's proof of compliance.
    - Safety invariants vs. Validity invariants (instant UB).
41. [02. Safety Preconditions, Soundness, and Undefined Behavior](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/02-safety-preconditions.md)
    - The definition of Soundness: safe code can never trigger UB.
    - The 3 Shapes of Sound Rust.
    - Universal preconditions: alignment, dereferenceability, initialized memory, aliasing.
    - Constructing rigorous Soundness Proofs.
42. [03. Memory Lifecycle & MaybeUninit](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/03-memory-lifecycle-and-uninit.md)
    - Memory phases: Allocation -> Initialization -> Usage -> Destruction -> Deallocation.
    - Why reading uninitialized bytes is immediate UB.
    - Sound uninitialized memory manipulation with `std::mem::MaybeUninit<T>`.
    - Partial array initialization and writing without dropping.
43. [04. Pinning in Depth: Self-Referential Types & Drop](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/04-pinning-in-depth.md)
    - What is a move? Why moves invalidate internal self-referential pointers.
    - The `Pin<P>` mathematical contract.
    - The `Unpin` auto trait and opting out with `PhantomPinned`.
    - Step-by-step construction of a sound self-referential buffer.
44. [05. Foreign Function Interface (FFI) & C Interoperability](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/05-ffi-and-language-interop.md)
    - ABI compatibility with `extern "C"`.
    - Struct memory layouts with `#[repr(C)]`.
    - String interoperability: `CStr` and `CString`.
    - End-to-end safe FFI wrapper around POSIX `srand` and `rand`.

---

## 🛠️ Companion Cargo Exercises Workspace

All 21 exercises from the curriculum are packaged as standalone, runnable Rust packages inside the [`exercises/`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises) directory.

### Running All Tests
To compile and test the entire exercise suite:
```bash
cargo test --manifest-path exercises/Cargo.toml
```

### Running an Individual Exercise
You can test any single exercise package directly:
```bash
# Test Day 1 Fibonacci
cargo test --manifest-path exercises/Cargo.toml -p day1_fibonacci

# Test Day 1 Collatz
cargo test --manifest-path exercises/Cargo.toml -p day1_collatz

# Test Day 2 Expression Evaluation
cargo test --manifest-path exercises/Cargo.toml -p day2_expression_eval

# Test Day 3 Protobuf Zero-Copy Parsing
cargo test --manifest-path exercises/Cargo.toml -p day3_protobuf_parsing

# Test Concurrency Dining Philosophers
cargo test --manifest-path exercises/Cargo.toml -p concurrency_dining_philosophers

# Test Concurrency Async Broadcast Chat
cargo test --manifest-path exercises/Cargo.toml -p concurrency_async_chat
```

### Exercise Package Index
| Package Name | Topic | Key Concepts |
| :--- | :--- | :--- |
| [`day1_fibonacci`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day1_fibonacci) | Types & Arithmetic | Recursion vs. iteration, integer overflow in debug mode. |
| [`day1_collatz`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day1_collatz) | Control Flow | `while` loops, `if` expressions, mutable arguments. |
| [`day1_nested_arrays`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day1_nested_arrays) | Compound Types | Stack-allocated 2D matrices, copy semantics. |
| [`day1_geometry`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day1_geometry) | References & Slices | In-place normalization with `&mut [f64; 3]`. |
| [`day1_elevator`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day1_elevator) | User-Defined Types | Nested enums with data, type aliases, constructors. |
| [`day2_expression_eval`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_expression_eval) | Pattern Matching | Recursive AST evaluation using `Box<Expression>`, division by zero handling. |
| [`day2_logger_trait`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_logger_trait) | Traits & Methods | Logger abstraction, decorator filter struct composition. |
| [`day2_generic_min`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_generic_min) | Generics | `Ord` trait bounds, total vs. partial ordering. |
| [`day2_log_filter`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_log_filter) | Closures | `Fn(&str) -> bool` predicates, environment capture. |
| [`day2_counter`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_counter) | Standard Library | Generic frequency counting using `HashMap::entry()`. |
| [`day2_rot13`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day2_rot13) | Standard I/O Traits | Custom stream decryptor implementing `std::io::Read`. |
| [`day3_builder_type`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day3_builder_type) | Ownership & Move | Consuming builder pattern with `self` by value. |
| [`day3_binary_tree`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day3_binary_tree) | Smart Pointers | Recursive BST using `Option<Box<Node<T>>>`. |
| [`day3_protobuf_parsing`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day3_protobuf_parsing) | Lifetimes | Zero-copy tag-length-value decoding using `&'a str`. |
| [`day4_gui_modules`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day4_gui_modules) | Modules & Packages | Module tree encapsulation, trait objects (`dyn Widget`). |
| [`day4_luhn_algorithm`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day4_luhn_algorithm) | Testing & Algorithms | Luhn checksum algorithm with comprehensive unit tests. |
| [`day4_result_error_handling`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/day4_result_error_handling) | Error Handling | Structured parser returning `Result<T, ParseError>`. |
| [`concurrency_dining_philosophers`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/concurrency_dining_philosophers) | Multi-threading | Resource hierarchy lock ordering to prevent deadlocks. |
| [`concurrency_link_checker`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/concurrency_link_checker) | Message Passing | Worker threads sending results across `mpsc::channel`. |
| [`concurrency_async_chat`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/concurrency_async_chat) | Async Tokio | Asynchronous multi-client broadcast using `tokio::sync::broadcast`. |
| [`unsafe_ffi_wrapper`](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/exercises/unsafe_ffi_wrapper) | Unsafe & FFI | Safe wrapper around C standard library `abs()` handling `i32::MIN`. |

---

## 🎯 Recommended Study Roadmap

1. **Week 1: Fundamentals (Days 1 & 2):** Read Chapters 1 through 12. Run and experiment with exercises `day1_*` and `day2_*`. Build comfort with the compiler and type checker.
   - **Java Developers:** Start with [07-java-to-rust-comparison.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/07-java-to-rust-comparison.md) and [08-debugging-java-developers.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/08-debugging-java-developers.md)

2. **Week 2: Memory & Ownership (Days 3 & 4):** Read Chapters 13 through 22. Focus heavily on stack/heap diagrams, move semantics, and lifetimes. Work through exercises `day3_*` and `day4_*`.
   - **Java Developers:** Read [07-java-developers-visual-guide.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/07-java-developers-visual-guide.md) for visual ownership understanding
   - **Type System Focus:** Read [07-deep-dive-types-java-developers.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/07-deep-dive-types-java-developers.md) for type mastery

3. **Week 3: Concurrency & Async:** Read Chapters 23 through 29. Run the multi-threaded and async exercises to master thread safety (`Send`/`Sync`) and Tokio event loops.
   - **Java Developers:** Read [08-java-developers-concurrency-guide.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/08-java-developers-concurrency-guide.md) for thread safety transition

4. **Week 4: Idiomatic Systems Design & Unsafe Deep Dive:** Read Chapters 30 through 40. Study how top Rust engineers design zero-cost typestates, sound unsafe wrappers, and FFI bindings.

---

## 🎓 Special Features for Java Developers

### Visual Learning
- **Memory diagrams** showing Java vs Rust memory models
- **Borrowing visualizations** with step-by-step ownership tracking
- **Lifetime annotations** with practical examples

### Type System Mastery
- **Algebra of Types** (Product vs Sum)
- **Newtype pattern** for zero-cost type safety
- **Phantom types** for compile-time state machines
- **Zero-sized types** for capability-based security

### Error Handling
- **Result vs Exceptions** comparison
- **thiserror** for library errors
- **anyhow** for application errors
- **Migration guide** from Java exception handling

### Concurrency
- **Send & Sync** traits as type system guarantees
- **Arc<Mutex<T>>** vs Java synchronized blocks
- **Channels** vs Java BlockingQueue
- **Tokio** vs CompletableFuture

### Debugging
- **Common error messages** with Java comparisons
- **Borrow checker debugging** workflows
- **Debugging tools** specific to Rust

---

## 📚 Why This Curriculum Is Unique

This curriculum combines Google's battle-tested internal training with **comprehensive enhancements for Java developers**:

1. **Visual Learning:** Memory diagrams and ownership visualizations help bridge the mental model gap
2. **Type System Depth:** Deep dives into algebraic types, phantom types, and zero-cost abstractions
3. **Error Handling:** Comprehensive comparison of Result vs Exceptions with migration guides
4. **Concurrency:** Thread safety explained through Java developer lens
5. **Debugging Guides:** Common issues and solutions specific to Java developers

The result is a **complete, self-contained learning path** that teaches Rust while leveraging your existing Java knowledge.