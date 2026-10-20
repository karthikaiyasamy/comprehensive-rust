# Chapter 21: Error Handling: Panics, Result, thiserror, and anyhow

Rust divides errors into two distinct categories: **unrecoverable errors** (panics) and **recoverable errors** (`Result<T, E>`). Rust does not have exceptions. Instead, recoverable errors are values that must be handled explicitly.

---

## 1. Unrecoverable Errors: Panics

A **panic** occurs when a bug is detected that cannot be safely handled at runtime:
- Explicitly via `panic!("Something went horribly wrong!");`
- Implicitly via failed assertions (`assert!`), out-of-bounds indexing (`a[10]`), or integer overflow in debug mode.

```rust
fn divide_infallible(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Denominator cannot be zero!");
    }
    a / b
}
```

### Unwinding vs. Aborting
- **Stack Unwinding (Default):** Rust walks back up the stack, cleaning up and dropping variables on each frame.
- **Immediate Abort:** If you configure `panic = "abort"` in `Cargo.toml`, Rust immediately terminates the process without unwinding. This reduces binary size and is common in embedded and systems programming:

```toml
# Cargo.toml
[profile.release]
panic = "abort"
```

---

## 2. Recoverable Errors: `Result<T, E>` & The `?` Operator

For operations that can reasonably fail (e.g., file I/O, parsing, network requests), return `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### The `?` Operator & Automatic Conversion
The `?` operator extracts `val` from `Ok(val)` or returns `Err(From::from(err))` early:

```rust
use std::fs::File;
use std::io::{self, Read};

fn load_config(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // Returns early on io::Error
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

---

## 3. Library Errors vs. Application Errors

The Rust ecosystem adopts two distinct, complementary approaches to error management:

```
+------------------------------------+------------------------------------+
|     Library Errors (`thiserror`)   |   Application Errors (`anyhow`)    |
+------------------------------------+------------------------------------+
| - For reusable libraries/crates    | - For top-level applications / CLIs|
| - Callers must match on errors     | - Callers care about error context |
| - Structured enum variants         | - Dynamic trait object:            |
| - Derives `std::error::Error`      |   `anyhow::Result<T>`              |
| - Zero runtime overhead            | - Easy `.context("...")` chains    |
+------------------------------------+------------------------------------+
```

### 3.1 Custom Library Errors with `thiserror`
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to host: {host}:{port}")]
    ConnectionFailed { host: String, port: u16 },

    #[error("Record not found for id {0}")]
    NotFound(u64),

    #[error("Underlying I/O error")]
    Io(#[from] std::io::Error), // Automatically implements From<io::Error>
}
```

### 3.2 Application Error Handling with `anyhow`
```rust
use anyhow::{Context, Result};

fn run_app() -> Result<()> {
    let config = std::fs::read_to_string("config.json")
        .context("Failed to read configuration file at config.json")?;
    println!("Config loaded: {config}");
    Ok(())
}
```

---

## 4. Exercise: Rewriting Panics to Result

### Problem Description
You are given a fragile parser that panics on bad inputs:
```rust
fn parse_key_value(input: &str) -> (&str, i32) {
    let mut parts = input.split('=');
    let key = parts.next().unwrap();
    let val = parts.next().unwrap().parse::<i32>().unwrap();
    (key, val)
}
```

Rewrite this function to return a structured `Result<(&str, i32), ParseError>` using an enum for errors (`MissingEquals`, `InvalidNumber`).

### Solution & Deep-Dive

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingEquals,
    InvalidNumber,
}

pub fn parse_key_value(input: &str) -> Result<(&str, i32), ParseError> {
    let (key, val_str) = input.split_once('=').ok_or(ParseError::MissingEquals)?;
    
    let val = val_str
        .trim()
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidNumber)?;

    Ok((key.trim(), val))
}

fn main() {
    assert_eq!(parse_key_value("timeout = 30"), Ok(("timeout", 30)));
    assert_eq!(parse_key_value("no_equals_here"), Err(ParseError::MissingEquals));
    assert_eq!(parse_key_value("port = abc"), Err(ParseError::InvalidNumber));
    println!("Result error handling verified successfully!");
}
```

---

## Summary & Next Step

You have mastered Rust's error handling model: panics, `Result`, the `?` operator, `thiserror`, and `anyhow`. Next, we explore the boundary between safe and low-level code in **Unsafe Rust Basics and Safe FFI Wrappers** in [04-unsafe-rust-basics.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/04-day4-error-testing-modules/04-unsafe-rust-basics.md).
