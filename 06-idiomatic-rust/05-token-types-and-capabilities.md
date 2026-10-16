# Chapter 34: Token Types & Capability-Based Security

In secure systems design, **Capability-Based Security** dictates that components must possess an unforgeable ticket ("token") to execute privileged operations. In Rust, you can implement capability-based security using **Zero-Sized Token Types**.

---

## 1. What is a Token Type?

A token type is a struct with private fields that represents proof that an invariant has been verified:

```rust
pub mod security {
    // Unforgeable Token: Cannot be constructed outside this module because of private field `()`!
    pub struct AdminToken(());

    pub fn authenticate_admin(password: &str) -> Option<AdminToken> {
        if password == "super_secret_rust_key" {
            Some(AdminToken(()))
        } else {
            None
        }
    }

    // Privileged operation: Demands proof of authentication via `&AdminToken`!
    pub fn wipe_database(_token: &AdminToken) {
        println!("Database wiped by authenticated administrator.");
    }
}

fn main() {
    // Caller cannot create AdminToken directly:
    // let fake_token = security::AdminToken(()); // COMPILE ERROR: field is private!

    if let Some(token) = security::authenticate_admin("super_secret_rust_key") {
        security::wipe_database(&token); // Authorized!
    }
}
```

---

## 2. Zero Runtime Overhead (ZSTs)

Because `AdminToken` contains only the unit type `()`, its size in memory is **0 bytes**:
- Passing `token` by value or reference generates **no machine code**.
- No heap allocations or runtime pointer chasing occurs.
- The authorization check is executed entirely at compile time!

---

## 3. Hardware Initialization Tokens

In bare-metal embedded systems, trying to access a peripheral (like a UART serial port or SPI bus) before initializing the system clock controller results in a hardware fault.
Tokens guarantee the correct sequence:

```rust
struct ClockConfigured;

fn configure_system_clocks() -> ClockConfigured {
    // Write registers to enable PLL and clocks
    ClockConfigured
}

fn init_uart(_proof: &ClockConfigured) {
    // Safe to initialize UART because clocks are guaranteed active
}
```

---

## Summary & Next Step

You have learned capability-based security using zero-sized token types. Next, we explore modern design patterns in **Polymorphism and Composition over Inheritance** in [06-polymorphism-composition.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/06-polymorphism-composition.md).
