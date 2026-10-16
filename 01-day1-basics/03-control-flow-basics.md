# Chapter 3: Control Flow Basics & Functions

Rust is an **expression-oriented language**. Unlike C, C++, or Java where control structures like `if` and blocks are statements that don't produce values, in Rust almost every construct evaluates to a value.

---

## 1. Blocks, Scopes, and Expressions vs. Statements

### 1.1 Lexical Scopes and Blocks
A block is a sequence of statements enclosed in curly braces `{ ... }`. Blocks introduce new lexical scopes:

```rust
fn main() {
    let x = 10;
    {
        let y = 20;
        println!("Inner scope: x = {x}, y = {y}");
    }
    // println!("y = {y}"); // COMPILE ERROR: `y` is out of scope!
}
```

### 1.2 Variable Shadowing
Rust allows you to declare a new variable with the same name as a previous one within the same or inner scope. This is known as **shadowing**:

```rust
fn main() {
    let spaces = "   ";           // spaces is a &str
    let spaces = spaces.len();     // spaces is shadowed as a usize!
    println!("Number of spaces: {spaces}");
}
```

#### Shadowing vs. Mutation:
- With `mut`, the variable's type cannot change (`spaces = spaces.len()` would fail).
- With shadowing, you can change both the **type** and **mutability** of the binding without inventing temporary names like `spaces_str` and `spaces_len`.

### 1.3 Expressions vs. Statements
- **Statement:** An instruction that performs an action and does not produce a value (or produces the unit type `()`). Statements end with a semicolon `;`.
- **Expression:** A construct that evaluates to a resultant value. Omit the semicolon on the final line of a block to return its value!

```rust
fn main() {
    let result = {
        let a = 10;
        let b = 25;
        a + b // NO semicolon: evaluates to 35 and assigns to `result`
    };
    println!("result: {result}"); // 35
}
```

> [!IMPORTANT]
> If you put a semicolon on `a + b;`, the block evaluates to the **unit type** `()` instead of `35`!

---

## 2. Conditionals: `if` Expressions

In Rust, `if` is an expression. You do not wrap the condition in parentheses:

```rust
fn main() {
    let score = 85;

    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else {
        'C'
    }; // Note the semicolon after the let statement

    println!("Grade: {grade}");
}
```

### Type Consistency
Because `if` is an expression, every branch must evaluate to the **exact same type**:

```rust
// COMPILE ERROR: `if` and `else` have incompatible types (i32 vs &str)
// let val = if true { 10 } else { "hello" };
```

---

## 3. Pattern Matching: `match` Expressions

The `match` expression provides powerful pattern matching that replaces `switch` statements from C/C++:

```rust
fn main() {
    let number = 3;

    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "something else", // `_` acts as the wildcard/default case
    };

    println!("{number} is {description}");
}
```

### Key Properties of `match`:
1. **Exhaustive:** The compiler verifies that every possible value of the matched type is covered. If you omit a variant or case, compilation fails.
2. **Evaluates to a value:** All match arms must produce values of the same type.

---

## 4. Loops in Rust

Rust provides three loop constructs: `loop`, `while`, and `for`.

### 4.1 Infinite Loops with `loop`
`loop` executes unconditionally until an explicit `break`. It can also **return a value from the loop**:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Break returns 20 to `result`
        }
    };

    println!("Result from loop: {result}"); // 20
}
```

### 4.2 Conditional Loops with `while`
```rust
fn main() {
    let mut n = 5;
    while n > 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("Liftoff!");
}
```

### 4.3 Collection Loops with `for`
The `for` loop iterates over any type that implements `IntoIterator`, including ranges:

```rust
fn main() {
    // Range exclusive: 0..5 (0, 1, 2, 3, 4)
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Range inclusive: 1..=5 (1, 2, 3, 4, 5)
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
}
```

### 4.4 Loop Labels
Nested loops can be disambiguated with labels starting with a single quote `'`:

```rust
fn main() {
    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            if x * y > 6 {
                println!("Breaking outer loop from inner loop at x={x}, y={y}");
                break 'outer;
            }
        }
    }
}
```

---

## 5. Functions

Functions are defined with `fn`:
- Parameter types must always be explicitly declared.
- Return types are declared with `-> Type`.
- The final expression without a semicolon is returned implicitly.

```rust
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    println!("gcd(143, 52) = {}", gcd(143, 52)); // 13
}
```

### Functions Without Return Value: The Unit Type `()`
If a function does not specify a return type, it implicitly returns `()`, the **unit type**:

```rust
fn print_greeting(name: &str) -> () {
    println!("Hello, {name}!");
}
// Equivalent to:
fn print_greeting_concise(name: &str) {
    println!("Hello, {name}!");
}
```

---

## 6. Standard Library Macros

Macros in Rust end with an exclamation mark (`!`) and perform compile-time code generation:

| Macro | Description |
| :--- | :--- |
| `println!(...)` | Prints formatted text with a newline to standard output (`stdout`). |
| `eprintln!(...)` | Prints formatted text with a newline to standard error (`stderr`). |
| `format!(...)` | Creates an owned heap-allocated `String` with formatted text. |
| `dbg!(expr)` | Prints file, line number, expression, and its value, returning the value. |
| `todo!(...)` | Marks unfinished code; panics at runtime if invoked. |
| `unreachable!(...)` | Informs compiler and runtime that a branch is impossible. |
| `assert!(cond)` | Panics if the boolean condition is `false`. |
| `assert_eq!(a, b)` | Panics if `a != b`, printing both values. |

---

## 7. Exercise: Collatz Sequence

### Problem Description
The **Collatz sequence** (also known as the $3n + 1$ problem) is defined for any positive integer $n_1 > 0$:
- If $n_i$ is 1, the sequence terminates.
- If $n_i$ is even: $n_{i+1} = \frac{n_i}{2}$
- If $n_i$ is odd: $n_{i+1} = 3 \cdot n_i + 1$

For example, starting with $n_1 = 3$:
$$3 \to 10 \to 5 \to 16 \to 8 \to 4 \to 2 \to 1 \quad (\text{length} = 8)$$

Write a function `collatz_length(mut n: i32) -> u32` to determine the number of steps required to reach 1.

### Starter Code
```rust
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    todo!("Implement this function")
}

fn main() {
    println!("Length of 11: {}", collatz_length(11)); // Should print 15
}
```

---

### Solution & Deep-Dive

```rust
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        len += 1;
    }
    len
}

fn main() {
    assert_eq!(collatz_length(11), 15);
    assert_eq!(collatz_length(3), 8);
    assert_eq!(collatz_length(1), 1);
    println!("All Collatz tests passed!");
}
```

### Key Observations:
1. `mut n: i32`: Declaring the parameter as `mut n` makes the local variable `n` mutable within the function. Because `i32` implements `Copy`, this does not mutate the caller's variable!
2. `if` Expression: Instead of writing `if n % 2 == 0 { n = n / 2; } else { n = 3 * n + 1; }`, assigning the result of `if` directly to `n` is idiomatic Rust.
3. Edge Cases: If $n \le 0$, this loop would be infinite or invalid. In robust production Rust, we would either accept `u32` or return `Option<u32>` / `Result<u32, Error>`.

---

## Summary & Next Step

You now understand Rust's expression-based control flow, loops, shadowing, functions, and common macros. Next, we explore compound types: **Tuples, Arrays, and Destructuring** in [04-tuples-and-arrays.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/01-day1-basics/04-tuples-and-arrays.md).
