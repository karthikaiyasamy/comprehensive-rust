# Chapter 24: Message Passing & MPSC Channels

Rust provides channel-based message passing in `std::sync::mpsc` (Multiple Producer, Single Consumer). This allows threads to communicate by sending messages rather than sharing state behind locks.

---

## 1. Unbounded Channels (`mpsc::channel`)

In an unbounded channel, the buffer grows as needed to accommodate sent messages:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Spawn a producer thread
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "worker", "thread"];
        for msg in messages {
            tx.send(msg).unwrap(); // Send message across channel
        }
    }); // tx is dropped here when the thread terminates

    // Consumer reads messages:
    // Calling `for msg in rx` blocks until all senders are dropped!
    for received in rx {
        println!("Received: {received}");
    }
}
```

### 1.1 Multiple Producers (Cloning the Sender)
Because the channel is **MPSC**, you can clone the `Sender` (`tx`) to send from multiple threads:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=3 {
        let thread_tx = tx.clone(); // Clone sender for each thread
        thread::spawn(move || {
            thread_tx.send(format!("Message from worker #{i}")).unwrap();
        });
    }

    // Drop original sender so receiver knows when all worker threads are done:
    drop(tx);

    for msg in rx {
        println!("{msg}");
    }
}
```

---

## 2. Bounded Channels & Backpressure (`mpsc::sync_channel`)

An unbounded channel can exhaust system memory if a fast producer generates messages faster than a slow consumer can process them.
A **bounded synchronous channel** (`sync_channel`) enforces a maximum buffer capacity, providing **backpressure**:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Buffer capacity of exactly 2 items
    let (tx, rx) = mpsc::sync_channel(2);

    thread::spawn(move || {
        for i in 1..=5 {
            println!("Producer sending item {i}...");
            tx.send(i).unwrap(); // Blocks on item 3 until consumer reads item 1!
            println!("Item {i} placed in channel buffer.");
        }
    });

    thread::sleep(Duration::from_millis(500));
    for item in rx {
        println!("Consumer read item: {item}");
        thread::sleep(Duration::from_millis(100));
    }
}
```

### Rendezvous Channel (`sync_channel(0)`)
A synchronous channel with a capacity of 0 creates a **rendezvous channel**: the sender blocks until the receiver is actively waiting to take the item, forcing strict handoff synchronization between threads.

---

## 3. Disconnection & Ownership Guarantees

Rust channels provide exceptional ownership safety when disconnects occur:
1. **Receiver Dropped:** If the consumer exits or drops `rx`, subsequent calls to `tx.send(value)` fail and return `Err(SendError(value))`. The channel hands ownership of the unsent item back to the caller!
2. **All Senders Dropped:** When every clone of `tx` leaves scope, `rx.recv()` immediately unblocks and returns `Err(RecvError)`, allowing consumer loops to terminate cleanly.

---

## Summary & Next Step

You have learned unbounded and bounded MPSC channels, backpressure, and graceful disconnection semantics. Next, we explore the type-level foundations of thread safety in **The Send and Sync Marker Traits** in [03-send-and-sync.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/05-concurrency/03-send-and-sync.md).
