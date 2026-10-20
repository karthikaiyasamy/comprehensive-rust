# Chapter 8: Debugging Rust for Java Developers: Common Issues & Solutions

This guide covers the **most common issues** Java developers face when learning Rust, with step-by-step debugging strategies and real-world solutions.

---

## 1. The "Cannot Assign Twice" Error

### Problem
```rust
let x = 10;
x = 20; // ERROR: cannot assign twice to immutable variable `x`
```

### Java Developer Expectation
"In Java, variables are mutable by default. Why is Rust different?"

### Why This Happens
Rust's default is **immutable** to ensure thread safety and prevent accidental mutations.

### Solutions

#### Option 1: Use `mut` (Most Common)
```rust
let mut x = 10;
x = 20; // OK
```

#### Option 2: embrace immutability
```rust
let x = 10;
let x = x + 10; // Shadowing: creates new variable
println!("{}", x); // 20
```

### Debugging Checklist
1. Check if you actually need mutation
2. If yes, add `mut` to the variable declaration
3. If no, use shadowing for transformations

---

## 2. "Borrow of Moved Value" Error

### Problem
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // ERROR: borrow of moved value `s1`
```

### Java Developer Expectation
"In Java, `s2 = s1` would just copy the reference. Both should work."

### Why This Happens
Rust **moves ownership** by default instead of copying references. After the move, `s1` is invalidated.

### Solutions

#### Option 1: Clone the value
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Deep copy
println!("{}", s1); // OK, both valid
```

#### Option 2: Borrow instead (if function accepts reference)
```rust
fn takes_string_ref(s: &String) { println!("{}", s); }

let s1 = String::from("hello");
takes_string_ref(&s1); // Borrow, no move
println!("{}", s1); // OK, s1 still valid
```

### Debugging Checklist
1. Ask: "Do I need ownership or just read access?"
2. If read access: Use `&value` instead of `value`
3. If ownership: Use `.clone()` (but consider if you really need it)

---

## 3. "Cannot Borrow as Mutable More Than Once" Error

### Problem
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ERROR: cannot borrow as mutable more than once
```

### Java Developer Expectation
"In Java, I can have multiple references to the same object."

### Why This Happens
Rust enforces **aliasing XOR mutability**: either many `&T` OR one `&mut T`, never both.

### Solutions

#### Option 1: Use scopes to limit borrow lifetime
```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    r1.push_str(" world");
} // r1's borrow ends here

{
    let r2 = &mut s;
    r2.push_str("!");
} // r2's borrow ends here

println!("{}", s); // OK
```

#### Option 2: Use iteration instead of manual borrowing
```rust
let mut vec = vec![1, 2, 3];

// ❌ Wrong: cannot borrow twice
// let r1 = &mut vec[0];
// let r2 = &mut vec[1];

// ✅ Right: iteration handles borrowing
for item in &mut vec {
    *item *= 2;
}
```

### Debugging Checklist
1. Look for overlapping mutable borrows
2. Use scopes to limit borrow lifetimes
3. Consider if you can rewrite using iteration

---

## 4. "Missing lifetime specifier" Error

### Problem
```rust
fn longest(x: &str, y: &str) -> &str { // ERROR: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

### Java Developer Expectation
"In Java, references work without explicit lifetime annotations."

### Why This Happens
The compiler cannot determine how long the returned reference will be valid because it depends on both inputs.

### Solution: Add lifetime annotations
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Debugging Checklist
1. Check if function returns a reference
2. If yes, identify which input parameter's lifetime applies
3. Use lifetime parameters to express the relationship

---

## 5. The "Double Free" Fear

### Problem
Java developers often worry: "Won't Rust double-free memory?"

### Why This Fear Exists
In C/C++, manually freeing memory twice causes undefined behavior. Java developers are conditioned to fear this.

### Rust's Safety Guarantee
Rust **prevents double-free at compile time** through ownership tracking:

```rust
let s1 = String::from("hello");
let s2 = s1; // Move occurs
// s1 is now INVALIDATED
// Only s2 owns the String
// When s2 goes out of scope, drop() is called exactly ONCE
// s1 cannot be used, so it cannot be dropped again
```

### Proof: Compiler rejects double-free attempts
```rust
let s1 = String::from("hello");
drop(s1); // Explicitly drop s1
// println!("{}", s1); // ERROR: borrow of moved value: `s1`
```

---

## 6. Common Type Errors

### Problem 1: "Mismatched types"
```rust
let x: i32 = 10;
let y: i64 = x; // ERROR: mismatched types
```

### Solution: Explicit casting
```rust
let y: i64 = x as i64;
```

### Problem 2: "Cannot infer type"
```rust
let v = Vec::new(); // ERROR: cannot infer type
v.push(1);
```

### Solution: Annotate type
```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
```

---

## 7. Borrow Checker Debugging Tools

### Tool 1: `cargo expand`
Show monomorphized code to understand what the compiler is doing.

```bash
cargo install cargo-expand
cargo expand > expanded.rs
```

### Tool 2: Rust Playground
Use https://play.rust-lang.org/ to experiment with small code snippets and see compiler errors.

### Tool 3: Rust Analyzer in VS Code
- Hover over variables to see types
- Click "Go to Definition" to see trait implementations
- See live borrow checker annotations

---

## 8. Debugging Workflow for Java Developers

### Step 1: Read the error message carefully
Rust compiler errors are famously helpful. Read them word-for-word.

### Step 2: Identify the ownership question
Ask: "Who owns this value right now?"

### Step 3: Check the borrow status
Ask: "Is it borrowed as `&` or `&mut`?"

### Step 4: Use scopes to limit borrows
```rust
{
    let r = &mut data;
    // Use r
} // r's borrow ends here
// Now data is available again
```

### Step 5: Consider cloning if necessary
Sometimes you really do need a copy:
```rust
let s2 = s1.clone(); // Explicit, intentional copy
```

---

## 9. Real-World Example: Converting Java Code to Rust

### Java Code
```java
public class User {
    private String name;
    private List<String> hobbies;
    
    public User(String name) {
        this.name = name;
        this.hobbies = new ArrayList<>();
    }
    
    public void addHobby(String hobby) {
        this.hobbies.add(hobby);
    }
    
    public List<String> getHobbies() {
        return this.hobbies;
    }
}
```

### Rust Translation (with common mistakes)
```rust
// ❌ Wrong: owns data, but mutable methods are missing
struct User {
    name: String,
    hobbies: Vec<String>,
}

impl User {
    fn new(name: String) -> Self {
        User { name, hobbies: Vec::new() }
    }
    
    // ❌ Wrong: takes ownership of self, can't use after this call
    fn add_hobby(self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // ❌ Wrong: returns reference with no lifetime
    fn get_hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
}
```

### Correct Rust Version
```rust
struct User {
    name: String,
    hobbies: Vec<String>,
}

impl User {
    fn new(name: String) -> Self {
        User { name, hobbies: Vec::new() }
    }
    
    // ✅ Right: mutable borrow receiver
    fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }
    
    // ✅ Right: return slice (more flexible)
    fn get_hobbies(&self) -> &[String] {
        &self.hobbies
    }
}

// Usage
let mut user = User::new("Alice".to_string());
user.add_hobby("Rust".to_string());
user.add_hobby("JavaScript".to_string());

let hobbies = user.get_hobbies();
for hobby in hobbies {
    println!("{}", hobby);
}
```

### Key Differences from Java
| Java | Rust |
|------|------|
| `this` implicitly available | `&self`, `&mut self`, `self` |
| Mutable by default | Immutable by default (use `mut`) |
| References always valid | Borrows have strict rules |
| GC cleans up | Drop at scope end (deterministic) |

---

## 10. Debugging Tips Summary

### Checklist for Common Errors
1. **"Cannot assign twice"** → Add `mut` or use shadowing
2. **"Borrow of moved value"** → Use `&value` or `.clone()`
3. **"Cannot borrow as mutable more than once"** → Use scopes
4. **"Missing lifetime specifier"** → Add lifetime parameters
5. **"Mismatched types"** → Use `as` for casting
6. **"Cannot infer type"** → Add explicit type annotation

### Mental Model Checklist
- Who owns this value?
- Is it borrowed as `&` or `&mut`?
- Where is the last use of the borrow?
- Can I use scopes to limit borrow lifetime?

### Debugging Commands
```bash
# Compile without running (fast feedback)
cargo check

# Show warnings (more detailed than errors)
cargo clippy

# Expand macros to see generated code
cargo expand

# Run with backtrace for panic messages
RUST_BACKTRACE=1 cargo run
```

---

## 11. Moving from Java to Rust: The Mental Model Transition

### Phase 1: "This is too restrictive" (Week 1)
- Focus on learning the rules
- Use `cargo check` constantly
- Write small, isolated examples

### Phase 2: "I understand the rules" (Week 2-3)
- Start using borrow checker as a tool
- Learn to read compiler errors quickly
- Embrace immutability

### Phase 3: "I want to write Rust, not Java" (Week 4+)
- Think in terms of ownership
- Design with type safety in mind
- Use the borrow checker to enforce invariants

---

## 12. Final Advice

### Don't Fight the Borrow Checker
The borrow checker is your friend. It's catching bugs before they run. Instead of fighting it, learn its language.

### Read the Error Messages
Rust compiler errors are among the best in the industry. Read them carefully.

### Use the Community
- https://stackoverflow.com/questions/tagged/rust
- https://users.rust-lang.org/
- https://rust-lang.zulipchat.com/

### Practice, Practice, Practice
The more you write, the more intuitive ownership becomes. Start small, build up.

---

**Remember**: Every Rust developer has struggled with the borrow checker. It's not a sign of weakness—it's a sign you're learning something profound about memory safety and type systems.
