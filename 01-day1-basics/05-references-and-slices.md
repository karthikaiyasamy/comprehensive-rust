# Chapter 5: References, Slices, and Strings

References are the bedrock of Rust's safety model. They allow code to access and borrow data without transferring ownership or copying memory, with complete guarantees against null pointers, data races, and dangling pointers.

---

## 1. References (`&T` and `&mut T`)

A reference is a pointer with compile-time checked validity rules:

```rust
fn main() {
    let a = 'A';
    let b = 'B';
    
    // Shared reference to `char`
    let mut r: &char = &a;
    println!("r points to: {}", *r); // Dereference with `*`
    
    r = &b; // Reassign reference to point to `b`
    println!("r now points to: {}", *r);
}
```

### 1.1 Shared vs. Exclusive References

Rust divides references into two strictly distinct categories:

```
+------------------------------------+------------------------------------+
|       Shared References: `&T`      |     Exclusive References: `&mut T` |
+------------------------------------+------------------------------------+
| - Read-only access                 | - Read-write access                |
| - Can have UNLIMITED shared refs   | - Can have ONLY ONE exclusive ref  |
| - Data is immutable while borrowed | - No other refs (shared or mut)    |
|   by shared references             |   can exist simultaneously         |
| - Known as "aliased" reads         | - Known as "exclusive" write       |
+------------------------------------+------------------------------------+
```

#### The Fundamental Aliasing XOR Mutability Rule:
> **You can have ANY number of shared references (`&T`) OR EXACTLY ONE exclusive reference (`&mut T`), but NEVER BOTH at the same time.**

### 1.2 Exclusive References in Action

```rust
fn main() {
    let mut point = (10, 20);

    let x_ref: &mut i32 = &mut point.0;
    *x_ref = 99; // Mutate value via dereference

    println!("Point: {point:?}"); // Prints: (99, 20)
}
```

#### Distinguishing Binding Mutability vs. Target Mutability:
```rust
let mut p1: &i32 = &10;     // Mutable reference variable pointing to immutable i32 (can point to another i32)
let p2: &mut i32 = &mut 10; // Immutable reference variable pointing to mutable i32 (cannot rebind p2, but can mutate *p2)
let mut p3: &mut i32 = &mut 10; // Mutable reference variable pointing to mutable i32
```

---

## 2. Slices (`&[T]`)

A slice is a dynamically sized **view** into a contiguous sequence of elements (such as an array or a `Vec`):

```rust
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let slice1: &[i32] = &a[2..4]; // Elements at index 2 and 3: [30, 40]
    let slice_all: &[i32] = &a[..]; // Whole array as a slice

    println!("slice1: {slice1:?}");
    println!("Length: {}", slice1.len());
}
```

### Slice Memory Representation: The Fat Pointer
A regular reference `&i32` is a single machine pointer (8 bytes on 64-bit architectures).
A slice `&[T]` is a **Fat Pointer** containing two words (16 bytes on 64-bit):

```
Stack:
+-----------------------+
|  slice1 (&[i32])      |
|  [ptr]  --> &a[2]     |  (8 bytes: address of element index 2)
|  [len]  --> 2         |  (8 bytes: number of elements)
+-----------------------+
        |
        v
Array `a` in memory:
[ 10 | 20 | 30 | 40 | 50 | 60 ]
            ^         ^
            +---------+  (Slice range: length 2)
```

### Range Slicing Syntax
- `&a[0..3]`: Elements 0, 1, 2 (exclusive end).
- `&a[..3]`: Same as `0..3` (omitted start defaults to 0).
- `&a[2..]`: From index 2 to end of array.
- `&a[..]`: Entire array as slice.

> [!TIP]
> Functions should almost always take slices (`&[T]`) rather than array references (`&[T; N]`). A function taking `&[T]` can accept arrays of any length, slices, and vectors!

---

## 3. Rust Strings: `&str` vs. `String`

Rust cleanly distinguishes between borrowed string slices and owned string buffers:

```
+------------------------------------+------------------------------------+
|          String Slice: `&str`      |         Owned Buffer: `String`     |
+------------------------------------+------------------------------------+
| - Borrowed view of UTF-8 bytes     | - Heap-allocated, growable buffer  |
| - Stored as Fat Pointer:           | - Stored as 3 words on stack:      |
|   (pointer, length)                |   (pointer, length, capacity)      |
| - Cannot be resized                | - Can `push`, `push_str`, truncate |
| - String literals `"..."` are      | - Created with `String::new()` or  |
|   `&'static str`                   |   `String::from(...)`              |
+------------------------------------+------------------------------------+
```

### Example: Strings in Practice
```rust
fn main() {
    // 1. String literal (stored in binary data section, type &'static str)
    let greeting: &str = "Hello";

    // 2. Heap-allocated owned String
    let mut s = String::from(greeting);
    s.push_str(" World!");
    println!("{s}");

    // 3. Borrowing a slice from a String
    let slice: &str = &s[0..5];
    println!("Slice: {slice}"); // "Hello"
}
```

### String Slicing Warning: UTF-8 Boundaries
Rust strings are strictly encoded in **UTF-8**. Characters can be 1, 2, 3, or 4 bytes long.
- If you slice a string at a byte index that does not land on a valid UTF-8 character boundary, your program will **panic**!
- To safely inspect characters, iterate with `.chars()`:
  ```rust
  for ch in "🦀 Rust".chars() {
      println!("{ch}");
  }
  ```

---

## 4. Reference Validity: No Null, No Dangling Pointers

Rust guarantees that a reference is **always valid**:
1. **No Null References:** In safe Rust, references can never be null. Nullable values are represented explicitly with `Option<&T>`.
2. **No Dangling Pointers:** The compiler guarantees that references never outlive the data they borrow:

```rust
fn dangling_pointer() {
    /*
    let r;
    {
        let x = 42;
        r = &x; // COMPILE ERROR: `x` does not live long enough!
    }
    println!("{r}");
    */
}
```

---

## 5. Exercise: 3D Geometry Utilities

### Problem Description
Implement two geometric functions operating on 3D vectors represented as `[f64; 3]`:
1. `magnitude(vector: &[f64; 3]) -> f64`: Calculates the Euclidean norm $\sqrt{x^2 + y^2 + z^2}$.
2. `normalize(vector: &mut [f64; 3])`: Normalizes the vector in-place so its magnitude becomes 1.0 without changing its direction.

### Starter Code
```rust
fn magnitude(vector: &[f64; 3]) -> f64 {
    todo!("Calculate vector magnitude")
}

fn normalize(vector: &mut [f64; 3]) {
    todo!("Normalize vector in-place")
}

fn main() {
    println!("Unit magnitude: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("After normalization: {v:?}");
    println!("New magnitude: {}", magnitude(&v));
}
```

---

### Solution & Deep-Dive

```rust
/// Calculate the magnitude of the 3D vector.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut sum_of_squares = 0.0;
    for &coord in vector {
        sum_of_squares += coord * coord;
    }
    sum_of_squares.sqrt()
}

/// Change the magnitude of the vector to 1.0 in-place.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag; // Dereference mutable reference to update in-place
    }
}

fn main() {
    let unit = [0.0, 1.0, 0.0];
    assert!((magnitude(&unit) - 1.0).abs() < 1e-9);

    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    assert!((magnitude(&v) - 1.0).abs() < 1e-9);
    println!("Geometry tests passed successfully!");
}
```

### Key Observations:
- In `magnitude(&[f64; 3])`, we take a shared reference because we only read coordinates.
- In `normalize(&mut [f64; 3])`, we take an exclusive reference because coordinates are modified in-place.
- In `for item in vector`, iterating over `&mut [f64; 3]` produces `item: &mut f64`. We use `*item /= mag` to modify the value through the reference.

---

## Summary & Next Step

You have learned shared references (`&T`), exclusive references (`&mut T`), slice fat pointers, string semantics, and in-place vector mutation. Next, we build custom data abstractions in **User-Defined Types: Structs, Enums, Const, and Static** in [06-user-defined-types.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/06-user-defined-types.md).
