# Chapter 11: Standard Library Types: Option, Result, Vec, String, and HashMap

Rust's standard library (`std`) provides a cohesive set of foundational types designed for memory safety, high performance, and ergonomic error handling.

---

## 1. `Option<T>`: Eliminating Null

In Rust, values are never null. Optionality is modeled explicitly using the `Option<T>` enum:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

### 1.1 Working Ergonomically with `Option`
Avoid relying on `.unwrap()`, which panics if the value is `None`. Instead, use safe combinators:

```rust
fn main() {
    let username: Option<&str> = Some("karthik");

    // 1. unwrap_or: Provide fallback default value
    let display_name = username.unwrap_or("Anonymous");

    // 2. map: Transform inner value if Some
    let len = username.map(|name| name.len()); // Some(7)

    // 3. and_then (flat_map): Chain operations returning Option
    let parsed: Option<i32> = Some("42").and_then(|s| s.parse().ok());
    println!("Parsed: {parsed:?}");
}
```

---

## 2. `Result<T, E>`: Recoverable Errors

The `Result<T, E>` enum represents operations that can succeed with value `T` or fail with error `E`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 2.1 The `?` (Try) Operator
The `?` operator unwraps `Ok(val)` or immediately returns `Err(e)` from the current function:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

---

## 3. Dynamic Arrays: `Vec<T>`

A `Vec<T>` is a growable, heap-allocated array:

```
Stack:
+---------------------------+
|  vec: Vec<i32>            |
|  [ptr]  --> Heap Buffer   |
|  [len]  --> 3             |
|  [cap]  --> 4             |
+---------------------------+
             |
             v
Heap Allocation:
[ 10 | 20 | 30 | (unallocated space) ]
```

### Common Operations
```rust
fn main() {
    // Macro initialization
    let mut numbers = vec![1, 2, 3];

    // Push and pop (amortized O(1))
    numbers.push(4);
    numbers.push(5);
    let last = numbers.pop(); // Some(5)

    // Indexing (panics if out of bounds)
    let second = numbers[1]; // 2

    // Safe indexing with .get() (returns Option<&T>)
    if let Some(val) = numbers.get(10) {
        println!("At 10: {val}");
    } else {
        println!("Index 10 out of bounds!");
    }
}
```

---

## 4. Key-Value Maps: `HashMap<K, V>`

`HashMap` stores key-value pairs using a cryptographically secure, collision-resistant hashing algorithm (SipHash 1-3):

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 25);

    // Querying with .get() -> Option<&V>
    if let Some(&score) = scores.get("Blue") {
        println!("Blue team: {score}");
    }
}
```

### The Powerful `Entry` API
The `.entry()` API allows inspecting and mutating a map key in a single lookup:

```rust
let mut scores = HashMap::new();

// Insert 50 only if "Yellow" is not already present:
scores.entry("Yellow").or_insert(50);

// Increment counter in-place:
let text = "hello world hello rust";
let mut counts = HashMap::new();

for word in text.split_whitespace() {
    let count = counts.entry(word).or_insert(0);
    *count += 1; // Mutate value through returned &mut usize
}
println!("Word frequencies: {counts:?}");
```

---

## 5. Iterators: Lazy, Composable Data Pipelines

In Rust, iterators are **lazy**; they produce no values until consumed:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Functional pipeline
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // Keep even numbers: 2, 4, 6
        .map(|&x| x * x)          // Square them: 4, 16, 36
        .sum();                   // Consume and sum

    println!("Sum: {sum_of_even_squares}"); // 56
}
```

### The Three Forms of Iteration
| Method | Yields | Transfers Ownership? | Collection Available After? |
| :--- | :--- | :--- | :--- |
| `.iter()` | `&T` (shared refs) | No | Yes (Read-only) |
| `.iter_mut()` | `&mut T` (exclusive refs) | No | Yes (Modified) |
| `.into_iter()` | `T` (owned values) | Yes | No (Moved/Consumed) |

---

## 6. Exercise: Frequency Counter with Entry API

### Problem Description
Implement a generic function:
```rust
fn count_frequencies<T, I>(items: I) -> HashMap<T, usize>
where
    T: Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
```
that counts the occurrences of items in any iterable collection using the `Entry` API.

### Solution & Walkthrough

```rust
use std::collections::HashMap;
use std::hash::Hash;

fn count_frequencies<T, I>(items: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: IntoIterator<Item = T>,
{
    let mut map = HashMap::new();
    for item in items {
        let counter = map.entry(item).or_insert(0);
        *counter += 1;
    }
    map
}

fn main() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let counts = count_frequencies(words);

    assert_eq!(counts.get("apple"), Some(&3));
    assert_eq!(counts.get("banana"), Some(&2));
    assert_eq!(counts.get("cherry"), Some(&1));
    assert_eq!(counts.get("durian"), None);
    println!("Counter verified successfully!");
}
```

---

## Summary & Next Step

You have mastered the standard library's core data structures: `Option`, `Result`, `Vec`, `HashMap`, and lazy iterators. Next, we examine fundamental system traits: **Standard Library Traits: Comparisons, Conversions, and I/O** in [06-standard-library-traits.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/02-day2-types-and-traits/06-standard-library-traits.md).
