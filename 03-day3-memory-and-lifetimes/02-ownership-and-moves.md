# Chapter 14: Ownership, Move Semantics, and Drop

In Rust, memory safety without a garbage collector is achieved through **move semantics** and deterministic destruction via the `Drop` trait.

---

## 1. Move Semantics: Shallow Copy + Invalidation

Consider what happens when you assign one string variable to another:

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1; // MOVE occurs here!

    // println!("{s1}"); // COMPILE ERROR: borrow of moved value: `s1`
    println!("{s2}");
}
```

### Memory Diagram: Move Semantics
```
Before `let s2 = s1`:
Stack:                           Heap:
+-----------------------+        +---+---+---+---+
| s1                    |        | R | u | s | t |
| [ptr] ----------------+------> +---+---+---+---+
| [len: 4, cap: 4]      |
+-----------------------+

After `let s2 = s1`:
Stack:                           Heap:
+-----------------------+        +---+---+---+---+
| s1 (INVALIDATED!)     |        | R | u | s | t |
+-----------------------+        +---+---+---+---+
                                         ^
+-----------------------+                |
| s2                    |                |
| [ptr] ----------------+----------------+
| [len: 4, cap: 4]      |
+-----------------------+
```

### Why Does Rust Move Instead of Deep-Copying?
1. **Performance:** A move is a tiny 24-byte stack copy (pointer, length, capacity). It never allocates heap memory or copies payload bytes.
2. **Double-Free Prevention:** If both `s1` and `s2` were allowed to remain valid owners pointing to the same heap buffer, both would attempt to free that heap buffer when leaving scope, corrupting memory. By invalidating `s1`, Rust guarantees **exactly one free**.

---

## 2. `Copy` vs. `Clone`

Not all types are moved. Types that live entirely on the stack and have no external resources implement the `Copy` trait:

```
+------------------------------------+------------------------------------+
|             `Copy` Trait           |            `Clone` Trait           |
+------------------------------------+------------------------------------+
| - Implicit bitwise memcpy          | - Explicit duplication via         |
| - Original remains completely      |   `.clone()` method                |
|   valid after assignment           | - Can perform arbitrary deep       |
| - Implemented by primitives:       |   allocations                      |
|   `i32`, `f64`, `bool`, `char`,    | - Implemented by `String`, `Vec`,  |
|   arrays of Copy types             |   and user structs                 |
| - CANNOT be implemented by types   | - Required whenever heap data      |
|   that implement `Drop`            |   must be duplicated               |
+------------------------------------+------------------------------------+
```

### Example
```rust
fn main() {
    let x: i32 = 42;
    let y = x; // Copied implicitly!
    println!("x = {x}, y = {y}"); // Both x and y are valid!

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit deep heap allocation!
    println!("s1 = {s1}, s2 = {s2}"); // Both valid
}
```

---

## 3. The `Drop` Trait: Deterministic Destruction

The `Drop` trait provides custom destructor logic executed automatically when a value leaves its scope:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

### 3.1 Drop Order Guarantees
Rust provides strict, deterministic destruction order:
1. **Local Variables:** Dropped in **reverse order of declaration** (LIFO: Last In, First Out).
2. **Struct Fields:** Dropped in **order of declaration** (FIFO: First In, First Out).

### 3.2 Early Drop via `std::mem::drop`
You cannot invoke `.drop()` directly because that would cause a double-drop bug when the variable leaves scope. Instead, use `std::mem::drop`:

```rust
fn main() {
    let resource = CustomSmartPointer { data: String::from("file_lock") };
    
    // Explicit early release:
    std::mem::drop(resource); // Moves `resource` into function, dropping it immediately
    
    println!("Resource was already freed before this line.");
}
```

---

## 4. Exercise: Type-Safe Builder Pattern

### Problem Description
The Builder pattern allows configuring complex objects incrementally. Using Rust's move semantics (`self`), we can build a fluent API that consumes the builder at each step and finally converts into the target struct.

Implement a `PackageBuilder` for constructing a `Package`:
- Fields: `name: String`, `version: String`, `authors: Vec<String>`.
- Fluent methods: `.version(str)`, `.author(str)`.
- Terminal method: `.build() -> Package`.

### Solution & Walkthrough

```rust
#[derive(Debug, PartialEq)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

struct PackageBuilder {
    name: String,
    version: Option<String>,
    authors: Vec<String>,
}

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        PackageBuilder {
            name: name.into(),
            version: None,
            authors: Vec::new(),
        }
    }

    // Takes `mut self` by value and returns `self` for chaining:
    fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    fn author(mut self, author: impl Into<String>) -> Self {
        self.authors.push(author.into());
        self
    }

    // Consumes the builder by value (`self`), transferring ownership into `Package`
    fn build(self) -> Package {
        Package {
            name: self.name,
            version: self.version.unwrap_or_else(|| String::from("0.1.0")),
            authors: self.authors,
        }
    }
}

fn main() {
    let pkg = PackageBuilder::new("serde")
        .version("1.0.193")
        .author("Erick Tryzelaar")
        .author("David Tolnay")
        .build();

    assert_eq!(pkg.name, "serde");
    assert_eq!(pkg.version, "1.0.193");
    assert_eq!(pkg.authors.len(), 2);
    println!("Builder verified: {pkg:?}");
}
```

### Why Passing `self` by Value is Superior:
By having `.build(self)` consume `self`, the builder is invalidated. The caller cannot accidentally reuse an incomplete or modified builder!

---

## Summary & Next Step

You have mastered move semantics, `Copy` vs `Clone`, drop order, and ownership-based builders. Next, we explore heap allocations and shared ownership in **Smart Pointers: Box, Rc, and RefCell** in [03-smart-pointers.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/03-smart-pointers.md).
