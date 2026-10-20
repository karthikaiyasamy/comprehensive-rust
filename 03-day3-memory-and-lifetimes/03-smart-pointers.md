# Chapter 15: Smart Pointers: Box, Rc, and Interior Mutability

Smart pointers are data structures that act like pointers while providing additional metadata and capabilities such as heap allocation, reference counting, and runtime borrow checking.

---

## 1. `Box<T>`: Heap Allocation & Recursive Types

A `Box<T>` is the simplest smart pointer in Rust. It provides exclusive ownership of an allocation on the heap:

```rust
fn main() {
    let b: Box<i32> = Box::new(1024);
    println!("Value on heap: {b}");
} // `b` leaves scope; heap memory is automatically deallocated
```

### Solving Recursive Types with `Box`
At compile time, Rust must know the exact memory size of every type. Recursive types have unbounded size unless indirection is introduced:

```rust
// COMPILE ERROR: recursive type `List` has infinite size!
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// SOLUTION: Box provides a known, fixed pointer size (8 bytes on 64-bit):
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## 2. `Rc<T>`: Reference Counted Shared Ownership

In graph structures or shared data models, a value may require multiple simultaneous owners. `Rc<T>` (Reference Counted) provides **shared ownership** within a single thread:

```rust
use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(vec![1, 2, 3]);

    // Cloning an Rc increments the reference count; it does NOT copy the vector!
    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);

    println!("Strong count: {}", Rc::strong_count(&shared_data)); // 3
} // When count reaches 0, the inner vector is dropped
```

> [!WARNING]
> `Rc<T>` uses non-atomic reference count operations for maximum speed. It is strictly for **single-threaded** use and does not implement `Send` or `Sync`. For multi-threaded shared ownership, use `Arc<T>` (Atomic Reference Counting).

---

## 3. `RefCell<T>` & Interior Mutability

Normally, Rust forbids mutating a value through a shared reference (`&T`). 
The **Interior Mutability Pattern** allows mutating data even when only shared references exist, by deferring borrow-checker enforcement to **runtime**:

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(10);

    // Borrow mutably at runtime:
    {
        let mut mut_ref = data.borrow_mut();
        *mut_ref += 5;
    } // mut_ref dropped here

    // Borrow immutably at runtime:
    println!("Value: {}", *data.borrow()); // 15
}
```

### Runtime Borrow Checker Panics
`RefCell` dynamically tracks active borrows. If you violate the borrowing rules at runtime, `RefCell` **panics**:

```rust
let cell = RefCell::new(1);
let r1 = cell.borrow();
// let mut r2 = cell.borrow_mut(); // PANIC: already borrowed: BorrowMutError!
```

---

## 4. The Shared Mutable Pattern: `Rc<RefCell<T>>`

To build shared mutable data structures (like graphs, doubly-linked lists, or UI widget trees), pair `Rc` with `RefCell`:

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}
```

---

## 5. Exercise: Generic Binary Search Tree

### Problem Description
Implement a binary search tree using `Box`:
- A struct `BinaryTree<T>` containing a root `Option<Box<Node<T>>>`.
- A method `.insert(value: T)` that inserts elements preserving BST invariants (left $\le$ parent, right $>$ parent).
- A method `.contains(&value: &T) -> bool`.

### Solution & Deep-Dive

```rust
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(node_slot: &mut Option<Box<Node<T>>>, value: T) {
        match node_slot {
            None => {
                *node_slot = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
            Some(node) => {
                if value <= node.value {
                    Self::insert_node(&mut node.left, value);
                } else {
                    Self::insert_node(&mut node.right, value);
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == &node.value {
                return true;
            } else if value < &node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(50);
    tree.insert(25);
    tree.insert(75);
    tree.insert(10);
    tree.insert(30);

    assert!(tree.contains(&25));
    assert!(tree.contains(&30));
    assert!(!tree.contains(&99));
    println!("BinaryTree verified successfully!");
}
```

---

## Summary & Next Step

You have mastered `Box`, `Rc`, `RefCell`, and recursive data structures. Next, we explore the core of Rust's borrow checker: **Borrowing, Non-Lexical Lifetimes, and Reborrowing** in [04-borrowing-and-references.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/03-day3-memory-and-lifetimes/04-borrowing-and-references.md).
