# Chapter 28: Async Control Flow: Channels, Join, and Select

Writing asynchronous code requires coordinating concurrent operations: executing tasks in parallel, waiting for all operations to finish, or racing multiple operations against a timeout.

---

## 1. Asynchronous Channel Types

The `tokio::sync` module provides specialized channel types tailored for async architectures:

```
+---------------+---------------------------------------------------------------+
| Channel Type  | Description & Typical Use Case                                |
+---------------+---------------------------------------------------------------+
| `mpsc`        | Multi-producer, single-consumer. General task queues.         |
+---------------+---------------------------------------------------------------+
| `oneshot`     | Single-producer, single-consumer. For sending a single result |
|               | back from a background worker task.                           |
+---------------+---------------------------------------------------------------+
| `broadcast`   | Multi-producer, multi-consumer. Chat rooms, event buses       |
|               | (every subscriber receives all messages).                     |
+---------------+---------------------------------------------------------------+
| `watch`       | Single-producer, multi-consumer. Configuration updates        |
|               | (subscribers always inspect the latest updated value).        |
+---------------+---------------------------------------------------------------+
```

---

## 2. Running Concurrently with `tokio::join!`

When you have multiple futures that can proceed simultaneously and you need all of their results:

```rust
async fn fetch_user_avatar() -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    "avatar.png".to_string()
}

async fn fetch_user_posts() -> Vec<String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    vec!["Post 1".to_string(), "Post 2".to_string()]
}

#[tokio::main]
async fn main() {
    // Executes both futures concurrently on the same task!
    // Total elapsed time is ~150ms instead of 250ms!
    let (avatar, posts) = tokio::join!(fetch_user_avatar(), fetch_user_posts());

    println!("Avatar: {avatar}, Posts: {posts:?}");
}
```

---

## 3. Multiplexing & Racing with `tokio::select!`

The `tokio::select!` macro waits on multiple branch futures simultaneously, executing the branch that resolves **first** and cancelling the others:

```rust
use tokio::time::{sleep, Duration};

async fn long_computation() -> &'static str {
    sleep(Duration::from_millis(500)).await;
    "Computation finished"
}

#[tokio::main]
async fn main() {
    tokio::select! {
        result = long_computation() => {
            println!("Got result: {result}");
        }
        _ = sleep(Duration::from_millis(200)) => {
            println!("Timed out after 200ms!");
        }
    }
}
```

### 3.1 Looping with `select!` (Server Event Loop)
```rust
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("ping").await.unwrap();
    });

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                println!("Received message: {msg}");
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl+C received. Shutting down.");
                break;
            }
        }
    }
}
```

---

## Summary & Next Step

You have learned Tokio channel architectures, concurrent execution with `join!`, and multiplexing with `select!`. Next, we explore critical traps and edge cases in **Async Pitfalls, Pinning, and Cancellation Safety** in [07-async-pitfalls-and-cancellation.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/05-concurrency/07-async-pitfalls-and-cancellation.md).
