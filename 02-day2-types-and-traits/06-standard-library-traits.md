# Chapter 12: Standard Library Traits: Comparisons, Conversions, and I/O

The Rust standard library provides foundational traits that integrate user types into language syntax—enabling operator overloading (`+`, `*`), conversions (`.into()`), and standard I/O streaming.

---

## 1. Comparison Traits: `PartialEq`, `Eq`, `PartialOrd`, and `Ord`

Rust separates mathematical equality and ordering into partial and total variants:

```
+------------------------------------+------------------------------------+
|            Partial Traits          |             Total Traits           |
+------------------------------------+------------------------------------+
| `PartialEq`:                       | `Eq`:                              |
| - Symmetric: a == b => b == a      | - Reflexive: a == a for ALL values |
| - Transitive: a == b & b == c =>   | - Required for HashMap keys        |
|               a == c               | - Implemented by integers, str     |
| - Implemented by f32, f64          | - NOT implemented by f32/f64       |
|                                    |   (because NaN != NaN)             |
+------------------------------------+------------------------------------+
| `PartialOrd`:                      | `Ord`:                             |
| - Can return `None` from           | - Strict total ordering: every pair|
|   `partial_cmp(&a, &b)`            |   is <, ==, or >                   |
| - Allows `<` and `>` operators     | - Required for `BTreeMap`, `.sort()`|
+------------------------------------+------------------------------------+
```

### Deriving Comparisons
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

fn main() {
    let v1 = Version { major: 1, minor: 2, patch: 0 };
    let v2 = Version { major: 1, minor: 3, patch: 0 };
    assert!(v1 < v2);
}
```

---

## 2. Operator Overloading (`std::ops`)

Rust allows overloading operators by implementing corresponding traits in `std::ops`:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Invokes Add::add(p1, p2)
    assert_eq!(p3, Point { x: 4, y: 6 });
}
```

---

## 3. Conversions: `From`, `Into`, and `TryFrom`

### 3.1 Infallible Conversions: `From` & `Into`
`From` and `Into` represent conversions that **cannot fail**:

```rust
struct Seconds(u32);

impl From<u32> for Seconds {
    fn from(s: u32) -> Self {
        Seconds(s)
    }
}

fn main() {
    // 1. Direct From usage
    let s1 = Seconds::from(60);

    // 2. Automatic Into implementation!
    // Implementing From<T> for U automatically gives you Into<U> for T!
    let s2: Seconds = 120.into();
}
```

### 3.2 Fallible Conversions: `TryFrom` & `TryInto`
When a conversion can fail (e.g., converting a large integer to a smaller one):

```rust
use std::convert::TryFrom;

fn main() {
    let big_num: i64 = 1000;
    let small_num = u8::try_from(big_num);
    assert!(small_num.is_err()); // 1000 exceeds u8::MAX (255)

    let valid_num: i64 = 42;
    let small_valid = u8::try_from(valid_num);
    assert_eq!(small_valid, Ok(42));
}
```

> [!WARNING]
> Prefer `TryFrom` over raw `as` casting. `1000 as u8` silently truncates to `232` without any warning, causing security and correctness bugs.

---

## 4. The Standard I/O Traits: `Read` and `Write`

Rust's I/O model is based on two stream traits in `std::io`:

### 4.1 `std::io::Read`
```rust
pub trait Read {
    // Reads bytes into `buf`, returning the number of bytes read
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}
```

### 4.2 `std::io::Write`
```rust
pub trait Write {
    // Writes bytes from `buf`, returning the number of bytes written
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;
}
```

---

## 5. Exercise: ROT13 Streaming Decoder

### Problem Description
ROT13 ("rotate by 13 places") is a classic substitution cipher that rotates letters by 13 positions in the alphabet:
- `'a'..='m'` and `'A'..='M'` move forward by 13.
- `'n'..='z'` and `'N'..='Z'` move backward by 13.
- Non-alphabetic characters remain untouched.

Implement a streaming wrapper struct `Rot13<R: Read>` that implements `std::io::Read`, decrypting data on-the-fly as it is read from the underlying source.

### Starter Code
```rust
use std::io::Read;

struct Rot13<R: Read> {
    input: R,
}

impl<R: Read> Read for Rot13<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!("Implement ROT13 streaming reader")
    }
}
```

---

### Solution & Deep-Dive

```rust
use std::io::{self, Read};

struct Rot13<R: Read> {
    input: R,
}

impl<R: Read> Rot13<R> {
    fn new(input: R) -> Self {
        Rot13 { input }
    }
}

fn rot13_byte(byte: u8) -> u8 {
    match byte {
        b'a'..=b'm' | b'A'..=b'M' => byte + 13,
        b'n'..=b'z' | b'N'..=b'Z' => byte - 13,
        other => other,
    }
}

impl<R: Read> Read for Rot13<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read into buffer from underlying reader
        let bytes_read = self.input.read(buf)?;

        // Transform the bytes read in-place
        for byte in &mut buf[..bytes_read] {
            *byte = rot13_byte(*byte);
        }

        Ok(bytes_read)
    }
}

fn main() -> io::Result<()> {
    let cipher_text = "Uryyb, Ehyvat Cyna!"; // "Hello, Ruling Plan!"
    let mut decoder = Rot13::new(cipher_text.as_bytes());
    
    let mut decoded = String::new();
    decoder.read_to_string(&mut decoded)?;

    assert_eq!(decoded, "Hello, Ruling Plan!");
    println!("ROT13 Decoder verified successfully: '{decoded}'");
    Ok(())
}
```

### Architectural Insights:
1. **Composability:** Because `Rot13<R>` takes any `R: Read`, you can wrap file streams (`File`), network sockets (`TcpStream`), or in-memory byte buffers (`&[u8]`).
2. **Zero In-Memory Allocation:** Decryption occurs directly inside the caller-provided buffer `&mut buf[..bytes_read]`. No intermediate heap strings or allocations are created.

---

## Summary & Next Step

You have completed **Day 2: Advanced Types, Traits, and the Standard Library**! 

Proceed to **Day 3: Memory Management, Ownership, and Lifetimes** starting with [03-day3-memory-and-lifetimes/01-memory-management.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/01-memory-management.md).
