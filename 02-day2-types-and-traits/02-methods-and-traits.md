# Chapter 8: Methods, Traits, and Deriving

Rust does not have classical classes or inheritance. Instead, it decouples data definition (`struct`, `enum`) from behavior (`impl` blocks and `trait` definitions). This separation yields clean, modular, and reusable systems architecture.

---

## 1. Methods & Associated Functions

Methods are functions associated with a particular type. They are defined inside `impl` blocks:

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 1. Associated Function (Constructor): No `self` receiver
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // 2. Immutable Borrow Receiver (&self): Read-only method
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // 3. Mutable Borrow Receiver (&mut self): Modifies instance
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // 4. Value Receiver (self): Consumes and destroys the instance
    fn destroy(self) {
        println!("Destroying rectangle {}x{}", self.width, self.height);
    }
}
```

### 1.1 Receiver Types Breakdown
| Receiver | Ownership & Semantics | When to Use |
| :--- | :--- | :--- |
| `&self` | Borrows the instance immutably. | Inspecting fields, calculations, queries. |
| `&mut self` | Borrows the instance mutably. | In-place modifications, mutating internal state. |
| `self` | Takes ownership by value. Consumes instance. | Builder finalizers (`.build()`), type state transitions, resource cleanup. |
| None (`Self::fn`) | Associated function (static method). | Constructors (`new()`), factory methods. |

---

## 2. Traits: Defining Shared Behavior

A **trait** defines a set of methods that a type must provide. It is Rust's equivalent of interfaces or abstract base classes:

```rust
trait Describable {
    fn describe(&self) -> String;

    // Default method implementation:
    fn print_description(&self) {
        println!("{}", self.describe());
    }
}

struct Point { x: i32, y: i32 }

impl Describable for Point {
    fn describe(&self) -> String {
        format!("Point at ({}, {})", self.x, self.y)
    }
}
```

### 2.1 The Orphan Rule (Trait Coherence)
Rust enforces strict **coherence**: for any given pair of trait and type, there can be at most one implementation across the entire program.
- **Rule:** You can implement a trait for a type if and only if **either the trait OR the type is defined in your current crate**.
- You cannot implement an external trait (e.g., `Display` from `std`) for an external type (e.g., `Vec<T>` from `std`). This prevents two different dependencies from conflicting.

### 2.2 Supertraits
A trait can declare that it requires another trait to be implemented first:

```rust
// Person requires Display to be implemented
trait Person: std::fmt::Display {
    fn age(&self) -> u32;
}
```

### 2.3 Associated Types
Associated types allow a trait to define placeholder types within its signature:

```rust
trait Iterator {
    type Item; // Associated type
    fn next(&mut self) -> Option<Self::Item>;
}
```
*Why use associated types over generic parameters?* An iterator over a collection only ever yields one type of item. Using an associated type means you write `impl Iterator for MyList` rather than having to annotate the type parameter everywhere.

---

## 3. Deriving Traits

For common traits in the standard library, the compiler can automatically generate the implementation using `#[derive(...)]`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}
```

### Core Derivable Traits
- `Debug`: Enables formatting with `{:?}`.
- `Clone`: Explicit duplicate generation via `.clone()`.
- `Copy`: Enables implicit stack copy semantics (memcpy) instead of move semantics.
- `PartialEq`, `Eq`: Equality comparisons (`==`, `!=`).
- `PartialOrd`, `Ord`: Ordering comparisons (`<`, `>`, `<=`, `>=`).
- `Default`: Provides `Coordinate::default()`.
- `Hash`: Allows use as a key in `HashSet` or `HashMap`.

---

## 4. Exercise: Modular Logger Trait & Decorator

### Problem Description
Design an extensible logging subsystem using traits:
1. Define a `Logger` trait:
   ```rust
   trait Logger {
       fn log(&self, verbosity: u8, message: &str);
   }
   ```
2. Implement `StderrLogger` which prints `[LOG] message` to `eprintln!` if verbosity meets criteria.
3. Implement a decorator struct `VerbosityFilter<L: Logger>` that wraps any inner logger and filters out log messages whose verbosity exceeds `max_verbosity`.

### Starter Code
```rust
trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

struct VerbosityFilter<L: Logger> {
    max_verbosity: u8,
    inner: L,
}

// TODO: Implement Logger for StderrLogger and VerbosityFilter
```

---

### Solution & Deep-Dive

```rust
trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("[v{verbosity}] {message}");
    }
}

struct VerbosityFilter<L: Logger> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let raw_logger = StderrLogger;
    
    // Wrap StderrLogger in a filter that only permits verbosity <= 2
    let filtered_logger = VerbosityFilter {
        max_verbosity: 2,
        inner: raw_logger,
    };

    filtered_logger.log(1, "Critical info: shown"); // Shown
    filtered_logger.log(2, "Standard notice: shown"); // Shown
    filtered_logger.log(5, "Trace debug: filtered out!"); // Suppressed
}
```

### Architectural Insights:
- **Composition over Inheritance:** The `VerbosityFilter<L>` struct wraps any type that implements `Logger` and itself implements `Logger`. This is the classic **Decorator Pattern**, achieved cleanly through generic trait composition with zero virtual dispatch overhead!

---

## Summary & Next Step

You have learned methods, receivers (`&self`, `&mut self`, `self`), traits, supertraits, the orphan rule, and derive macros. Next, we delve into type parameterization: **Generics, Trait Bounds, and Dynamic Dispatch** in [03-generics.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/02-day2-types-and-traits/03-generics.md).
