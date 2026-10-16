# Chapter 17: Lifetimes: Annotations, Structs, and Elision Rules

Lifetimes ensure that references never point to invalidated or deallocated memory. Every reference in Rust has a lifetime, which describes the span of code for which the reference is guaranteed to remain valid.

---

## 1. Why Lifetime Annotations Exist

Consider this function that returns a reference to the longest of two string slices:

```rust
// COMPILE ERROR: missing lifetime specifier
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
```

### Why Does the Compiler Reject This?
The compiler does not execute code at compile time; it cannot know whether the function will return `x` or `y`. 
- If `x` lives for 10 lines and `y` lives for 3 lines, how long is the returned reference valid?
- The compiler demands that we annotate the relationship between input and output references using **generic lifetime parameters**:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### What Does `'a` Mean?
- `'a` represents the **overlap (intersection) of the lifetimes of `x` and `y`**.
- The return value is guaranteed to be valid for as long as **both** `x` and `y` remain in scope.

---

## 2. The Three Lifetime Elision Rules

In most everyday Rust code, you do not write explicit lifetimes because the compiler applies **three deterministic elision rules**:

```
                              Lifetime Elision Rules
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
    Rule 1                           Rule 2                           Rule 3
Each elided input ref            If exactly ONE input             If `&self` or `&mut self`
gets its own unique              lifetime exists, it is           is present, its lifetime
lifetime parameter               assigned to all outputs          is assigned to all outputs
fn f(x: &i32, y: &str)           fn f(s: &str) -> &str            impl MyStruct {
-> fn f<'a, 'b>(...)             -> fn f<'a>(...) -> &'a str       fn f(&self, x: &str) -> &str
                                                                  -> gets lifetime of `self`
```

If after applying these three rules the output lifetime remains ambiguous, compilation halts with a descriptive error requiring explicit annotation.

---

## 3. Structs Holding References

If a struct stores a reference rather than an owned type, it must declare a lifetime parameter:

```rust
struct Highlight<'a> {
    text: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");

    let h = Highlight { text: first_sentence };
    println!("Highlighted: {}", h.text);
}
```

### Invariant:
The struct `Highlight<'a>` **cannot outlive** the memory referenced by `text`. If `novel` is dropped, `h` immediately becomes invalid.

---

## 4. The `'static` Lifetime

The `'static` lifetime is the longest possible lifetime: it lasts for the **entire duration of the program execution**.

### 1. String Literals
String literals are baked into the read-only data segment of the compiled binary:
```rust
let s: &'static str = "I live in the binary's data section forever!";
```

### 2. Trait Bounds (`T: 'static`)
When a generic parameter has a `'static` bound (`T: 'static`), it means `T` does not contain any non-static references. Owned types like `String`, `Vec<i32>`, and `i32` satisfy `T: 'static`!

---

## 5. Exercise: Zero-Copy Protocol Decoding (Protobuf Parsing)

### Problem Description
Protocol Buffers encode structured messages as key-value pairs where strings and byte arrays are encoded with a length prefix.
In high-performance systems, we want **zero-copy deserialization**: instead of allocating new `String` objects, the parsed struct holds borrowed string slices (`&'a str`) pointing directly into the input byte buffer!

Write a zero-copy parser for simple tag-length-value records:
```rust
struct Field<'a> {
    tag: u64,
    value: &'a str,
}

fn parse_field<'a>(buffer: &'a str) -> Option<(Field<'a>, &'a str)>
```

### Solution & Deep-Dive

```rust
#[derive(Debug, PartialEq)]
struct Field<'a> {
    tag: u64,
    value: &'a str,
}

/// Parses a field in the format: "<tag>:<len>:<data><remaining>"
/// Example: "1:5:helloREST_OF_BUFFER" -> tag 1, value "hello", remainder "REST_OF_BUFFER"
fn parse_field<'a>(buffer: &'a str) -> Option<(Field<'a>, &'a str)> {
    // 1. Extract tag
    let (tag_str, rest) = buffer.split_once(':')?;
    let tag: u64 = tag_str.parse().ok()?;

    // 2. Extract length
    let (len_str, rest) = rest.split_once(':')?;
    let len: usize = len_str.parse().ok()?;

    // 3. Extract payload slice using length
    if rest.len() < len {
        return None;
    }

    let value = &rest[..len];
    let remainder = &rest[len..];

    Some((Field { tag, value }, remainder))
}

fn main() {
    let wire_data = "101:4:PING102:4:PONG";

    let (f1, rest1) = parse_field(wire_data).expect("Failed to parse field 1");
    assert_eq!(f1, Field { tag: 101, value: "PING" });

    let (f2, rest2) = parse_field(rest1).expect("Failed to parse field 2");
    assert_eq!(f2, Field { tag: 102, value: "PONG" });
    assert_eq!(rest2, "");

    println!("Zero-copy protocol decoder verified successfully!");
}
```

### Performance Implication:
Notice that no `String::clone()` or heap allocations occurred. `Field<'a>` borrows directly from `wire_data`, achieving maximum throughput and zero GC pressure.

---

## Summary & Next Step

You have learned generic lifetime annotations, the three elision rules, struct lifetime bounds, and zero-copy decoding. Next, we look at advanced lifetimes: **Slices, Subtyping, and Lifetime Bounds** in [06-slices-and-lifetimes.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/03-day3-memory-and-lifetimes/06-slices-and-lifetimes.md).
