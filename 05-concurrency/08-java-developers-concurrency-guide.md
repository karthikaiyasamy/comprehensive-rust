# Chapter 30: Concurrency & Async: Java to Rust Transition Guide

This chapter helps Java developers understand Rust's concurrency model by showing side-by-side comparisons and explaining the mental model shift.

---

## 1. Threads: std::thread vs Java Thread Pool

### Java: Thread Pool
```java
ExecutorService pool = Executors.newFixedThreadPool(4);

pool.submit(() -> {
    System.out.println("Task 1");
});

pool.submit(() -> {
    System.out.println("Task 2");
});

pool.shutdown();
```

### Rust: Manual Thread Management
```rust
use std::thread;

let handles: Vec<_> = (0..4)
    .map(|i| {
        thread::spawn(move || {
            println!("Task {}", i);
        })
    })
    .collect();

// Wait for all threads to complete
for handle in handles {
    handle.join().unwrap();
}
```

### Key Difference
- **Java:** Thread pool with worker queue (reusable threads)
- **Rust:** Each `spawn` creates a new OS thread (must join to wait)

---

## 2. Shared State: Mutex & Arc

### Java: Synchronized Blocks
```java
private final Object lock = new Object();
private int counter = 0;

public void increment() {
    synchronized(lock) {
        counter++;
    }
}
```

### Rust: Arc<Mutex<T>>
```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

let handle = thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});

handle.join().unwrap();
println!("Counter: {}", *counter.lock().unwrap());
```

### Memory Layout Comparison

#### Java
```
Stack:                           Heap:
+-----------+                   +-------------+
| lock      |                   | counter     │
+-----------+                   +-------------+
      │                                │
      ▼                                ▼
+-----------+                   +-------------+
| Object    │                   | int: 0      │
+-----------+                   +-------------+
```

#### Rust
```
Stack:                           Heap:
+-----------+                   +-------------+
| counter   |                   | Mutex<i32>  │
| Arc<Mutex>│                   +-------------+
+-----------+                       │
      │                             │
      ▼                             ▼
+-----------+                   +-------------+
| counter   |                   | i32: 0      │
| Arc<Mutex>│                   +-------------+
+-----------+
```

### Why Arc<Mutex<T>> Works
- **Arc:** Reference-counted pointer (like Java's shared reference)
- **Mutex:** Runtime lock with poisoning detection
- **Together:** Thread-safe shared mutable state

---

## 3. The Send and Sync Traits: Thread Safety as Types

### Java: Runtime Checks
```java
// In Java, any object can be shared across threads
// if you use synchronization. No compile-time checks.
public class SharedCounter {
    private int count;
    
    public synchronized void increment() {
        count++;
    }
}
```

### Rust: Compile-Time Checks
```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));

// This compiles: Arc<Mutex<i32>> is Send + Sync
let handle = thread::spawn({
    let counter = Arc::clone(&counter);
    move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
});

// ❌ This does NOT compile:
// let counter = Mutex::new(0);
// let handle = thread::spawn(move || {
//     let mut num = counter.lock().unwrap(); // ERROR!
//     *num += 1;
// });

// Why? Mutex<T> is not Send unless T: Send
// And Mutex<i32> is not Sync (cannot share between threads)
```

### The Send + Sync Rules

```
Send: Can be transferred to another thread
      (Owner can move across thread boundary)

Sync: Can be shared between threads
      (Shared reference &T can be sent to another thread)

Fundamental Equivalence:
    T is Sync  ⇔  &T is Send

Standard Library Types:
┌─────────────┬──────┬──────┐
│    Type     │ Send │ Sync │
├─────────────┼──────┼──────┤
│ i32         │ Yes  │ Yes  │
│ String      │ Yes  │ Yes  │
│ Vec<T>      │ Yes  │ Yes  │
│ Arc<T>      │ Yes  │ Yes* │
│ Mutex<T>    │ Yes* │ Yes  │
│ RwLock<T>   │ Yes* │ Yes  │
│ RefCell<T>  │ No   │ No   │
│ Rc<T>       │ No   │ No   │
└─────────────┴──────┴──────┘
* Requires T: Send
```

### Why Rc and RefCell Don't Work
```rust
use std::rc::Rc;

let counter = Rc::new(0);

// ❌ ERROR: Rc<T> is not Send
let handle = thread::spawn({
    let counter = Rc::clone(&counter);
    move || {
        println!("{}", counter);
    }
});

// Why? Rc uses non-atomic reference counting!
// If two threads increment at the same time, race condition!
```

---

## 4. Channels: MPSC vs Java BlockingQueue

### Java: BlockingQueue
```java
BlockingQueue<String> queue = new LinkedBlockingQueue<>();

// Producer
new Thread(() -> {
    queue.put("Hello");
}).start();

// Consumer
new Thread(() -> {
    String msg = queue.take();
    System.out.println(msg);
}).start();
```

### Rust: std::sync::mpsc
```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

// Producer
let tx_clone = tx.clone();
thread::spawn(move || {
    tx_clone.send("Hello".to_string()).unwrap();
});

// Consumer
let received = rx.recv().unwrap();
println!("Received: {}", received);
```

### Mpsc Features
- **Multiple Producer, Single Consumer**
- **Unbounded** (default): Grows until memory exhausted
- **Bounded**: Use `sync_channel(capacity)` for backpressure

### Example: Bounded Channel with Backpressure
```rust
use std::sync::mpsc;
use std::thread;

// Bounded channel with capacity 2
let (tx, rx) = mpsc::sync_channel(2);

// Producer
thread::spawn(move || {
    for i in 0..10 {
        tx.send(i).unwrap(); // Blocks if channel is full!
        println!("Sent {}", i);
    }
});

// Consumer (slower)
thread::spawn(move || {
    for _ in 0..10 {
        thread::sleep(std::time::Duration::from_millis(500));
        let received = rx.recv().unwrap();
        println!("Received {}", received);
    }
});
```

---

## 5. Async/Await: Tokio vs CompletableFuture

### Java: CompletableFuture
```java
CompletableFuture<String> future = CompletableFuture
    .supplyAsync(() -> {
        return performTask();
    })
    .thenApply(result -> {
        return process(result);
    })
    .exceptionally(ex -> {
        return "Error: " + ex.getMessage();
    });

String result = future.get(); // Blocks
```

### Rust: Tokio
```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = perform_task()
        .await
        .and_then(process)
        .unwrap_or("Error".to_string());
    
    println!("Result: {}", result);
}

async fn perform_task() -> Result<String, String> {
    // Async operation
    Ok("task completed".to_string())
}

async fn process(result: String) -> Result<String, String> {
    // Process result
    Ok(result)
}
```

### Mental Model: Futures as State Machines

#### Java CompletableFuture
```
Future: Promise<T>
- Composable with .thenApply(), .thenCompose()
- Blocking: .get()
- Non-blocking: .thenAccept()
```

#### Rust Future
```
Future: Future<Output = T>
- Must be .await'ed
- Zero-sized type (state machine)
- Composable with tokio::join!(), select!()
```

### Why Rust's Approach Is Different
- **Java:** Futures are objects (heap allocated)
- **Rust:** Futures are zero-sized state machines (compile-time generated)

---

## 6. Real-World Example: Concurrent Web Crawler

### Java Version
```java
public class Crawler {
    private final ExecutorService executor = Executors.newFixedThreadPool(10);
    private final Set<String> visited = ConcurrentHashMap.newKeySet();
    
    public void crawl(String url) {
        if (visited.contains(url)) return;
        
        visited.add(url);
        String html = fetch(url);
        
        List<String> links = extractLinks(html);
        
        for (String link : links) {
            executor.submit(() -> crawl(link));
        }
    }
}
```

### Rust Version
```rust
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

type Visited = Arc<Mutex<HashSet<String>>>;

fn crawl(url: String, visited: Visited) {
    // Check if already visited
    {
        let v = visited.lock().unwrap();
        if v.contains(&url) {
            return;
        }
    }
    
    // Mark as visited
    {
        let mut v = visited.lock().unwrap();
        v.insert(url.clone());
    }
    
    // Fetch URL (simplified)
    let html = fetch(&url);
    
    // Extract links and spawn new tasks
    let links = extract_links(&html);
    
    for link in links {
        let visited_clone = Arc::clone(&visited);
        thread::spawn(move || {
            crawl(link, visited_clone);
        });
    }
}

fn fetch(url: &str) -> String {
    // Simulated HTTP request
    format!("HTML content of {}", url)
}

fn extract_links(html: &str) -> Vec<String> {
    // Simulated link extraction
    vec!["http://example.com/1".to_string(), "http://example.com/2".to_string()]
}

fn main() {
    let visited = Arc::new(Mutex::new(HashSet::new()));
    crawl("http://example.com".to_string(), visited);
}
```

### Key Differences

| Java | Rust |
|------|------|
| ExecutorService | thread::spawn |
| ConcurrentHashMap | Arc<Mutex<T>> |
| Volatile/Atomic | Arc (atomic ref counting) |
| Blocking .get() | JoinHandle.join() |

---

## 7. Common Pitfalls for Java Developers

### Pitfall 1: Not Joining Threads
```rust
// ❌ Wrong: thread exits immediately
thread::spawn(|| {
    println!("Hello from thread");
});

// ✅ Right: wait for thread to complete
let handle = thread::spawn(|| {
    println!("Hello from thread");
});
handle.join().unwrap();
```

### Pitfall 2: Forgetting Arc
```rust
let counter = Mutex::new(0);

// ❌ Wrong: Mutex is not Send
thread::spawn(move || {
    let mut num = counter.lock().unwrap();
    *num += 1;
});

// ✅ Right: Arc for shared ownership
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});
```

### Pitfall 3: Mutex Poisoning
```rust
let mutex = Arc::new(Mutex::new(0));

let handle = {
    let mutex = Arc::clone(&mutex);
    thread::spawn(move || {
        let mut num = mutex.lock().unwrap();
        *num += 1;
        panic!("Something went wrong!");
    })
};

handle.join().unwrap(); // Panics: mutex is poisoned!

// ✅ Handle poisoned mutex
match mutex.lock() {
    Ok(num) => println!("Value: {}", *num),
    Err(poisoned) => {
        println!("Mutex was poisoned, but we can recover: {}", poisoned.get_ref());
    }
}
```

---

## 8. Debugging Concurrency Issues

### Tool 1: `RUST_LOG`
```bash
RUST_LOG=trace cargo run
```

### Tool 2: `cargo FlameGraph`
```bash
cargo install flamegraph
cargo flamegraph
```

### Tool 3: Clippy
```bash
cargo clippy -- -D warnings
```

### Common Error Messages

#### "Thread 'main' panicked at 'lock poisoned'"
- Another thread panicked while holding the lock
- Use `.unwrap_or_else(|e| e.get_ref())` to recover

#### "Cannot send data across thread boundaries"
- Type doesn't implement `Send`
- Wrap in `Arc<Mutex<T>>` if shared mutable state needed

#### "borrowed value does not live long enough"
- Reference outlives the data it points to
- Use `Arc` or `'static` lifetime

---

## 9. Best Practices

### 1. Use Arc<Mutex<T>> for Shared Mutable State
```rust
let shared = Arc::new(Mutex::new(Vec::new()));

// Clone Arc for each thread
let shared_clone = Arc::clone(&shared);
thread::spawn(move || {
    let mut vec = shared_clone.lock().unwrap();
    vec.push(42);
});
```

### 2. Use Channels for Message Passing
```rust
let (tx, rx) = mpsc::channel();

// Producer
thread::spawn(move || {
    for i in 0..10 {
        tx.send(i).unwrap();
    }
});

// Consumer
for value in rx {
    println!("Received: {}", value);
}
```

### 3. Prefer Async for I/O-bound Tasks
```rust
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let shared = Arc::new(Mutex::new(0));
    
    let shared_clone = Arc::clone(&shared);
    tokio::spawn(async move || {
        let mut num = shared_clone.lock().await;
        *num += 1;
    });
}
```

---

## 10. Summary: Mental Model Mapping

| Concept | Java | Rust |
|---------|------|------|
| **Thread Creation** | `new Thread().start()` | `thread::spawn()` |
| **Thread Pool** | `ExecutorService` | `tokio::spawn()` |
| **Shared State** | `synchronized` blocks | `Arc<Mutex<T>>` |
| **Thread Safety** | Runtime checks | Compile-time `Send` + `Sync` |
| **Channels** | `BlockingQueue` | `std::sync::mpsc` |
| **Async** | `CompletableFuture` | `tokio::spawn()` + `.await` |

### Quick Reference: Thread-Safe Types

| Type | Send | Sync | Use Case |
|------|------|------|----------|
| `Arc<Mutex<T>>` | Yes* | Yes | Shared mutable state |
| `Arc<RwLock<T>>` | Yes* | Yes | Many readers, few writers |
| `mpsc::Sender<T>` | Yes | Yes | Message passing |
| `mpsc::Receiver<T>` | Yes | No | Single consumer |
| `JoinHandle<T>` | Yes | No | Wait for thread completion |

*Requires `T: Send`

---

## 11. Practice Exercises

### Exercise 1: Concurrent Counter
Implement a concurrent counter using Arc<Mutex<i32>>.
- Spawn 10 threads
- Each thread increments 1000 times
- Verify final count is 10000

**Solution:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let result = counter.lock().unwrap();
    assert_eq!(*result, 10000);
    println!("Final count: {}", *result);
}
```

### Exercise 2: Concurrent Prime Sieve
Implement a concurrent prime number sieve using channels.
- Worker threads check divisibility
- Main thread collects results

**Hint:** Use `mpsc::channel()` for result collection.

---

## 12. Final Advice

### Don't Fight the Type System
If the compiler rejects your code, it's catching a real concurrency bug. Trust it.

### Use the Right Tool
- **Arc<Mutex<T>>:** For shared mutable state
- **Channels:** For message passing
- **Tokio:** For async I/O

### Read the Documentation
- [std::sync documentation](https://doc.rust-lang.org/std/sync/)
- [Tokio documentation](https://tokio.rs/)

### Practice, Practice, Practice
Concurrency is hard. Start small, build up.

---

**Remember:** Every Java developer has struggled with Rust's concurrency model. But once you understand the `Send` and `Sync` traits, you'll have a level of thread safety that's impossible to achieve in Java.
