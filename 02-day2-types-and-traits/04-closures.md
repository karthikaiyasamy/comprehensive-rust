# Chapter 10: Closures and Environment Capturing

Closures are anonymous functions that can capture values from the scope in which they are defined. They are ubiquitous in Rust, powering iterators, thread spawning, and event-driven callbacks.

---

## 1. Closure Syntax & Type Inference

Closures use pipes `|...|` to enclose parameters:

```rust
fn main() {
    // Explicit type annotations (rarely needed)
    let add_one = |x: u32| -> u32 { x + 1 };

    // Concise syntax with full type inference:
    let square = |x| x * x;

    println!("square(4) = {}", square(4));
}
```

> [!NOTE]
> Closure parameter types are inferred from their **first use**. Once inferred, the closure is permanently locked to that concrete type.

---

## 2. Environment Capturing

Unlike regular functions, closures can access variables from their enclosing environment:

```rust
fn main() {
    let multiplier = 3;
    let multiply = |x| x * multiplier; // Captures `multiplier` from outer scope

    println!("Result: {}", multiply(10)); // 30
}
```

### How Capturing Works Under the Hood
The compiler automatically generates an anonymous, hidden `struct` representing the closure, storing the captured variables as fields.

---

## 3. The Three Closure Traits (`Fn`, `FnMut`, `FnOnce`)

Rust categorizes closures into three traits based on **how they interact with their captured environment**:

```
                  +-------------------+
                  |      FnOnce       |  Can be called at least once.
                  +---------^---------+  Consumes captured variables.
                            |
                  +---------+---------+
                  |       FnMut       |  Can be called multiple times.
                  +---------^---------+  Can mutate captured variables.
                            |
                  +---------+---------+
                  |        Fn         |  Can be called multiple times.
                  +-------------------+  Only reads captured variables.
```

### 3.1 `Fn`: Borrows Immutably
The closure only requires shared references (`&T`) to captured variables. It can be called repeatedly and concurrently:

```rust
let name = String::from("Rust");
let print_name = || println!("Name: {name}"); // Implements Fn
print_name();
print_name();
```

### 3.2 `FnMut`: Borrows Mutably
The closure requires mutable references (`&mut T`) to mutate captured variables:

```rust
let mut count = 0;
let mut increment = || {
    count += 1;
    println!("Count: {count}");
}; // Implements FnMut
increment();
increment();
```

### 3.3 `FnOnce`: Takes Ownership (Moves)
The closure consumes the captured variable, moving it out of the environment. It can only be called **once**:

```rust
let data = vec![1, 2, 3];
let consume_data = || {
    drop(data); // `data` is moved and destroyed
}; // Implements FnOnce only

consume_data();
// consume_data(); // COMPILE ERROR: use of moved value `consume_data`
```

---

## 4. The `move` Keyword

By default, closures capture variables by the least restrictive reference possible. Using the `move` keyword forces the closure to **take full ownership** of captured variables:

```rust
fn make_greeter(greeting: String) -> impl Fn(&str) {
    // `move` transfers ownership of `greeting` into the closure struct
    move |name| println!("{greeting}, {name}!")
}

fn main() {
    let greeter = make_greeter(String::from("Good morning"));
    greeter("Alice");
    greeter("Bob");
}
```

> [!IMPORTANT]
> The `move` keyword is required when passing closures to background threads (e.g., `std::thread::spawn`), because threads can outlive the stack frame where the closure was created.

---

## 5. Exercise: Log Filter with Closures

### Problem Description
Implement a filter utility for log entries. 
- A log entry has a format `(level: &str, message: &str)`.
- Write a function `filter_logs` that takes a slice of log tuples and a closure predicate `P: Fn(&str) -> bool` that filters messages by content.

### Starter Code
```rust
struct LogEntry<'a> {
    level: &'a str,
    message: &'a str,
}

fn filter_logs<'a, P>(logs: &[LogEntry<'a>], predicate: P) -> Vec<&'a str>
where
    P: Fn(&str) -> bool,
{
    todo!("Filter log messages using predicate")
}
```

---

### Solution & Deep-Dive

```rust
struct LogEntry<'a> {
    level: &'a str,
    message: &'a str,
}

fn filter_logs<'a, P>(logs: &[LogEntry<'a>], predicate: P) -> Vec<&'a str>
where
    P: Fn(&str) -> bool,
{
    let mut matching_messages = Vec::new();
    for entry in logs {
        if predicate(entry.message) {
            matching_messages.push(entry.message);
        }
    }
    matching_messages
}

fn main() {
    let logs = [
        LogEntry { level: "INFO", message: "System started" },
        LogEntry { level: "ERROR", message: "Failed to connect to database" },
        LogEntry { level: "WARN", message: "Disk usage above 85%" },
        LogEntry { level: "ERROR", message: "Database timeout" },
    ];

    // Filter using closure that captures `target_word`
    let target_word = "database";
    let database_errors = filter_logs(&logs, |msg| {
        msg.to_lowercase().contains(target_word)
    });

    assert_eq!(
        database_errors,
        vec!["Failed to connect to database", "Database timeout"]
    );
    println!("Log filter verified successfully!");
}
```

### Key Technical Insights:
- The closure `|msg| msg.to_lowercase().contains(target_word)` captures `target_word` by shared reference `&target_word`. Because it only reads `target_word`, it automatically implements `Fn(&str) -> bool`.
- The lifetime `'a` ensures the returned string slices remain valid for as long as the original log data lives.

---

## Summary & Next Step

You have learned closures, type inference, environment capturing, `Fn` / `FnMut` / `FnOnce`, and the `move` keyword. Next, we master core data structures in **Standard Library Types: Option, Result, Vec, String, and HashMap** in [05-standard-library-types.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/05-standard-library-types.md).
