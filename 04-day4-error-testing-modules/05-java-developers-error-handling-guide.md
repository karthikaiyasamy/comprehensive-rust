# Chapter 23: Error Handling: Java vs Rust - Complete Guide for Java Developers

This chapter provides a **comprehensive comparison** of error handling in Java and Rust, with practical examples and best practices.

---

## 1. Philosophy: Exceptions vs. Result Types

### Java: Exceptions (Control Flow)
```java
public int divide(int a, int b) throws Exception {
    if (b == 0) {
        throw new Exception("Division by zero");
    }
    return a / b;
}

try {
    int result = divide(10, 0);
} catch (Exception e) {
    System.err.println("Error: " + e.getMessage());
}
```

### Rust: Result Types (Data)
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

// Caller must handle the error explicitly
match divide(10, 0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### Key Difference
- **Java:** Errors are **control flow** (exceptions unwind the stack)
- **Rust:** Errors are **data** (Result is just a type like any other)

---

## 2. Compile-Time Guarantees

### Java: No Exhaustiveness
```java
try {
    riskyOperation();
} catch (SpecificException e) {
    // What if there are other exceptions?
    // Compiler doesn't check!
}
```

### Rust: Exhaustive Pattern Matching
```rust
match risky_operation() {
    Ok(result) => println!("Success: {}", result),
    Err(e) => println!("Error: {}", e),
    // Compiler ensures ALL cases handled!
}
```

---

## 3. The `?` Operator: Composable Error Propagation

### Java: Verbose try-catch
```java
public String readConfig() throws IOException {
    try {
        String content = Files.readString(Path.of("config.txt"));
        try {
            return parseConfig(content);
        } catch (ParseException e) {
            throw new IOException("Failed to parse config", e);
        }
    } catch (IOException e) {
        throw new IOException("Failed to read config", e);
    }
}
```

### Rust: Ergonomic `?` Operator
```rust
fn read_config() -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.txt")?;
    parse_config(content)
}
```

---

## 4. Custom Error Types

### Java: Exception Hierarchy
```java
public class ConfigError extends Exception {
    public ConfigError(String message) {
        super(message);
    }
}

public class ConfigParseError extends ConfigError {
    public ConfigParseError(String message, Exception cause) {
        super(message);
        initCause(cause);
    }
}
```

### Rust: thiserror crate (Clean, Declarative)
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    
    #[error("Failed to parse config: {0}")]
    Parse(#[from] serde_json::Error),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
}

// Usage
fn read_config() -> Result<String, ConfigError> {
    let content = std::fs::read_to_string("config.txt")?; // Auto-converts to ConfigError
    Ok(content)
}
```

### Why thiserror is Better
1. **Declarative:** Use attributes to define errors
2. **Automatic conversions:** `#[from]` derives `From` implementations
3. **No boilerplate:** No manual `cause` chaining

---

## 5. anyhow: Ergonomic Application Errors

### Java: Manual Context
```java
try {
    operation();
} catch (Exception e) {
    throw new RuntimeException("Failed to process: " + e.getMessage(), e);
}
```

### Rust: anyhow with Context
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.txt")
        .context("Failed to read config file")?;
    
    Ok(())
}
```

### Difference from thiserror
- **thiserror:** For libraries (public API, explicit error types)
- **anyhow:** For applications (ergonomic, automatic error chaining)

---

## 6. Real-World Example: API Client

### Java Version (Exceptions)
```java
public class ApiClient {
    public User getUser(String userId) throws IOException, ApiException {
        String response;
        try {
            response = httpClient.get("/users/" + userId);
        } catch (SocketTimeoutException e) {
            throw new IOException("Request timeout", e);
        }
        
        if (response.isEmpty()) {
            throw new ApiException("Empty response");
        }
        
        try {
            return parseUser(response);
        } catch (JsonParseException e) {
            throw new ApiException("Invalid JSON", e);
        }
    }
}
```

### Rust Version (Result + anyhow)
```rust
use anyhow::{Context, Result};

struct ApiClient {
    base_url: String,
}

impl ApiClient {
    fn get_user(&self, user_id: &str) -> Result<User> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        
        let response = reqwest::get(&url)
            .context("Failed to make HTTP request")?
            .text()
            .context("Failed to read response body")?;
        
        if response.is_empty() {
            anyhow::bail!("Empty response from server");
        }
        
        serde_json::from_str(&response)
            .context("Failed to parse user JSON")
    }
}

// Usage
fn main() -> Result<()> {
    let client = ApiClient {
        base_url: "https://api.example.com".to_string(),
    };
    
    match client.get_user("123") {
        Ok(user) => println!("User: {:?}", user),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Caused by: {}", e.source().unwrap());
        }
    }
    
    Ok(())
}
```

### Key Differences
| Java | Rust |
|------|------|
| Multiple catch blocks | Single `?` operator |
| Manual error chaining | Automatic with `context()` |
| Runtime exceptions | Compile-time Result type |

---

## 7. Best Practices for Java Developers

### 1. For Libraries: Use thiserror
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyLibraryError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, MyLibraryError>;
```

### 2. For Applications: Use anyhow
```rust
use anyhow::Result;

fn main() -> Result<()> {
    // All errors automatically convert to anyhow::Error
    let data = load_config()?;
    process_data(data)?;
    save_results()?;
    
    Ok(())
}
```

### 3. Use `?` Operator for Propagation
```rust
fn process() -> Result<()> {
    let file = File::open("data.txt")?; // Returns early on Err
    let content = read_file(&file)?;
    parse_content(content)?;
    
    Ok(())
}
```

### 4. Add Context with `.context()`
```rust
let config = std::fs::read_to_string("config.json")
    .context("Failed to read config file")?;
```

---

## 8. Debugging Error Handling

### Use `RUST_BACKTRACE=1`
```bash
RUST_BACKTRACE=1 cargo run
```

This shows the full backtrace when a panic occurs.

### Print Error Chain
```rust
use anyhow::Result;

fn main() -> Result<()> {
    // ...
    Ok(())
}

// In catch block
Err(e) => {
    eprintln!("Error: {}", e);
    for source in e.chain() {
        eprintln!("Caused by: {}", source);
    }
}
```

---

## 9. Migration Guide: Converting Java Code

### Pattern 1: Simple Method with Error
```java
// Java
public Result process() throws ValidationException, DatabaseException {
    validateInput();
    saveToDatabase();
    return new Result();
}
```

```rust
// Rust
fn process() -> Result<Result, ProcessError> {
    validate_input()?;
    save_to_database()?;
    Ok(Result::new())
}

#[derive(Error, Debug)]
enum ProcessError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
```

### Pattern 2: Method with Multiple Error Types
```java
// Java
public String readConfig() throws IOException, ParseException {
    String content = Files.readString(path);
    return parse(content);
}
```

```rust
// Rust
fn read_config() -> Result<String, ConfigError> {
    let content = std::fs::read_to_string("config.txt")?;
    parse(&content)
}

#[derive(Error, Debug)]
enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}
```

---

## 10. Summary: Mental Model Mapping

| Concept | Java | Rust |
|---------|------|------|
| **Error Representation** | Exceptions (control flow) | Result<T, E> (data type) |
| **Error Propagation** | try-catch blocks | `?` operator |
| **Custom Errors** | Exception hierarchy | Enum with thiserror |
| **Context** | Manual chaining | `.context()` method |
| **Exhaustiveness** | Runtime checks | Compile-time matching |
| **Error Library** | N/A (built-in) | thiserror, anyhow |

### Quick Reference: Common Patterns

#### Pattern 1: Library with Public Errors
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, MyError>;
```

#### Pattern 2: Application with anyhow
```rust
use anyhow::Result;

fn main() -> Result<()> {
    // All errors automatically convert
    let data = load()?;
    process(data)?;
    Ok(())
}
```

#### Pattern 3: Chaining with Context
```rust
let data = std::fs::read_to_string("file.txt")
    .context("Failed to read file")?;
```

---

## 11. Practice Exercises

### Exercise 1: Convert Java API to Rust
```java
// Java API
public class UserService {
    public User createUser(String name, String email) 
        throws ValidationException, DatabaseException {
        validateName(name);
        validateEmail(email);
        return database.insertUser(name, email);
    }
}
```

**Solution:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, UserServiceError>;

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(UserServiceError::InvalidName("Name cannot be empty".to_string()));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') {
        return Err(UserServiceError::InvalidEmail("Invalid email format".to_string()));
    }
    Ok(())
}

fn create_user(database: &mut Database, name: &str, email: &str) -> Result<User> {
    validate_name(name)?;
    validate_email(email)?;
    database.insert_user(name, email)
}
```

### Exercise 2: Error Chain
Implement a function that reads a config file and returns a parsed configuration.
- Use anyhow for application-level errors
- Add context to each operation
- Print full error chain on failure

**Solution:**
```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read config file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config JSON")?;
    
    Ok(config)
}

fn main() -> Result<()> {
    match load_config() {
        Ok(config) => println!("Config: {:?}", config),
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            for source in e.chain() {
                eprintln!("  Caused by: {}", source);
            }
        }
    }
    
    Ok(())
}
```

---

## 12. Final Advice for Java Developers

### Don't Fight the Result Type
The Result type is not a limitation—it's a feature. It forces you to think about error handling explicitly.

### Use the Right Library
- **thiserror** for libraries (public API)
- **anyhow** for applications (ergonomic)

### Embrace the `?` Operator
The `?` operator is one of Rust's most powerful features. Use it everywhere.

### Read Error Messages Carefully
Rust's error messages are incredibly helpful. Read them word-for-word.

---

**Remember**: Every Java developer has struggled with Rust's error handling at first. But once you embrace the Result type, you'll never want to go back to exceptions for recoverable errors.
