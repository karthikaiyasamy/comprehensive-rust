# Chapter 14: Enhanced Exercise Solutions for Java Developers

This chapter provides **detailed solutions with Java-to-Rust comparisons**, common mistakes, and deep dives for each exercise from the original curriculum.

---

## Exercise 1: Fibonacci Sequence

### Original Problem
Write a function `fib(n: u32) -> u32` that calculates the n-th Fibonacci number.

### Common Java Developer Mistake
```rust
// ❌ Wrong: assumes Java-style mutable defaults
fn fib(n: u32) -> u32 {
    let prev = 0;  // immutable
    let curr = 1;  // immutable
    
    for i in 2..=n {
        let next = prev + curr;
        prev = curr; // ERROR: cannot assign twice
        curr = next;
    }
    
    curr
}
```

### Correct Solution
```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    
    // Use mutable variables explicitly
    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

// Or use recursive version (less efficient but demonstrates match)
fn fib_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}
```

### Java-to-Rust Comparison

#### Java Version
```java
public static long fib(int n) {
    if (n < 2) return n;
    
    long prev = 0;
    long curr = 1;
    
    for (int i = 2; i <= n; i++) {
        long next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    return curr;
}
```

#### Key Difference
- **Java:** Variables mutable by default (no `final`)
- **Rust:** Variables immutable by default (need `mut`)

### Deep Dive: When Does It Panic?
`u32` max is 4,294,967,295. Fibonacci at n=48 overflows:
```rust
// F(47) = 2,971,215,073 (fits)
// F(48) = 4,807,526,976 (overflows u32!)

// In debug mode: panics
// In release mode: wraps to 512,559,680 (incorrect!)
```

### Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_recursive() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_fib_debug_panic() {
        fib(48); // Panics in debug mode
    }
}
```

---

## Exercise 2: Arithmetic Expression Evaluation

### Original Problem
Implement a parser and evaluator for arithmetic expressions:
- `Expression` enum: `Number(i32)`, `Add(Box<Expression>, Box<Expression>)`, `Sub`, `Mul`, `Div`
- Support parentheses and operator precedence

### Common Java Developer Mistake
```rust
// ❌ Wrong: using Java-style inheritance
struct Add {
    left: Box<Expression>,
    right: Box<Expression>,
}

enum Expression {
    Number(i32),
    Add(Add), // Java-style: separate struct for each type
    // ...
}
```

### Correct Solution (Rust Style)
```rust
#[derive(Debug, Clone)]
enum Expression {
    Number(i32),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(n) => *n,
            Expression::Add(left, right) => left.evaluate() + right.evaluate(),
            Expression::Sub(left, right) => left.evaluate() - right.evaluate(),
            Expression::Mul(left, right) => left.evaluate() * right.evaluate(),
            Expression::Div(left, right) => {
                if right.evaluate() == 0 {
                    panic!("Division by zero");
                }
                left.evaluate() / right.evaluate()
            }
        }
    }
}

// Java equivalent would require:
// - Interface Expression
// - Classes implementing Expression
// - Runtime instanceof checks
// Rust: Single enum, exhaustive match
```

### Java-to-Rust Comparison

#### Java Version (Inheritance-based)
```java
interface Expression {
    int evaluate();
}

class Number implements Expression {
    private int value;
    public int evaluate() { return value; }
}

class Add implements Expression {
    private Expression left, right;
    public int evaluate() { return left.evaluate() + right.evaluate(); }
}
```

#### Rust Version (Enum-based)
```rust
enum Expression {
    Number(i32),
    Add(Box<Expression>, Box<Expression>),
    // ...
}

impl Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(n) => *n,
            // ...
        }
    }
}
```

### Key Differences
| Java | Rust |
|------|------|
| Inheritance tree | Single enum |
| Runtime type checks | Compile-time exhaustive match |
| Virtual dispatch | Static dispatch (no vtable) |

---

## Exercise 3: Logger Trait

### Original Problem
Implement a logger trait with different implementations:
- `ConsoleLogger`: Prints to stdout
- `FileLogger`: Writes to a file
- `FilteredLogger`: Decorator that filters messages

### Solution
```rust
trait Logger {
    fn log(&self, message: &str);
}

// Console logger
struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[CONSOLE] {}", message);
    }
}

// File logger
use std::fs::OpenOptions;
use std::io::Write;

struct FileLogger {
    path: String,
}

impl FileLogger {
    fn new(path: &str) -> Self {
        FileLogger { path: path.to_string() }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Failed to open file");
        
        writeln!(file, "[FILE] {}", message).expect("Failed to write");
    }
}

// Filtered logger (decorator pattern)
struct FilteredLogger<L: Logger> {
    logger: L,
    filter: Box<dyn Fn(&str) -> bool>,
}

impl<L: Logger> FilteredLogger<L> {
    fn new(logger: L, filter: impl Fn(&str) -> bool + 'static) -> Self {
        FilteredLogger {
            logger,
            filter: Box::new(filter),
        }
    }
}

impl<L: Logger> Logger for FilteredLogger<L> {
    fn log(&self, message: &str) {
        if (self.filter)(message) {
            self.logger.log(message);
        }
    }
}

// Usage
fn main() {
    let console = ConsoleLogger;
    let file = FileLogger::new("app.log");
    
    let debug_logger = FilteredLogger::new(console, |msg| msg.contains("[DEBUG]"));
    let info_logger = FilteredLogger::new(file, |msg| msg.contains("[INFO]"));
    
    debug_logger.log("[DEBUG] Starting application");
    info_logger.log("[INFO] Application started");
}
```

### Java-to-Rust Comparison

#### Java Version
```java
interface Logger {
    void log(String message);
}

class ConsoleLogger implements Logger {
    public void log(String msg) { System.out.println(msg); }
}

class FileLogger implements Logger {
    public void log(String msg) { /* write to file */ }
}

class FilteredLogger implements Logger {
    private Logger logger;
    private Predicate<String> filter;
    
    public void log(String msg) {
        if (filter.test(msg)) logger.log(msg);
    }
}
```

#### Key Differences
- **Java:** Interfaces + inheritance
- **Rust:** Traits + composition (no inheritance)

### Deep Dive: Why Traits Are Better
1. **No inheritance hierarchy:** Composition is more flexible
2. **Zero-cost abstractions:** Static dispatch (no vtable lookup)
3. **Orphan rule:** Prevents conflicting implementations

---

## Exercise 4: ROT13 Streaming Decoder

### Original Problem
Implement `std::io::Read` for a ROT13 decoder.

### Solution
```rust
use std::io;

struct Rot13Reader<R: io::Read> {
    inner: R,
}

impl<R: io::Read> Rot13Reader<R> {
    fn new(reader: R) -> Self {
        Rot13Reader { inner: reader }
    }
}

impl<R: io::Read> io::Read for Rot13Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Read from inner reader
        let bytes_read = self.inner.read(buf)?;
        
        // Apply ROT13 transformation
        for byte in &mut buf[..bytes_read] {
            *byte = rot13(*byte);
        }
        
        Ok(bytes_read)
    }
}

fn rot13(byte: u8) -> u8 {
    match byte {
        b'A'..=b'Z' => {
            let offset = byte - b'A';
            b'A' + ((offset + 13) % 26)
        }
        b'a'..=b'z' => {
            let offset = byte - b'a';
            b'a' + ((offset + 13) % 26)
        }
        _ => byte,
    }
}

// Usage
fn main() -> io::Result<()> {
    let encoded = b"Uryyb Jbeyq"; // "Hello World" in ROT13
    let mut reader = Rot13Reader::new(&encoded[..]);
    let mut buffer = [0u8; 100];
    
    reader.read(&mut buffer)?;
    let decoded = String::from_utf8_lossy(&buffer);
    assert_eq!(decoded, "Hello World");
    
    Ok(())
}
```

### Java-to-Rust Comparison

#### Java Version (Stream decorator)
```java
class Rot13Reader extends FilterInputStream {
    public Rot13Reader(InputStream in) {
        super(in);
    }
    
    public int read() throws IOException {
        int b = in.read();
        if (b == -1) return -1;
        return rot13((byte)b);
    }
    
    private int rot13(byte b) {
        // implementation
    }
}
```

#### Key Differences
- **Java:** Class inheritance
- **Rust:** Traits (composition over inheritance)

---

## Exercise 5: Luhn Algorithm

### Original Problem
Validate identification numbers using the Luhn algorithm.

### Solution with Tests
```rust
pub fn luhn_validate(code: &str) -> bool {
    // Filter whitespace and parse digits
    let digits: Vec<u32> = match code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>() // Returns None if any non-digit
    {
        Some(d) => d,
        None => return false,
    };
    
    // Must have more than 1 digit
    if digits.len() <= 1 {
        return false;
    }
    
    // Luhn algorithm
    let mut sum = 0;
    for (i, &digit) in digits.iter().rev().enumerate() {
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
    }

    #[test]
    fn test_invalid_checksum() {
        assert!(!luhn_validate("49927398717"));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(!luhn_validate("055a 444 285"));
    }

    #[test]
    fn test_too_short() {
        assert!(!luhn_validate("0"));
    }
}
```

### Java-to-Rust Comparison

#### Java Version
```java
public static boolean luhnValidate(String code) {
    String sanitized = code.replaceAll("\\s+", "");
    if (sanitized.length() <= 1) return false;
    
    int sum = 0;
    for (int i = sanitized.length() - 1; i >= 0; i--) {
        int digit = Character.getNumericValue(sanitized.charAt(i));
        if ((sanitized.length() - 1 - i) % 2 == 1) {
            digit *= 2;
            if (digit > 9) digit -= 9;
        }
        sum += digit;
    }
    
    return sum % 10 == 0;
}
```

#### Key Differences
- **Java:** Exception on invalid characters, runtime validation
- **Rust:** `Option` type, compile-time guarantee of valid digits

---

## Exercise 6: Type-Safe Builder Pattern

### Original Problem
Implement a builder pattern that consumes the builder on `.build()`.

### Solution
```rust
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    email: String,
    age: u32,
}

struct UserBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
        }
    }
    
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    // Consumes the builder by value
    fn build(self) -> Result<User, String> {
        let name = self.name.ok_or("Name is required")?;
        let email = self.email.ok_or("Email is required")?;
        let age = self.age.unwrap_or(0);
        
        Ok(User { name, email, age })
    }
}

// Usage
fn main() {
    let user = UserBuilder::new()
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .age(30)
        .build()
        .expect("Invalid user");
    
    assert_eq!(user.name, "Alice");
}
```

### Why This Pattern Is Powerful
1. **Compile-time guarantee:** Cannot use builder after `.build()`
2. **No intermediate state bugs:** Each method returns a fresh builder
3. **Explicit error handling:** `.build()` returns `Result`

### Java Equivalent (Without Move Semantics)
```java
public class User {
    private final String name;
    private final String email;
    private final int age;
    
    private User(Builder builder) {
        this.name = builder.name;
        this.email = builder.email;
        this.age = builder.age;
    }
    
    public static class Builder {
        private String name;
        private String email;
        private int age;
        
        public Builder name(String name) {
            this.name = name;
            return this;
        }
        
        // ... other setters
        
        public User build() {
            // Validation
            if (name == null) throw new IllegalStateException("Name required");
            return new User(this);
        }
    }
}
```

#### Key Difference
- **Java:** Builder remains valid after `.build()` (can modify and build again)
- **Rust:** Builder is consumed (move semantics), cannot reuse

---

## Summary: Java-to-Rust Patterns

### Pattern 1: Mutability
| Java | Rust |
|------|------|
| `var x = 10; x = 20;` | `let mut x = 10; x = 20;` |
| `final var x = 10;` | `let x = 10;` |

### Pattern 2: References
| Java | Rust |
|------|------|
| `obj1 = obj2;` (both valid) | `let y = x;` (x invalidated) |
| `obj.clone()` | `x.clone()` |

### Pattern 3: Interfaces vs Traits
| Java | Rust |
|------|------|
| `interface Logger { void log(); }` | `trait Logger { fn log(&self); }` |
| `class ConsoleLogger implements Logger` | `struct ConsoleLogger; impl Logger for ConsoleLogger` |

### Pattern 4: Error Handling
| Java | Rust |
|------|------|
| `throw new Exception()` | `Err(Error)` |
| `try { } catch { }` | `match result { Ok => , Err => }` |

### Pattern 5: Inheritance vs Composition
| Java | Rust |
|------|------|
| `class A extends B` | `struct A { b: B }` |
| `interface I` | `trait I` |

---

## Practice Exercises

### Exercise 1: Convert Java Code to Rust
```java
// Java code
public class Calculator {
    private double value;
    
    public Calculator() {
        this.value = 0;
    }
    
    public Calculator add(double x) {
        this.value += x;
        return this;
    }
    
    public Calculator multiply(double x) {
        this.value *= x;
        return this;
    }
    
    public double getValue() {
        return this.value;
    }
}
```

**Solution:**
```rust
struct Calculator {
    value: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { value: 0.0 }
    }
    
    fn add(mut self, x: f64) -> Self {
        self.value += x;
        self
    }
    
    fn multiply(mut self, x: f64) -> Self {
        self.value *= x;
        self
    }
    
    fn get_value(&self) -> f64 {
        self.value
    }
}
```

### Exercise 2: Type-Safe API Client
Create an API client where:
- Cannot send request before authentication
- Cannot reuse request after sending
- Compile-time guarantee of valid endpoints

**Hint:** Use phantom types to encode state.

---

**Next:** Move on to Day 3 exercises and master ownership/borrowing with the visual guide in Chapter 18.
