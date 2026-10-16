# Chapter 30: Foundations of API Design & Documentation

Writing idiomatic Rust requires adhering to established community conventions and the **Rust API Guidelines**. An idiomatic API is predictable, ergonomic, self-documenting, and catches errors at compile time.

---

## 1. Meaningful Documentation Comments

Rust provides first-class documentation tools built into `cargo doc`:
- `///`: Documents the item immediately following the comment (functions, structs, traits).
- `//!`: Documents the enclosing module or crate root.

```rust
//! # Configuration Management Module
//!
//! Provides utilities for loading and parsing application config files.

/// A user account within the authentication service.
pub struct User {
    pub id: u64,
    pub username: String,
}

impl User {
    /// Creates a new user with the specified username.
    ///
    /// # Arguments
    ///
    /// * `username` - The unique username for the user.
    ///
    /// # Panics
    ///
    /// Panics if `username` contains control characters or is empty.
    ///
    /// # Errors
    ///
    /// Returns `Err(AuthError)` if the database connection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::User;
    /// let user = User::new("karthik").unwrap();
    /// assert_eq!(user.username, "karthik");
    /// ```
    pub fn new(username: &str) -> Result<Self, AuthError> {
        // Implementation
    }
}
```

### Standard Documentation Sections
The Rust community expects standard markdown headers in doc comments:
1. `# Examples`: Executable doc-tests that run during `cargo test`.
2. `# Panics`: Scenarios where the function can panic.
3. `# Errors`: Possible error variants returned if the function returns `Result`.
4. `# Safety`: Mandatory for any `unsafe fn`, explaining the caller's preconditions.

---

## 2. Predictable API: Standard Naming Conventions

Rust adheres to strict naming conventions specified in RFC 430:

```
+------------------------------------+------------------------------------+
| Convention                         | Meaning / Cost                     |
+------------------------------------+------------------------------------+
| `as_` (e.g., `as_bytes`, `as_str`) | Borrowed -> Borrowed (free/cheap)  |
| `to_` (e.g., `to_string`, `to_vec`)| Borrowed -> Owned (allocates copy) |
| `into_` (e.g., `into_bytes`)       | Owned -> Owned (consumes self)     |
+------------------------------------+------------------------------------+
```

### 2.1 Getters: Omit the `get_` Prefix!
In Rust, field getters **omit the `get_` prefix**:
```rust
struct Client {
    timeout: std::time::Duration,
}

impl Client {
    // IDIOMATIC:
    pub fn timeout(&self) -> std::time::Duration {
        self.timeout
    }

    // UNIDIOMATIC:
    // pub fn get_timeout(&self) -> std::time::Duration { self.timeout }
}
```
*(The only exception is `get` on map-like collections returning `Option<&V>`, e.g., `map.get(&key)`).*

### 2.2 Domain Types vs. Primitives (`C-CUSTOM-TYPE`)
Avoid "stringly-typed" APIs. Wrap primitive types into distinct domain types:
```rust
// BAD: Caller can easily swap host and port
fn connect(host: String, port: String, timeout_seconds: u32) { ... }

// IDIOMATIC:
struct Hostname(String);
struct Port(u16);
struct Timeout(std::time::Duration);

fn connect(host: Hostname, port: Port, timeout: Timeout) { ... }
```

---

## Summary & Next Step

You have learned how to write comprehensive doc comments with doctests and apply the Rust API naming guidelines. Next, we use the compiler to prevent logic errors in **Leveraging the Type System: Newtypes, RAII, and Extension Traits** in [02-leveraging-the-type-system.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/02-leveraging-the-type-system.md).
