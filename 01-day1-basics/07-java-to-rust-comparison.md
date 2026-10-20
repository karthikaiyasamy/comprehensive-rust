# Chapter 7: Java to Rust Comparison Guide for Java Developers

If you're coming from Java, Rust's syntax may look familiar, but its mental model is profoundly different. This guide bridges the gap by showing side-by-side comparisons and explaining **why** Rust does things differently.

---

## 1. Variable Declarations & Mutability

### Java
```java
// Immutable by default (final)
final int x = 10;
int y = 20;  // Mutable
y = 30;      // OK
```

### Rust
```rust
// MUTABLE by default (no keyword needed)
let x = 10;      // Immutable (cannot reassign)
let mut y = 20;  // Mutable (can reassign)
y = 30;          // OK
```

### Why This Difference?
- **Rust's philosophy:** Make the default safe. Immutable is safer for concurrency and reasoning.
- **Java inherited C++ convention:** Mutable is default because it's more intuitive for OOP beginners.

### Java Developer Pain Point & Fix
❌ **Common mistake:** Forgetting `mut` and getting "cannot assign twice" error.

✅ **Solution:** Train yourself to think "Rust is conservative—assume immutability unless you explicitly need mutation."

---

## 2. Memory Management: GC vs Ownership

### Java
```java
public class Person {
    private String name;
    
    public Person(String name) {
        this.name = name;
    }
    
    public void greet() {
        System.out.println("Hello, " + name);
    }
}

// Memory is managed by Garbage Collector (GC)
Person p1 = new Person("Alice");
Person p2 = p1;  // Both references point to same object
p1 = null;       // p2 still works, GC cleans up later
```

### Rust
```rust
struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Person { name }
    }
    
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }
}

// Memory is managed by ownership system
let p1 = Person::new(String::from("Alice"));
let p2 = p1;  // MOVE occurs! p1 is invalidated
// p1.greet(); // COMPILE ERROR: borrow of moved value: `p1`
p2.greet();
```

### Mental Model Shift
```
┌─────────────────────────────────────┐
│           Java (GC)                 │
├─────────────────────────────────────┤
│ - Heap allocations everywhere       │
│ - GC pauses stop-the-world          │
│ - References are always pointers    │
│ - No compile-time aliasing checks   │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│          Rust (Ownership)           │
├─────────────────────────────────────┤
│ - Stack-first allocation            │
│ - Zero runtime overhead             │
│ - References have strict rules      │
│ - Compile-time aliasing enforcement │
└─────────────────────────────────────┘
```

### Java Developer Pain Point & Fix
❌ **Common mistake:** Trying to pass objects by reference expecting them to remain valid after move.

✅ **Solution:** Think in terms of **ownership transfer**:
- Java `obj1 = obj2` → Both variables reference same object
- Rust `let p2 = p1` → Ownership moves to p2, p1 is invalidated

---

## 3. Methods & Behavior: Classes vs Traits

### Java
```java
public class Rectangle {
    private double width;
    private double height;
    
    public Rectangle(double width, double height) {
        this.width = width;
        this.height = height;
    }
    
    public double area() {
        return width * height;
    }
    
    public void scale(double factor) {
        width *= factor;
        height *= factor;
    }
}
```

### Rust
```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Constructor (associated function)
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
    
    // Immutable borrow receiver (&self)
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    // Mutable borrow receiver (&mut self)
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

### Key Differences

| Java | Rust |
|------|------|
| Behavior is bundled inside class | Behavior is external (impl blocks) |
| Inheritance tree (extends/implements) | Composition + Traits (code reuse) |
| Virtual method dispatch (runtime) | Static dispatch by default (compile-time) |
| Single class per file (usually) | Multiple impl blocks, one per type |

### Rust's Trait System: The "Interface on Steroids"
```rust
// Define behavior contract (like Java interface)
trait Describable {
    fn describe(&self) -> String;
    
    // Default implementation (like default methods in Java 8+)
    fn print_description(&self) {
        println!("{}", self.describe());
    }
}

// Implement trait for any type
struct Point { x: i32, y: i32 }

impl Describable for Point {
    fn describe(&self) -> String {
        format!("Point at ({}, {})", self.x, self.y)
    }
}
```

### Why Rust's Approach is Better
1. **No inheritance hell:** Composition is preferred over deep inheritance trees
2. **Zero-cost abstractions:** Static dispatch via monomorphization (no vtable lookup)
3. **Trait coherence:** Prevents conflicting trait implementations

### Java Developer Pain Point & Fix
❌ **Common mistake:** Looking for `extends` or `implements` keywords.

✅ **Solution:** Learn the "Rust way":
- Use **traits** instead of interfaces
- Use **composition** instead of inheritance
- Use **generics with trait bounds** instead of interface-based polymorphism

---

## 4. Type System: Newtypes vs Primitive Obsession

### Java (Primitive Obsession)
```java
public class UserId {
    private final long value;
    
    public UserId(long value) {
        this.value = value;
    }
    
    public long getValue() {
        return value;
    }
}

// Manual wrapper required to avoid confusion
public void processUser(UserId userId, OrderId orderId) {
    // No compile-time protection against mixing IDs
}
```

### Rust (Newtype Pattern - Built-in)
```rust
// Zero-cost abstraction: compiles to the same as u64
struct UserId(u64);
struct OrderId(u64);

// Compile-time protection: cannot mix types
fn process_user(user_id: UserId, order_id: OrderId) {
    // compiler error if arguments swapped!
}

// Usage
let user = UserId(123);
let order = OrderId(456);
process_user(user, order);
// process_user(order, user); // COMPILE ERROR: mismatched types
```

### Memory Layout Comparison
```
Java:                              Rust (Newtype):
┌─────────────┐                   ┌─────────────┐
│ UserId      │                   │ UserId      │
│ [long value]│  ── heap alloc ──▶│ [u64 value] │  ── stack alloc
└─────────────┘                   └─────────────┘
       │                                  │
       ▼                                  ▼
┌─────────────┐                   ┌─────────────┐
│ long: 123   │                   │ u64: 123    │
└─────────────┘                   └─────────────┘
       16 bytes (object header + ref)     8 bytes (same as u64!)
```

### Why This Matters
- **Java:** Wrapper classes have object overhead (16+ bytes)
- **Rust:** Newtypes compile to zero cost (same as primitive)

### Java Developer Pain Point & Fix
❌ **Common mistake:** Thinking "I need to define a full class with getters/setters."

✅ **Solution:** Learn the Newtype pattern:
```rust
// One-line type definition
struct Email(String);
struct Password(String);

// Auto-implement traits via derive macro
#[derive(Debug, Clone)]
struct UserId(u64);
```

---

## 5. Error Handling: Exceptions vs Result Type

### Java (Exceptions)
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

### Rust (Result Type)
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

// Or use ? operator for short-circuiting
fn process() -> Result<(), String> {
    let x = divide(10, 2)?;  // Returns early if Err
    let y = divide(x, 0)?;   // This will short-circuit
    Ok(())
}
```

### Mental Model: Result as a "Box That Might Contain an Error"
```
┌─────────────────────────────────────────┐
│              Java Exceptions            │
├─────────────────────────────────────────┤
│ - Checked (must declare throws)        │
│ - Unchecked (runtime, no declaration)   │
│ - Stack unwinding (performance cost)    │
│ - Can be caught anywhere in call stack  │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│              Rust Result                │
├─────────────────────────────────────────┤
│ - Single type: Result<T, E>            │
│ - No stack unwinding (compile-time)     │
│ - Must be explicitly handled            │
│ - Cannot be accidentally ignored        │
└─────────────────────────────────────────┘
```

### Why Rust's Approach is Better
1. **Exhaustiveness:** `match` forces you to handle both cases
2. **No silent failures:** You can't accidentally ignore a `Result`
3. **Composability:** `?` operator makes error propagation ergonomic

### Java Developer Pain Point & Fix
❌ **Common mistake:** Ignoring `Result` or using `.unwrap()` everywhere (like Java's `try-catch` without proper handling).

✅ **Solution:** Learn error handling patterns:
```rust
// For recoverable errors in libraries: return Result
pub fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// For application-level errors: use anyhow! or thiserror!
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

---

## 6. Concurrency: Threads & Synchronization

### Java
```java
// Shared state with explicit synchronization
private final Object lock = new Object();
private int counter = 0;

public void increment() {
    synchronized(lock) {
        counter++;
    }
}

// Thread pool
ExecutorService pool = Executors.newFixedThreadPool(4);
pool.submit(() -> {
    increment();
});
```

### Rust
```rust
// Shared state with type system protection
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

let handle = std::thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});

// Wait for thread to complete
handle.join().unwrap();
```

### Mental Model: Thread-Safety as a Type System Feature
```
┌─────────────────────────────────────────┐
│           Java Concurrency             │
├─────────────────────────────────────────┤
│ - Runtime synchronization (synchronized │
│   blocks, Lock interfaces)              │
│ -容易发生 data races (忘记同步)            │
│ - Deadlocks possible (circular wait)     │
│ - Memory model complex (JMM)            │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│          Rust Concurrency              │
├─────────────────────────────────────────┤
│ - Compile-time safety guarantees       │
│ - Send + Sync traits enforce thread    │
│   safety                                │
│ - No data races possible (compile-time) │
│ - Deadlocks still possible but rare     │
└─────────────────────────────────────────┘
```

### The `Send` and `Sync` Traits: Your Thread-Safety Checklist
```
Send: Can be transferred to another thread
      (Owner can move across thread boundary)

Sync: Can be shared between threads
      (Shared reference &T can be sent to another thread)

Standard Library Types:
┌─────────────┬──────┬──────┐
│    Type     │ Send │ Sync │
├─────────────┼──────┼──────┤
│ String      │ Yes  │ Yes  │
│ Vec<T>      │ Yes  │ Yes  │
│ Arc<T>      │ Yes  │ Yes* │
│ Mutex<T>    │ Yes* │ Yes  │
│ RefCell<T>  │ No   │ No   │
│ Rc<T>       │ No   │ No   │
└─────────────┴──────┴──────┘
* Requires T: Send
```

### Java Developer Pain Point & Fix
❌ **Common mistake:** Trying to share `Rc<T>` or `RefCell<T>` across threads (won't compile).

✅ **Solution:** Always use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared mutable state across threads:
```rust
use std::sync::{Arc, Mutex};

// Correct pattern
let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

// Clone Arc for each thread
let data_clone = Arc::clone(&shared_data);
let handle = std::thread::spawn(move || {
    let mut data = data_clone.lock().unwrap();
    data.push(4);
});
```

---

## 7. Advanced Type System Concepts

### Phantom Types: Type-Level Programming

#### Java (Limited)
```java
// Cannot encode state in type system
enum Status { PENDING, APPROVED, REJECTED }

class Document {
    private Status status;
    // Runtime check required
}
```

#### Rust (Full Power)
```rust
// Zero-cost type-level state machine
struct Pending;
struct Approved;
struct Rejected;

struct Document<T> {
    content: String,
    _state: std::marker::PhantomData<T>,
}

impl Document<Pending> {
    fn new(content: &str) -> Self {
        Document {
            content: content.to_string(),
            _state: std::marker::PhantomData,
        }
    }
    
    // Only pending documents can be approved
    fn approve(self) -> Document<Approved> {
        Document {
            content: self.content,
            _state: std::marker::PhantomData,
        }
    }
}

impl Document<Approved> {
    fn reject(self) -> Document<Rejected> {
        Document {
            content: self.content,
            _state: std::marker::PhantomData,
        }
    }
}

// Usage: Cannot approve a rejected document!
let doc = Document::new("Draft");
let approved = doc.approve();
// approved.approve(); // COMPILE ERROR: no method `approve` on Document<Approved>
```

### Why This Matters
- **Java:** Runtime checks with if/else
- **Rust:** Compile-time enforcement (zero runtime cost!)

---

## Summary: Mental Model Checklist

| Concept | Java | Rust | Mental Shift Required |
|---------|------|------|----------------------|
| **Variables** | `final` for immutability | `mut` for mutability | Think "immutability by default" |
| **Memory** | Garbage Collector | Ownership system | Think "explicit ownership transfer" |
| **Behavior** | Bundled in class | External impl blocks | Think "separate data from behavior" |
| **Polymorphism** | Inheritance + Interfaces | Traits + Generics | Think "traits as contracts" |
| **Errors** | Exceptions | Result<T, E> | Think "errors as values" |
| **Concurrency** | synchronized blocks | Send + Sync traits | Think "type system enforces safety" |
| **Types** | Classes only | Newtypes, generics, traits | Think "type system as programming language" |

### Next Steps for Java Developers
1. **Week 1:** Practice Newtype pattern and Result error handling
2. **Week 2:** Master ownership/borrowing with visual diagrams
3. **Week 3:** Learn traits as Rust's interface on steroids
4. **Week 4:** Deep dive into type-level programming with PhantomData

### Recommended Practice Projects
1. **Type-Safe API Client:** Use newtypes for endpoints, Result for responses
2. **State Machine with Types:** Document approval flow using phantom types
3. **Concurrent Counter:** Arc<Mutex<i32>> pattern practice
4. **Generic Data Processor:** Implement traits for custom processing

---

## Appendix: Quick Reference Table

| Java Concept | Rust Equivalent | Notes |
|--------------|-----------------|-------|
| `final` variable | `let` (immutable) | Rust is opposite: mut needs keyword |
| `interface` | `trait` | Rust traits can have defaults |
| `class` | `struct` + `impl` | Behavior separate from data |
| `extends` | `trait: OtherTrait` | Supertraits |
| `implements` | `impl Trait for Type` | Orphan rule applies |
| `throw new Exception()` | `Err(error)` | Result type |
| `try { } catch { }` | `match result { Ok => , Err => }` | Exhaustive matching |
| `synchronized(block)` | `MutexGuard` (RAII) | Lock auto-releases |
| `ExecutorService` | `std::thread::spawn` | Manual thread management |
| `HashMap<K,V>` | `HashMap<K,V>` | Same API! |
| `Optional<T>` | `Option<T>` | Same concept, different naming |
| `Stream<T>` | `Iterator<T>` | Similar methods (map, filter) |
| `CompletableFuture<T>` | `Future<T>` + `tokio` | Async is explicit in Rust |
