# Chapter 18: Visual Ownership & Borrowing Guide for Java Developers

This chapter provides **visual memory diagrams** and **step-by-step explanations** for Rust's ownership and borrowing system. If you're a Java developer, these diagrams will help bridge the gap between Java's GC-based mental model and Rust's compile-time ownership system.

---

## 1. Memory Models: Java vs. Rust

### Java Memory Model (Garbage Collection)
```
┌─────────────────────────────────────────────────────────────┐
│                    Java Memory Model                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stack:                                Heap:                 │
│  +-----------+                         +-----------------+  │
│  | ref1      |   ────┐                 | [String "hello"]│  │
│  | (ptr)     │       │                 +-----------------+  │
│  +-----------+       │                           │          │
│                      │                           ▼          │
│  | ref2      |   ────┼────→  [String "hello"]                │
│  | (ptr)     │       │                 +-----------------+  │
│  +-----------+       │                 | [String "world"]│  │
│                      │                 +-----------------+  │
│  | ref3      |   ────┘                           │          │
│  | (ptr)     │                                   ▼          │
│  +-----------+                            [String "world"]  │
│        │                                        │          │
│        ▼                                        │          │
│   [Garbage Collector]                          ▼            │
│   - Pauses execution                         [FREE]         │
│   - Traces references                       (freed)         │
│   - Frees unreachable                       (freed)         │
│     objects                                                  │
└─────────────────────────────────────────────────────────────┘
```

**Characteristics:**
- Heap allocations everywhere
- GC decides when to free
- Multiple references to same object
- No compile-time aliasing checks

### Rust Memory Model (Ownership System)
```
┌─────────────────────────────────────────────────────────────┐
│                   Rust Memory Model                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Stack:                                    Heap:             │
│  +-----------+                             +-------------+  │
│  | s1        |                             | "hello"     │  │
│  | [ptr]     |   ────┐                     +-------------+  │
│  | [len:5]   |       │                                     │
│  +-----------+       │                                     │
│                      │                                     │
│  | s2        |   ────┼────→ [MOVED - s1 invalidated]      │
│  | [ptr]     |       │                                     │
│  | [len:5]   |       │                                     │
│  +-----------+       │                                     │
│                      │                                     │
│  | ref1      |   ────┘                                     │
│  | &s2       |   (borrow, no allocation)                   │
│  +-----------+                                             │
│                                                             │
│  Drop order (LIFO): s2, then ref1                           │
│  - No GC pauses                                             │
│  - Drop called at exact scope end                           │
└─────────────────────────────────────────────────────────────┘
```

**Characteristics:**
- Stack allocation first
- Compile-time drop order
- Single owner by default
- Borrow checker enforces rules

---

## 2. Ownership Moves: Step-by-Step Visual

### Java Behavior (Reference Sharing)
```java
// Java code:
String s1 = new String("hello");
String s2 = s1;  // Both point to same object!
s1 = null;       // s2 still works
```

**Memory diagram:**
```
Before:                         After:
+-------+                       +-------+
| s1    |                       | null  |
| ptr───┼─┐                     +-------+
+-------+ │                     +-------+
          │                     | s2    |
+-------+ │                     | ptr───┼─┐
| "hello" │                     +-------+ │
+-------+ │                           │   │
          │                           ▼   │
+-------+ │                     +-------+
| s2    | │                     | "hello" │
| ptr───┼─┘                     +-------+
+-------+
```

**Key point:** Both `s1` and `s2` can access the same object. GC cleans up later.

### Rust Behavior (Move)
```rust
// Rust code:
let s1 = String::from("hello");
let s2 = s1;  // MOVE - s1 is invalidated!
// println!("{}", s1); // ERROR: borrow of moved value
```

**Memory diagram:**
```
Before:                         After:
Stack:                          Stack:
+-------+                       +-------+
| s1    |                       | s1    │ (INVALIDATED)
| [ptr] │                       +-------+
| [len] │                         ^
+-------+                         │
                                  │
+-------+         +-------------+ │
| s2    |         | "hello"     │ │
| [ptr] ├─────────┤ [len:5]     │ │
| [len:5]│        +-------------+ │
+-------+                         │
                                  │
+-------------+                   │
| "hello"     │◄──────────────────┘
+-------------+
```

**Key point:** Ownership moves to `s2`. `s1` is now invalid.

### Rust Behavior (Clone)
```rust
// Rust code:
let s1 = String::from("hello");
let s2 = s1.clone();  // DEEP COPY
```

**Memory diagram:**
```
Stack:                          Stack:
+-------+                       +-------+
| s1    |                       | s1    |
| [ptr] │                       | [ptr] │
| [len] │                       | [len] │
+-------+                       +-------+
      │                               │
      ▼                               ▼
+-------------+               +-------------+
| "hello"     │               | "hello"     │
+-------------+               +-------------+
                                    ^
                                    │
+-------+                           │
| s2    |                           │
| [ptr] ├───────────────────────────┘
| [len] │
+-------+
```

**Key point:** Two separate heap allocations. Both valid.

---

## 3. Borrowing: References in Action

### Shared Borrow (&T)
```rust
let s1 = String::from("hello");
let r1 = &s1;  // Shared borrow
let r2 = &s1;  // Another shared borrow (OK!)
```

**Memory diagram:**
```
Stack:                          Heap:
+-------+                       +-------------+
| s1    |                       | "hello"     │
| [ptr] │                       +-------------+
| [len] │
+-------+
      │
      ├───────┐
      │       │
      ▼       ▼
+-------+ +-------+
| r1    | | r2    |
| &s1   | | &s1   │
+-------+ +-------+
```

**Rules:**
- Multiple shared borrows allowed
- Data is "frozen" (cannot modify)
- Borrow lasts until last use

### Mutable Borrow (&mut T)
```rust
let mut s1 = String::from("hello");
let r1 = &mut s1;  // Exclusive borrow
// let r2 = &mut s1; // ERROR: cannot borrow as mutable more than once
```

**Memory diagram:**
```
Stack:                          Heap:
+-------+                       +-------------+
| s1    |                       | "hello"     │
| [ptr] │                       +-------------+
| [len] │
+-------+
      │
      ▼
+-------+
| r1    |
| &mut  │
| s1    │
+-------+
```

**Rules:**
- Only one mutable borrow allowed
- No shared borrows allowed
- Ensures no data races

---

## 4. Borrow Checker Scenarios: Visual Analysis

### Scenario 1: The "Iterator Invalidation" Problem

#### Java (Runtime Error)
```java
List<String> list = new ArrayList<>();
list.add("a");
list.add("b");

for (String item : list) {
    if (item.equals("a")) {
        list.add("c");  // Runtime: ConcurrentModificationException!
    }
}
```

**Memory state during iteration:**
```
list: [ref1] ──→ [a] ──→ [b] ──→ null
         │
         ▼ (iterator points here)
         
After adding "c":
list: [ref1] ──→ [a] ──→ [b] ──→ null
              [c] (memory corruption risk!)
```

#### Rust (Compile Error)
```rust
let mut vec = vec![String::from("a"), String::from("b")];

for item in &vec {
    if item == "a" {
        // vec.push(String::from("c")); // ERROR!
        // Cannot borrow `vec` as mutable because it's also borrowed as immutable
    }
}
```

**Why?**
```
for item in &vec creates:       Attempting vec.push requires:
+-------+                       +-------+
| item  |                       | vec   │
| &str  |                       | &mut  │
+-------+                       +-------+
      │                               │
      └───────┬───────────────────────┘
              │
      Cannot have both & and &mut!
```

### Scenario 2: The "Use After Free" Problem

#### C (Undefined Behavior)
```c
char* ptr = malloc(10);
free(ptr);
printf("%s", ptr);  // Undefined behavior!
```

#### Rust (Compile Error)
```rust
let s1 = String::from("hello");
let s2 = s1;  // Move
// println!("{}", s1); // ERROR: borrow of moved value
```

**Memory diagram:**
```
After move:
Stack:                          Heap:
+-------+                       +-------------+
| s1    |                       | "hello"     │
| (INVALID)│                    +-------------+
+-------+                               │
                                        │
+-------+                               │
| s2    |◄──────────────────────────────┘
| [ptr] │
+-------+
```

**Key point:** Rust prevents this at compile time.

---

## 5. Lifetimes Visualized

### Basic Lifetime Annotation
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**Visual explanation:**
```
Lifetime 'a represents: "The shorter of the two input lifetimes"

┌─────────────── 'a ────────────────┐
│                                   │
│  +---------+     +---------+      │
│  |   x     │     |   y     │      │
│  | &str    │     | &str    │      │
│  +---------+     +---------+      │
│        │               │          │
│        └──────┬────────┘          │
│               ▼                   │
│        +────────────+             │
│        │  longest   │             │
│        │  returns   │             │
│        │  &'a str   │             │
│        +────────────+             │
└───────────────────────────────────┘
```

### Struct with Lifetimes
```rust
struct StrRef<'a> {
    reference: &'a str,
}
```

**Memory diagram:**
```
+---------------------+           +-------------+
| str_ref             │           | "hello"     │
| [reference: &str]   │           +-------------+
|   'a: ──────────────┼───────────┤             │
+---------------------+           +-------------+
```

**Rule:** The struct cannot outlive the data it references.

---

## 6. Common Java-to-Rust Ownership Misconceptions

### Misconception 1: "Rust is like Java with manual memory management"
**Reality:** Rust's ownership system is fundamentally different. It's not about "freeing memory"—it's about **compile-time guarantee of memory safety**.

### Misconception 2: "I need to think about pointers in Rust"
**Reality:** You rarely work with raw pointers. The borrow checker works with **references** (`&` and `&mut`), which are high-level abstractions.

### Misconception 3: "Ownership is complicated"
**Reality:** Ownership becomes intuitive once you think in terms of:
- **Single owner** (default)
- **Borrowing** (temporary access)
- **Moving** (transferring ownership)

---

## 7. Memory Diagram Practice Exercises

### Exercise 1: Trace This Code
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**Draw the memory diagram at each step.**

### Exercise 2: Move vs. Clone
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s3 = s1; // What happens here?
}
```

**Why does this compile? What's the memory state?**

### Exercise 3: Borrow Checker Puzzle
```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    // Where does s become available again?
}
```

**At what line does `s` become mutable again? Why?**

---

## 8. Debugging Borrow Errors: Visual Checklist

When you see a borrow error, ask:

### 1. Who owns the data?
```
Who has the last copy of the value?
```

### 2. What's the current borrow status?
```
Is it borrowed as & or &mut?
```

### 3. Where is the borrow still in use?
```
Look for the last use of the reference
```

### 4. What do you want to do?
```
Try to mutate? → Need &mut, but is there a &?
Read? → Can have multiple &s
```

---

## 9. Summary: Mental Model Mapping

| Concept | Java Mental Model | Rust Mental Model |
|---------|-------------------|-------------------|
| **Variable** | Reference to heap object | Binding with ownership |
| **Assignment** | Copy reference | Move ownership (or clone) |
| **Function Arg** | Pass reference | Pass by value, move, or borrow |
| **Memory Deallocation** | GC (unknown time) | Drop at scope end (deterministic) |
| **Aliasing** | Allowed freely | Checked by compiler (`&` vs `&mut`) |
| **Null** | `null` exists | `Option<T>` (exhaustive checking) |

---

## 10. Visual Summary: Ownership Rules

```
┌─────────────────────────────────────────────────────────────┐
│           Rust Ownership: The Three Rules                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. Each value has a single owner                            │
│     (one variable binding)                                  │
│                                                             │
│  2. When owner goes out of scope, value is dropped           │
│     (deterministic cleanup)                                 │
│                                                             │
│  3. Move, don't copy (unless Copy trait)                     │
│     (ownership transfers on assignment)                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│           Borrowing: The Two Rules                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. At any time:                                             │
│     - One mutable reference (&mut T) OR                     │
│     - Any number of shared references (&T)                  │
│     (but not both)                                          │
│                                                             │
│  2. References must always be valid                          │
│     (no dangling pointers)                                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 11. Quick Reference: Common Patterns

### Pattern 1: "I want to pass this to a function but keep using it"
```rust
// ❌ Wrong (moves)
let s = String::from("hello");
takes_ownership(s);
println!("{}", s); // ERROR

// ✅ Right (borrows)
let s = String::from("hello");
takes_borrow(&s);
println!("{}", s); // OK
```

### Pattern 2: "I need a function to modify a value"
```rust
// ❌ Wrong (immutable)
fn append_world(s: String) { s.push_str(", world!"); } // ERROR

// ✅ Right (mutable borrow)
fn append_world(s: &mut String) { s.push_str(", world!"); }
```

### Pattern 3: "I want a default value that can be overridden"
```rust
// Java style (mutable builder)
let config = new Config();
config.setHost("localhost");
config.setPort(8080);

// Rust style (immutable, returns new)
let config = Config::new();
let config = config.with_host("localhost");
let config = config.with_port(8080);
```

---

## Appendix: Tools for Understanding

### 1. Use `cargo expand` to see monomorphized code
```bash
cargo install cargo-expand
cargo expand > expanded.rs
```

### 2. Use `rust-analyzer` in VS Code
- Hover over references to see lifetimes
- Jump to type definitions
- See borrow checker annotations

### 3. Draw memory diagrams
- Paper or whiteboard
- Show stack, heap, references
- Track moves and borrows

### 4. Read compiler errors carefully
- "borrowed value does not live long enough"
- "cannot borrow as mutable because it is also borrowed as immutable"
- These messages tell you exactly what's wrong

---

**Next**: Move on to the actual Rust documentation and read the "Ownership" and "References and Borrowing" chapters. Use these diagrams as a reference while you learn!
