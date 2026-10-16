# Chapter 20: Testing: Unit Tests, Integration Tests, and Clippy

Testing is built directly into Rust and Cargo. You do not need third-party test runners or assertion libraries to write robust, automated test suites.

---

## 1. Unit Testing in Rust

Unit tests verify small, isolated components of code. They are typically co-located in the same file as the source code inside a dedicated test module:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from outer module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        panic!("divide by zero");
    }
}
```

### 1.1 Key Attributes
- `#[cfg(test)]`: Tells the compiler to compile this module **only when running `cargo test`**. In production release binaries, test code is completely omitted.
- `#[test]`: Marks a function as a test runner entry point.
- `#[should_panic]`: Asserts that the test function must panic to pass.

---

## 2. Integration Tests

Integration tests live in a dedicated `tests/` directory at the project root:

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

- Each file in `tests/` is compiled as a **completely separate crate**.
- Tests can only call the **public API** (`pub`) of your library, treating it as an external user would.

```rust
// tests/integration_test.rs
use my_project::add;

#[test]
fn test_external_add() {
    assert_eq!(add(10, 20), 30);
}
```

---

## 3. Documentation Tests (Doc-Tests)

Rust compiles and executes code snippets embedded inside doc comments (`///`):

```rust
/// Calculates the sum of two integers.
///
/// # Examples
/// ```
/// use my_project::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

When you run `cargo test`, Cargo extracts and executes these code blocks! This guarantees that your public API documentation **never goes out of date**.

---

## 4. Compiler Lints & Clippy

Rust includes two levels of static analysis:
1. **Compiler Lints:** Built into `rustc` (detecting unused variables, dead code, unreachable branches).
2. **Clippy:** The official Rust linter (`cargo clippy`) that identifies anti-patterns, performance pitfalls, and unidiomatic code.

### Lint Levels:
```rust
#[allow(unused_variables)] // Suppress specific warning
#[deny(clippy::all)]        // Elevate lints to hard compiler errors
```

---

## 5. Exercise: Validating Numbers with the Luhn Algorithm

### Problem Description
The **Luhn algorithm** (mod 10 algorithm) validates identification numbers such as credit cards:
1. Ignore all whitespace. If string contains non-digits, it is invalid.
2. The number must have a length greater than 1.
3. Starting from the **rightmost digit**, double every second digit.
4. If doubling results in a number greater than 9, subtract 9 from it.
5. Sum all digits. If the total sum is evenly divisible by 10 (`sum % 10 == 0`), the number is valid.

Implement `fn luhn_validate(code: &str) -> bool` and write unit tests covering valid numbers, invalid checksums, whitespace, and invalid characters.

### Solution & Deep-Dive

```rust
pub fn luhn_validate(code: &str) -> bool {
    let sanitized: Vec<u32> = match code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).ok_or(()))
        .collect::<Result<Vec<u32>, ()>>()
    {
        Ok(digits) => digits,
        Err(_) => return false, // Contained non-digit character
    };

    if sanitized.len() <= 1 {
        return false;
    }

    let mut sum = 0;
    // Iterate from right to left
    for (i, &digit) in sanitized.iter().rev().enumerate() {
        if i % 2 == 1 {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numbers() {
        assert!(luhn_validate("046 454 286"));
        assert!(luhn_validate("49927398716"));
        assert!(luhn_validate("0")); // Invalid: len 1
    }

    #[test]
    fn test_invalid_checksum() {
        assert!(!luhn_validate("49927398717"));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(!luhn_validate("055a 444 285"));
        assert!(!luhn_validate("055-444-285"));
    }
}

fn main() {
    assert!(luhn_validate("046 454 286"));
    println!("Luhn algorithm verified successfully!");
}
```

---

## Summary & Next Step

You have mastered unit tests, integration tests, doctests, and static analysis with Clippy. Next, we build resilient production software in **Error Handling: Panics, Result, thiserror, and anyhow** in [03-error-handling.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/03-error-handling.md).
