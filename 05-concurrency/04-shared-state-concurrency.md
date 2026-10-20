# Chapter 26: Shared State Concurrency: Arc, Mutex, and RwLock

While channels emphasize message passing, many high-performance algorithms require multiple threads to read and write to the same shared memory location. Rust makes shared-state concurrency safe through the combination of `Arc<T>` and `Mutex<T>`.

---

## 1. The Dynamic Duo: `Arc<Mutex<T>>`

To share mutable data across multiple threads, you wrap the data in a `Mutex` to synchronize access, and wrap the `Mutex` in an `Arc` to share ownership:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 1. Acquire mutex lock (blocks until available)
            let mut num = counter_clone.lock().unwrap();

            // 2. Mutate protected data
            *num += 1;

            // 3. Lock is automatically released when `num` (MutexGuard) leaves scope!
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap()); // 10
}
```

---

## 2. RAII Lock Guards & Lock Poisoning

### 2.1 RAII Lock Guards
In C/C++, you must remember to call `pthread_mutex_unlock()`. If a function returns early or throws an exception, the mutex remains locked forever (deadlock).
In Rust:
- Calling `mutex.lock()` yields a `MutexGuard<'a, T>`.
- `MutexGuard` implements `Deref` and `DerefMut` to provide transparent access to the inner data `T`.
- When `MutexGuard` goes out of scope, its `Drop` implementation **automatically releases the lock**.

### 2.2 Lock Poisoning
If a thread panics while holding a mutex lock, the data inside might be partially modified and in an inconsistent state.
- Rust flags the mutex as **poisoned**.
- Subsequent calls to `.lock()` return `Err(PoisonError)`.
- You can either propagate the panic with `.unwrap()`, or intentionally recover the data using `.unwrap_or_else(|e| e.into_inner())`.

---

## 3. `RwLock<T>`: Multiple Readers or One Writer

When data is read frequently but updated rarely, a `Mutex` creates unnecessary contention. `RwLock` allows:
- **Unlimited concurrent readers** (`.read().unwrap()`), OR
- **Exactly one exclusive writer** (`.write().unwrap()`).

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// Multiple concurrent readers:
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    println!("Readers see: {r1} and {r2}");
} // Reads released

// Single exclusive writer:
{
    let mut w = lock.write().unwrap();
    *w += 10;
} // Write released
```

---

## 4. Exercise: The Dining Philosophers Problem

### Problem Description
Five philosophers sit around a circular table. Between each pair of philosophers is a single fork (`Mutex<()>`). A philosopher needs **both the left and right forks** to eat. 
If every philosopher simultaneously picks up their left fork, the program deadlocks!

Implement a deadlock-free dining philosophers simulation using `Arc<Mutex<()>>` by enforcing an asymmetric pick-up order or ranking forks by memory address.

### Solution & Deep-Dive

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: &'static str,
    left_fork: usize,
    right_fork: usize,
}

impl Philosopher {
    fn new(name: &'static str, left: usize, right: usize) -> Self {
        Philosopher {
            name,
            left_fork: left,
            right_fork: right,
        }
    }

    fn eat(&self, forks: &[Arc<Mutex<()>>]) {
        // Deadlock prevention: always acquire the lower-indexed fork first!
        let first_fork = usize::min(self.left_fork, self.right_fork);
        let second_fork = usize::max(self.left_fork, self.right_fork);

        let _guard1 = forks[first_fork].lock().unwrap();
        println!("{} picked up fork {first_fork}", self.name);

        let _guard2 = forks[second_fork].lock().unwrap();
        println!("{} picked up fork {second_fork} and started eating...", self.name);

        thread::sleep(Duration::from_millis(10));
        println!("{} finished eating and put down both forks.", self.name);
    }
}

fn main() {
    let forks = (0..5)
        .map(|_| Arc::new(Mutex::new(())))
        .collect::<Vec<_>>();

    let philosophers = vec![
        Philosopher::new("Aristotle", 0, 1),
        Philosopher::new("Plato", 1, 2),
        Philosopher::new("Descartes", 2, 3),
        Philosopher::new("Spinoza", 3, 4),
        Philosopher::new("Kant", 4, 0), // Note: 4 and 0
    ];

    let mut handles = vec![];
    for p in philosophers {
        let forks_clone = forks.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..3 {
                p.eat(&forks_clone);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("All philosophers dined without deadlocks!");
}
```

### Why Resource Ordering Prevents Deadlock:
By forcing Kant (who sits between fork 4 and fork 0) to pick up fork 0 before fork 4, a circular wait condition is mathematically impossible. This is Dijkstra's **Resource Hierarchy Solution**.

---

## Summary & Next Step

You have mastered `Arc`, `Mutex`, `RwLock`, lock poisoning, and deadlock prevention. Next, we transition from OS threads to asynchronous programming: **Async Basics, Futures, and Runtimes** in [05-async-basics.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/05-concurrency/05-async-basics.md).
