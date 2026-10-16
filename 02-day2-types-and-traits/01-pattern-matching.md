# Chapter 7: Advanced Pattern Matching & Control Flow

Pattern matching is one of Rust's most expressive and powerful language features. It goes far beyond simple equality checking, allowing you to destructure complex algebraic data types, inspect internal payloads, and conditionally branch with compiler-verified exhaustiveness.

---

## 1. Refutable vs. Irrefutable Patterns

In Rust, patterns are categorized based on whether matching can fail:

```
+------------------------------------+------------------------------------+
|       Irrefutable Patterns         |        Refutable Patterns          |
+------------------------------------+------------------------------------+
| - Guaranteed to ALWAYS match       | - Can FAIL to match for some       |
| - Used where failure is illegal:   |   possible values                  |
|   `let`, function parameters,      | - Used in conditionals:            |
|   `for` loops                      |   `match`, `if let`, `while let`   |
| - Example: `let (x, y) = point;`   | - Example: `if let Some(x) = opt`  |
+------------------------------------+------------------------------------+
```

If you use a refutable pattern in a `let` statement, the compiler rejects it:
```rust
let opt: Option<i32> = Some(5);
// let Some(x) = opt; // COMPILE ERROR: refutable pattern in local binding
```

---

## 2. Pattern Matching Capabilities

Rust's `match` provides extensive pattern matching syntax:

### 2.1 Literals, Ranges, and Multiple Patterns
```rust
fn classify_char(c: char) -> &'static str {
    match c {
        'a'..='z' | 'A'..='Z' => "alphabetic letter",
        '0'..='9'             => "decimal digit",
        ' ' | '\t' | '\n'     => "whitespace",
        _                     => "other symbol",
    }
}
```

### 2.2 Destructuring Structs
You can match and bind specific fields, rename fields, or ignore unwanted fields:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn describe_point(p: Point) {
    match p {
        Point { x: 0, y: 0 } => println!("At the origin"),
        Point { x, y: 0 }    => println!("On the X axis at {x}"),
        Point { x: 0, y }    => println!("On the Y axis at {y}"),
        Point { x, .. }      => println!("Somewhere else with x = {x}"),
    }
}
```

### 2.3 Destructuring Enums & Pattern Binding (`@`)
The `@` operator lets you test a value against a pattern while simultaneously binding it to a variable:

```rust
enum Message {
    Hello { id: i32 },
}

fn inspect_message(msg: Message) {
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("Found id in range [3, 7]: {id_var}");
        }
        Message::Hello { id: 10 | 11 | 12 } => {
            println!("Found special fixed id");
        }
        Message::Hello { id } => {
            println!("Found general id: {id}");
        }
    }
}
```

### 2.4 Match Guards
A **match guard** is an additional `if` condition specified after the pattern:

```rust
fn check_pair(pair: (i32, i32)) {
    match pair {
        (x, y) if x == y => println!("Equal components: {x}"),
        (x, y) if x + y == 0 => println!("Opposite signs: {x} and {y}"),
        (x, y) => println!("Arbitrary coordinates: ({x}, {y})"),
    }
}
```

> [!NOTE]
> The compiler does not analyze the boolean logic inside match guards for exhaustiveness. Therefore, you must still provide a fallback arm when using match guards.

---

## 3. Expressive Let Control Flow

Rust provides ergonomic shortcuts for matching against single refutable patterns.

### 3.1 `if let` Expressions
Use `if let` when you only care about matching one specific variant and ignoring everything else:

```rust
let config_max: Option<u32> = Some(100);

if let Some(max) = config_max {
    println!("Maximum configured to {max}");
}
```

### 3.2 `while let` Loops
`while let` repeatedly executes a loop body as long as the pattern continues to match:

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Popped: {top}");
}
```

### 3.3 `let else` Statements (Guard Clauses)
Introduced in Rust 1.65, `let else` provides clean guard-clause semantics. It binds variables on pattern match or diverges (`return`, `break`, `continue`, or `panic!`):

```rust
fn process_integer(opt: Option<i32>) -> i32 {
    let Some(val) = opt else {
        println!("Value was None! Exiting early.");
        return 0; // The else block MUST diverge!
    };

    // `val` is now in scope in the rest of the function:
    val * 2
}
```

---

## 4. Exercise: Arithmetic Expression Evaluation

### Problem Description
We want to evaluate recursive arithmetic expressions using an algebraic data type:
- An operation `Op`: `Add`, `Sub`, `Mul`, `Div`.
- An expression `Expression`:
  - `Value(i64)`: A literal numeric constant.
  - `Op { op: Op, left: Box<Expression>, right: Box<Expression> }`: An operation applied to sub-expressions.
  *(Note: `Box` is required here because recursive types have dynamic sizes; `Box` provides pointer indirection).*

Write an evaluator function:
```rust
fn eval(e: Expression) -> Result<i64, String>
```
If division by zero occurs, return `Err(String::from("division by zero"))`.

### Starter Code
```rust
#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Value(i64),
    Op {
        op: Op,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

fn eval(e: Expression) -> Result<i64, String> {
    todo!("Evaluate recursive expression")
}

fn main() {
    let expr = Expression::Op {
        op: Op::Sub,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Op {
            op: Op::Mul,
            left: Box::new(Expression::Value(2)),
            right: Box::new(Expression::Value(3)),
        }),
    };
    println!("Result: {:?}", eval(expr)); // Should evaluate: 10 - (2 * 3) = 4
}
```

---

### Solution & Deep-Dive

```rust
#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Value(i64),
    Op {
        op: Op,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(val) => Ok(val),
        Expression::Op { op, left, right } => {
            let left_val = eval(*left)?;
            let right_val = eval(*right)?;
            match op {
                Op::Add => Ok(left_val + right_val),
                Op::Sub => Ok(left_val - right_val),
                Op::Mul => Ok(left_val * right_val),
                Op::Div => {
                    if right_val == 0 {
                        Err(String::from("division by zero"))
                    } else {
                        Ok(left_val / right_val)
                    }
                }
            }
        }
    }
}

fn main() {
    // 10 - (2 * 3) = 4
    let expr = Expression::Op {
        op: Op::Sub,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Op {
            op: Op::Mul,
            left: Box::new(Expression::Value(2)),
            right: Box::new(Expression::Value(3)),
        }),
    };
    assert_eq!(eval(expr), Ok(4));

    // Division by zero test: 5 / 0
    let div_zero = Expression::Op {
        op: Op::Div,
        left: Box::new(Expression::Value(5)),
        right: Box::new(Expression::Value(0)),
    };
    assert_eq!(eval(div_zero), Err(String::from("division by zero")));
    println!("Expression evaluation verified successfully!");
}
```

### Key Technical Concepts in the Solution:
1. **Recursive Data Types & `Box`:** A struct or enum cannot contain an unboxed instance of itself because its size would be infinite. `Box<Expression>` places the child node on the heap, giving the node a fixed pointer size of 8 bytes on the stack.
2. **The `?` Operator Preview:** `eval(*left)?` unboxes and evaluates the sub-expression. If evaluation returns `Ok(v)`, `v` is extracted. If it returns `Err(e)`, the function returns early with that error.

---

## Summary & Next Step

You have mastered refutable and irrefutable pattern matching, match guards, `let else`, and recursive expression trees. Next, we explore object-oriented abstractions in Rust: **Methods, Traits, and Derives** in [02-methods-and-traits.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/02-day2-types-and-traits/02-methods-and-traits.md).
