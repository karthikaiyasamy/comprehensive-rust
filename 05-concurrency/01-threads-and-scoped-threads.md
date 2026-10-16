# Chapter 23: Threads & Scoped Threads

Rust's concurrency model is famously described as **"Fearless Concurrency"**. The same ownership and type-system invariants that prevent memory bugs at compile time also prevent **data races** in concurrent multithreaded systems.

---

## 1. Operating System Threads (`std::thread::spawn`)

Rust maps directly to 1:1 native OS threads (pthreads on POSIX systems, Windows threads on Windows):

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Spawned thread: count {i}");
            thread::sleep(Duration::from_millis(50));
        }
        "Work finished successfully" // Return value from thread
    });

    for i in 1..=3 {
        println!("Main thread: count {i}");
        thread::sleep(Duration::from_millis(50));
    }

    // Wait for spawned thread to finish and retrieve its result:
    let result = handle.join().expect("Spawned thread panicked!");
    println!("Thread returned: {result}");
}
```

### 1.1 The `'static` Lifetime Requirement of Plain Threads
When you spawn a thread with `thread::spawn`, Rust requires that the closure satisfies a `'static` lifetime bound. 
- *Why?* The compiler cannot statically know whether the child thread will run for 1 millisecond or 10 years. Therefore, the child thread cannot borrow references from the parent thread's stack frame!
- To pass data into a spawned thread, you must transfer ownership using `move ||`:

```rust
let data = vec![1, 2, 3];
thread::spawn(move || {
    println!("Owned data in thread: {data:?}");
});
```

---

## 2. Scoped Threads (`std::thread::scope`)

Introduced in Rust 1.63, **scoped threads** solve the `'static` limitation! 
Scoped threads guarantee that all spawned threads are joined and finished **before the scope closes**. Because the child threads are guaranteed to exit before the parent stack frame unwinds, **child threads can safely borrow stack data without `Arc` or `move`!**

```rust
use std::thread;

fn main() {
    let mut numbers = vec![1, 2, 3];

    thread::scope(|scope| {
        // Thread 1 borrows numbers immutably
        scope.spawn(|| {
            println!("Thread 1 sees length: {}", numbers.len());
        });

        // Thread 2 borrows numbers immutably
        scope.spawn(|| {
            println!("Thread 2 reads first element: {}", numbers[0]);
        });
    }); // Implicit barrier: both child threads are automatically joined here!

    // Back in main thread: numbers is completely valid and accessible:
    numbers.push(4);
    println!("Main thread modified numbers: {numbers:?}");
}
```

### Scoped Threads vs. Plain Threads:
| Feature | `thread::spawn` | `thread::scope` |
| :--- | :--- | :--- |
| **Lifetime Bound** | `'static` (no borrowing from stack) | Scoped to block (can borrow local stack references) |
| **Joining** | Manual via `handle.join()` | Automatic and guaranteed when scope block exits |
| **Overhead** | Standard OS thread creation | Standard OS thread creation (zero wrapper overhead) |

---

## Summary & Next Step

You have learned how to spawn OS threads, handle return values, and use scoped threads to borrow stack references concurrently. Next, we explore message passing in **Channels and Message-Passing Concurrency** in [02-channels-and-message-passing.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/02-channels-and-message-passing.md).
