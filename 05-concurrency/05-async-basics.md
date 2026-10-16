# Chapter 27: Asynchronous Programming: Futures, Poll, and Runtimes

Asynchronous programming in Rust enables high-throughput concurrent I/O (handling tens of thousands of concurrent network connections) without the overhead of thousands of OS threads.

---

## 1. Cooperative Multitasking vs. Preemptive OS Threads

| Metric | Operating System Threads | Asynchronous Rust Tasks |
| :--- | :--- | :--- |
| **Model** | Preemptive (OS kernel context switches). | Cooperative (yields control at `.await` points). |
| **Memory Overhead** | Large stack per thread (typically 2–8 MB). | Tiny memory footprint (few hundred bytes per task). |
| **Concurrency Scale** | Thousands of threads before kernel exhaustion. | Millions of concurrent tasks on a single machine. |
| **Context Switch** | Heavy (registers, TLB flushes, kernel transition). | Extremely light (function return inside user-space). |

---

## 2. Futures are Lazy!

In JavaScript or C#, calling an async function immediately schedules it on a background event loop.
In Rust, **Futures are completely lazy**:

```rust
async fn fetch_data() -> String {
    // Perform async network I/O
    String::from("data")
}

fn main() {
    let future = fetch_data(); // DOES NOTHING! No network request is initiated!
}
```

> [!IMPORTANT]
> A Future does not execute any code until it is `.await`ed or passed to an executor.

---

## 3. The `Future` Trait Under the Hood

The entire async ecosystem in Rust is built on the `std::future::Future` trait:

```rust
pub trait Future {
    type Output;

    // Attempts to resolve the future to a final value
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),   // Operation complete; yields resultant value
    Pending,    // Operation incomplete; will wake current task when ready
}
```

### The Polling and Waker Cycle
1. The executor calls `poll()`.
2. If data is ready (e.g., packet received from NIC), the future returns `Poll::Ready(data)`.
3. If data is waiting on hardware (e.g., waiting for DNS), the future registers a **Waker** with the OS event loop (epoll/kqueue) and returns `Poll::Pending`.
4. When hardware signals completion, the `Waker` notifies the runtime executor to call `poll()` again.

---

## 4. The Compiler's Async State Machine

When you write an `async fn`, `rustc` compiles it into an anonymous **state machine enum**:

```rust
async fn process() {
    let x = step_one().await;
    step_two(x).await;
}
```

The compiler transforms this into:
```rust
enum ProcessStateMachine {
    Start,
    WaitingOnStepOne { /* state */ },
    WaitingOnStepTwo { x: i32, /* state */ },
    Done,
}
```
Every `.await` represents a transition point in the state machine.

---

## 5. Async Runtimes & Tokio

Rust's standard library does not ship with a built-in async runtime. Instead, the ecosystem relies on production-grade runtimes, primarily **Tokio**:

```rust
// Requires `tokio = { version = "1", features = ["full"] }` in Cargo.toml
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("Running asynchronously inside Tokio thread pool!");
        42
    });

    let result = handle.await.unwrap();
    println!("Task returned: {result}");
}
```

---

## Summary & Next Step

You have learned how Rust compiles `async fn` into lazy state machines, the `Future` polling model, and the role of the Tokio executor. Next, we master asynchronous control flow in **Async Channels, Join, and Select** in [06-async-channels-and-control-flow.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/05-concurrency/06-async-channels-and-control-flow.md).
