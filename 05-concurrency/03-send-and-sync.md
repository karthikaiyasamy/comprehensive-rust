# Chapter 25: The Thread-Safety Contract: Send and Sync

In Rust, thread safety is not a matter of runtime vigilance or developer discipline—it is an **invariable property of the type system** governed by two core marker traits: `Send` and `Sync`.

---

## 1. The Core Definitions

```
+------------------------------------+------------------------------------+
|               `Send`               |               `Sync`               |
+------------------------------------+------------------------------------+
| A type `T` is `Send` if ownership  | A type `T` is `Sync` if it is safe |
| can be transferred to another      | to share references (`&T`) across  |
| thread.                            | multiple threads concurrently.     |
+------------------------------------+------------------------------------+
```

### The Fundamental Equivalence:
$$\mathbf{T \text{ is } Sync \iff \&T \text{ is } Send}$$

If a shared reference `&T` can be safely sent to another thread, then multiple threads can read `&T` simultaneously, which by definition means `T` is thread-safe (`Sync`).

---

## 2. Auto Traits (Built-in Derivation)

`Send` and `Sync` are **auto traits**:
- You do not need to write `#[derive(Send, Sync)]`.
- The compiler automatically implements `Send` and `Sync` for any struct or enum whose fields all implement `Send` and `Sync`.
- Primitives (`i32`, `f64`, `bool`, `&str`), standard collections (`Vec<T>`, `String`, `HashMap<K, V>`), and plain structs are automatically `Send` and `Sync`.

---

## 3. Types that are NOT Thread-Safe

To understand thread safety, study the types that explicitly opt out:

```
+---------------+---------------+---------------+---------------------------------------+
| Type          | Send?         | Sync?         | Why it is NOT Thread-Safe             |
+---------------+---------------+---------------+---------------------------------------+
| `Rc<T>`       | **No**        | **No**        | Reference counter uses non-atomic     |
|               |               |               | arithmetic (`x++`). Data race!        |
+---------------+---------------+---------------+---------------------------------------+
| `RefCell<T>`  | **Yes**       | **No**        | Borrow counter uses non-atomic        |
|               |               |               | integers. Cannot be read concurrently.|
+---------------+---------------+---------------+---------------------------------------+
| `Cell<T>`     | **Yes**       | **No**        | Unsynchronized interior mutability.   |
+---------------+---------------+---------------+---------------------------------------+
| `*const T`    | **No**        | **No**        | Raw pointers have no lifetime or      |
| `*mut T`      |               |               | aliasing guarantees.                  |
+---------------+---------------+---------------+---------------------------------------+
```

### What Happens if You Try to Send `Rc<T>` Across Threads?
```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let rc = Rc::new(10);
    // thread::spawn(move || {
    //     println!("{rc}");
    // });
    // COMPILE ERROR: `Rc<i32>` cannot be sent between threads safely
    // the trait `Send` is not implemented for `Rc<i32>`
}
```
The compiler catches data races **before code ever runs!**

---

## 4. Upgrading to Thread-Safe Equivalents

Rust provides direct drop-in thread-safe equivalents for single-threaded abstractions:

| Single-Threaded Type | Multi-Threaded Equivalent | Synchronization Mechanism |
| :--- | :--- | :--- |
| `Rc<T>` | `Arc<T>` | Atomic Reference Counting (`fetch_add`, `fetch_sub`). |
| `RefCell<T>` | `Mutex<T>` / `RwLock<T>` | OS-level mutual exclusion lock / read-write lock. |
| `Cell<T>` | `AtomicBool`, `AtomicIsize`, etc. | CPU hardware atomic instructions. |

---

## 5. Implementing `Send` and `Sync` Manually

When writing low-level systems code that manages raw pointers (e.g., custom allocators, thread pools, or hardware ring buffers), you must implement `Send` and `Sync` using `unsafe`:

```rust
struct MyRawBuffer {
    ptr: *mut u8,
    len: usize,
}

// Safety: We guarantee that MyRawBuffer exclusively owns the heap buffer
// and no other thread accesses `ptr` simultaneously without synchronization.
unsafe impl Send for MyRawBuffer {}
unsafe impl Sync for MyRawBuffer {}
```

---

## Summary & Next Step

You have learned the formal definitions of `Send` and `Sync`, why `Rc` and `RefCell` are forbidden across threads, and how the compiler prevents data races at compile time. Next, we explore shared state concurrency: **Arc, Mutex, RwLock, and Deadlock Prevention** in [04-shared-state-concurrency.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/05-concurrency/04-shared-state-concurrency.md).
