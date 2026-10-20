# Chapter 16: The Borrow Checker: Aliasing, NLL, and Invariants

The Borrow Checker is the core subsystem of the Rust compiler that statically analyzes reference lifetimes and ensures memory safety without a runtime garbage collector.

---

## 1. The Core Borrowing Invariants

Every reference in Rust must satisfy two fundamental invariants:

1. **Validity:** A reference must **never outlive the data it references**.
2. **Aliasing XOR Mutability:** At any given point in program execution, a piece of data may have:
   - **One or more shared references (`&T`)**, OR
   - **Exactly one exclusive reference (`&mut T`)**,
   - **BUT NEVER BOTH.**

```
                      Borrow Checker Rules
                                |
               +----------------+----------------+
               |                                 |
       Shared References                 Exclusive Reference
           (`&T`)                             (`&mut T`)
               |                                 |
    Unlimited readers allowed           Sole writer permitted
    Data is frozen (read-only)          No other readers or writers
```

---

## 2. Freezing & Invalidation

When data is borrowed, the owner's capabilities are temporarily restricted ("frozen"):

```rust
fn main() {
    let mut vec = vec![1, 2, 3];

    // Shared borrow: `vec` is now FROZEN
    let first = &vec[0];

    // vec.push(4); // COMPILE ERROR: cannot borrow `vec` as mutable because it is also borrowed as immutable!

    println!("First element: {first}"); // `first` used here
    
    // Now that `first` is no longer used, `vec` is un-frozen!
    vec.push(4); // OK!
}
```

### Why Iterator Invalidation is Impossible in Rust
In C++ or Java, adding an element to a vector while iterating over it causes undefined behavior or `ConcurrentModificationException` because the vector may reallocate its internal buffer, leaving the iterator pointing to freed memory.
In Rust, `for item in &vec` creates a shared borrow on `vec`. Calling `vec.push()` requires an exclusive borrow, which the compiler rejects at compile time!

---

## 3. Non-Lexical Lifetimes (NLL)

In early versions of Rust, borrows lasted until the enclosing curly brace `}` of the scope. 
Since Rust 2018, the compiler uses **Non-Lexical Lifetimes (NLL)**:
- A borrow's lifetime extends **only to the statement where it is last used**, rather than to the end of the block.

```rust
fn main() {
    let mut data = vec![10, 20, 30];

    let r = &data[0];
    println!("Value: {r}"); // Last use of `r`!

    // Under NLL, `r`'s borrow ends right here!
    data.push(40); // Compiles cleanly under NLL!
}
```

---

## 4. Reborrowing

When you pass a mutable reference `&mut T` to a function, Rust does not move the reference; it performs a **reborrow**:

```rust
fn increment(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut val = 10;
    let r = &mut val;

    increment(r); // Reborrows `r` as `&mut *r`
    increment(r); // `r` is still valid here!

    println!("val: {val}"); // 12
}
```

---

## 5. Diagnostic Guide to Common Borrow Errors

| Compiler Error Message | Underlying Cause | Solution |
| :--- | :--- | :--- |
| `cannot borrow as mutable more than once at a time` | Attempting to create two `&mut` references simultaneously. | Scope the first mutable borrow, or refactor to perform updates sequentially. |
| `cannot borrow as mutable because it is also borrowed as immutable` | Reading through `&T` while trying to write through `&mut T`. | Finish using the shared reference before mutating, or clone the data if small. |
| `cannot move out of borrowed content` | Attempting to take ownership of a field from behind a reference `&T`. | Use `.clone()`, borrow a reference instead, or replace using `std::mem::replace` / `Option::take()`. |

---

## 6. Exercise: The Wizard's Inventory

### Problem Description
Manage an inventory system for an RPG game:
- An `Item` struct with `name: String` and `durability: u32`.
- A `Wizard` struct with an inventory of items: `items: Vec<Item>`.
- Methods:
  - `.add_item(item: Item)`
  - `.get_item(index: usize) -> Option<&Item>` (read-only borrow)
  - `.use_item(index: usize)` (mutable borrow: decreases durability by 10; removes item if durability reaches 0).

### Solution & Deep-Dive

```rust
#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    durability: u32,
}

struct Wizard {
    name: String,
    inventory: Vec<Item>,
}

impl Wizard {
    fn new(name: impl Into<String>) -> Self {
        Wizard {
            name: name.into(),
            inventory: Vec::new(),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    fn get_item(&self, index: usize) -> Option<&Item> {
        self.inventory.get(index)
    }

    fn use_item(&mut self, index: usize) -> bool {
        if let Some(item) = self.inventory.get_mut(index) {
            if item.durability > 10 {
                item.durability -= 10;
                true
            } else {
                // Item breaks! Remove from inventory
                self.inventory.remove(index);
                false
            }
        } else {
            false
        }
    }
}

fn main() {
    let mut gandalf = Wizard::new("Gandalf");
    gandalf.add_item(Item { name: String::from("Staff"), durability: 25 });

    // Inspect item
    assert_eq!(gandalf.get_item(0).unwrap().durability, 25);

    // Use item
    assert!(gandalf.use_item(0));
    assert_eq!(gandalf.get_item(0).unwrap().durability, 15);

    // Use again: durability becomes 5 <= 10, so item breaks and is removed
    assert!(!gandalf.use_item(0));
    assert_eq!(gandalf.get_item(0), None);
    println!("Wizard's inventory verified successfully!");
}
```

---

## Summary & Next Step

You have mastered the borrow checker's aliasing rules, non-lexical lifetimes (NLL), and reborrowing. Next, we explore named lifetimes in **Lifetimes: Annotations, Structs, and Elision Rules** in [05-lifetimes.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/03-day3-memory-and-lifetimes/05-lifetimes.md).
