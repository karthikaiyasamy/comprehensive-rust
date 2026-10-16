# Chapter 4: Tuples, Arrays, and Destructuring

In this chapter, we explore Rust's primitive compound types: **arrays** (fixed-size homogeneous collections) and **tuples** (fixed-size heterogeneous collections), along with pattern destructuring.

---

## 1. Arrays (`[T; N]`)

An array is a collection of multiple values of the **same type** `T` with a fixed length `N` known at compile time:

```rust
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    println!("Array: {a:?}"); // {:?} uses Debug formatting
    
    a[0] = 99;
    println!("Updated array: {a:?}");
}
```

### Memory Layout
Unlike languages like Java or Python where arrays are pointers to heap objects, in Rust, arrays are allocated **directly on the stack inline**:

```
Stack Frame for main():
+-------+-------+-------+-------+-------+
| a[0]  | a[1]  | a[2]  | a[3]  | a[4]  |
|  99   |   4   |   3   |   2   |   1   |
| 1 byte| 1 byte| 1 byte| 1 byte| 1 byte|
+-------+-------+-------+-------+-------+
Total size = 5 bytes (no pointer indirection, no heap overhead)
```

### 1.1 Array Properties
1. **Length is Part of the Type:** `[u8; 3]` and `[u8; 4]` are two completely distinct types. You cannot pass a 4-element array to a function expecting a 3-element array.
2. **Repeat Initialization Syntax:** You can initialize an array with repeated values using `[value; count]`:
   ```rust
   let buffer = [0u8; 1024]; // 1024 zeroes on the stack
   ```
3. **No Resizing:** Because the size is fixed at compile time, you cannot `push` or `pop` elements from an array (use `Vec<T>` for dynamic arrays).

### 1.2 Bounds Checking & Safety
Array indexing in Rust requires an index of type `usize`.
- **Compile-time check:** If you use a constant index out of bounds (e.g., `a[10]`), `rustc` rejects it at compile time!
- **Runtime check:** If the index is computed dynamically, Rust inserts an automatic bounds check. Out-of-bounds indexing safely **panics** without memory corruption or buffer overflows.
- In optimized release builds, LLVM eliminates bounds checks inside loops whenever it can prove the index is within bounds.

---

## 2. Tuples (`(T1, T2, ...)`)

A tuple groups multiple values of **different types** into a single compound value:

```rust
fn main() {
    let t: (i32, f64, &str) = (500, 6.4, "rust");
    
    // Access fields by dot index:
    println!("Integer: {}", t.0);
    println!("Float:   {}", t.1);
    println!("String:  {}", t.2);
}
```

### 2.1 The Unit Type `()`
The 0-element tuple `()` is called the **unit type**:
- It has only one possible value: `()`.
- Its size in memory is **0 bytes** (a Zero-Sized Type, or ZST).
- It is returned by functions that don't return an explicit value.

```rust
let unit: () = ();
println!("Size of unit type: {} bytes", std::mem::size_of::<()>()); // 0
```

---

## 3. Iterating Over Arrays

You can iterate directly over arrays using `for` loops:

```rust
fn main() {
    let primes = [2, 3, 5, 7, 11];

    for prime in primes {
        println!("Prime: {prime}");
    }

    // With index using .iter().enumerate():
    for (index, prime) in primes.iter().enumerate() {
        println!("primes[{index}] = {prime}");
    }
}
```

> [!NOTE]
> You cannot iterate over a tuple with a `for` loop because tuple elements can have different types, while a loop body requires a single homogeneous type for the iteration variable.

---

## 4. Pattern Destructuring

Rust allows unpacking compound types into separate variables using patterns:

```rust
fn main() {
    // 1. Destructuring tuples
    let triple = (10, 20, 30);
    let (x, y, z) = triple;
    println!("x={x}, y={y}, z={z}");

    // Ignoring components with `_`
    let (first, _, last) = triple;

    // 2. Destructuring arrays
    let numbers = [1, 2, 3, 4, 5];
    let [head, second, tail @ ..] = numbers;
    println!("head: {head}, second: {second}, remaining: {tail:?}");
}
```

---

## 5. Exercise: Nested Arrays (Matrix Transposition)

### Problem Description
Arrays can be nested to create multi-dimensional matrices:
```rust
let matrix: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];
```

Write a function `transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]` that transposes a $3 \times 3$ matrix (exchanging rows and columns):

$$\begin{pmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \\ 7 & 8 & 9 \end{pmatrix}^T = \begin{pmatrix} 1 & 4 & 7 \\ 2 & 5 & 8 \\ 3 & 6 & 9 \end{pmatrix}$$

### Starter Code
```rust
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!("Implement matrix transpose")
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    println!("Transposed: {transposed:?}");
}
```

---

### Solution & Deep-Dive

```rust
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    
    assert_eq!(transposed[0], [101, 201, 301]);
    assert_eq!(transposed[1], [102, 202, 302]);
    assert_eq!(transposed[2], [103, 203, 303]);
    println!("Matrix transpose verified successfully!");
}
```

### Deep Dive: Copy Semantics in Arrays
In Rust, compound types implement `Copy` if and only if **all of their components implement `Copy`**.
- Because `i32` implements `Copy`, the nested array `[[i32; 3]; 3]` also implements `Copy`.
- When passing `matrix` to `transpose(matrix)`, the array is copied by value onto the new stack frame. The original `matrix` in `main` remains intact and valid!

---

## Summary & Next Step

You now understand stack-allocated arrays, heterogeneous tuples, and pattern destructuring. Next, we explore one of Rust's most critical concepts: **References and Slices** in [05-references-and-slices.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/05-references-and-slices.md).
