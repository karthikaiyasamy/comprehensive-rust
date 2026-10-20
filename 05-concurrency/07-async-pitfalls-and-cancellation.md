# Chapter 29: Async Pitfalls, Pinning, and Cancellation Safety

Asynchronous Rust provides extraordinary performance, but introduces distinct traps that do not exist in synchronous code: executor starvation, self-referential pointer moves, and cancellation data loss.

---

## 1. Pitfall 1: Blocking the Executor

An async runtime runs on a small pool of worker threads (typically one thread per CPU core). Tasks are expected to yield cooperatively via `.await`.

> [!CAUTION]
> If you call a blocking synchronous function (such as `std::thread::sleep`, synchronous `std::fs::read`, or a heavy CPU computation) inside an `async fn`, you **block that OS thread completely**. All other hundreds or thousands of async tasks assigned to that thread are frozen!

```rust
// WRONG: Starves the Tokio thread pool!
async fn bad_handler() {
    std::thread::sleep(std::time::Duration::from_secs(5)); // BLOCKS WORKER THREAD!
}

// CORRECT: Uses non-blocking timer that yields cooperatively:
async fn good_handler() {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

// CORRECT: Offload heavy CPU or legacy blocking I/O to a dedicated blocking pool:
async fn cpu_heavy_handler() {
    let result = tokio::task::spawn_blocking(|| {
        // Runs on separate thread pool designed for blocking tasks
        compute_heavy_hashes()
    }).await.unwrap();
}
```

---

## 2. Pitfall 2: Pinning & Self-Referential Futures

When you hold a reference across an `.await` point, the compiled state machine becomes **self-referential**:

```rust
async fn example() {
    let data = [1, 2, 3];
    let slice = &data[..]; // Pointer to `data`
    async_io().await;     // Both `data` and `slice` stored inside the Future struct!
    println!("{slice:?}");
}
```

```
Future Struct in Memory:
+------------------------------------+
| data: [1, 2, 3]                    | <---+ (Self-referential pointer)
| slice: &[i32] ---------------------+-----+
| state: WaitingOnAsyncIo            |
+------------------------------------+
```

### The Move Problem:
If this struct is moved in memory (e.g., passed by value or reallocated in a vector), the `slice` pointer continues pointing to the **old memory address**, causing a use-after-free or dangling pointer!
- **Solution:** `Pin<P>` wraps a pointer and guarantees that **the underlying value will never be moved in memory** until it is dropped.
- Types that are safe to move implement the `Unpin` auto trait. Self-referential futures do **not** implement `Unpin`.

---

## 3. Pitfall 3: Cancellation Safety

When `tokio::select!` executes, whichever branch completes first wins. **All other branches are cancelled by dropping their futures immediately**.

### What Makes a Future Cancellation-Unsafe?
An operation is **cancellation-unsafe** if dropping it midway leaves state corrupted or drops incoming data:
- `tokio::io::AsyncReadExt::read`: If `read` fills half a buffer and is cancelled, those bytes are permanently discarded from the socket.
- `tokio::sync::mpsc::Receiver::recv`: **Cancellation-safe**. If cancelled before a message arrives, the message remains safely in the queue.

---

## 4. Exercise: Multi-Client Broadcast Chat Server

### Problem Description
Build an asynchronous broadcast chat server using Tokio:
- Clients connect via TCP.
- When any client sends a message, it is broadcast to all other connected clients using `tokio::sync::broadcast`.
- Handle client disconnections gracefully.

### Solution & Deep-Dive

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Chat server listening on 127.0.0.1:8080");

    // Create broadcast channel with capacity 16
    let (tx, _rx) = broadcast::channel(16);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client connected: {addr}");

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    // Read incoming message from this client
                    bytes_read = reader.read_line(&mut line) => {
                        if bytes_read.unwrap_or(0) == 0 {
                            println!("Client {addr} disconnected.");
                            break;
                        }
                        // Broadcast message to all subscribers
                        let _ = tx.send(format!("[{addr}]: {line}"));
                        line.clear();
                    }
                    // Receive broadcast from other clients and write to socket
                    Ok(msg) = rx.recv() => {
                        if writer.write_all(msg.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });
    }
}
```

---

## Summary & Next Step

You have completed **Module 5: Concurrency in Depth**! You understand OS threads, scoped threads, message passing, `Send`/`Sync`, `Arc`/`Mutex`, async runtimes, `join!`/`select!`, pinning, and cancellation safety.

Proceed to **Module 6: Idiomatic Rust & Advanced API Design** starting with [06-idiomatic-rust/01-api-design-and-documentation.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/06-idiomatic-rust/01-api-design-and-documentation.md).
