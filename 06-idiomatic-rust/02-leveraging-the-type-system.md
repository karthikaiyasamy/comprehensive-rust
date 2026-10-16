# Chapter 31: Leveraging the Type System: Newtypes, RAII, and Extension Traits

Idiomatic Rust shifts runtime bug detection to **compile-time guarantees** by exploiting the rich type system.

---

## 1. The Newtype Pattern

A newtype is a tuple struct wrapping an existing type:

```rust
pub struct UserId(u64);
pub struct OrderId(u64);

fn process_order(user: UserId, order: OrderId) {
    // Cannot accidentally swap arguments!
}

fn main() {
    let u = UserId(42);
    let o = OrderId(99);
    process_order(u, o);
    // process_order(o, u); // COMPILE ERROR: type mismatch!
}
```

### Key Advantages:
1. **Zero Runtime Overhead:** In machine code, `UserId` is identical to `u64`. LLVM strips the wrapper completely.
2. **Encapsulation:** The inner data can remain private, allowing the newtype to enforce invariant validation upon creation.
3. **Bypassing the Orphan Rule:** You cannot implement an external trait for an external type (e.g., `Display` for `Vec<T>`). By wrapping `Vec<T>` in `struct MyVec<T>(Vec<T>)`, the struct is local to your crate, allowing you to implement any trait!

---

## 2. RAII: Custom Resource Guards & Commit Pattern

**Resource Acquisition Is Initialization (RAII)** ties resource lifecycle to variable scope. The `Drop` trait ensures cleanup is deterministic, even in the event of panics.

### The Scope Guard / Rollback Pattern:
```rust
struct DatabaseTransaction<'a> {
    db: &'a mut Database,
    committed: bool,
}

impl<'a> DatabaseTransaction<'a> {
    pub fn new(db: &'a mut Database) -> Self {
        db.execute("BEGIN TRANSACTION;");
        DatabaseTransaction { db, committed: false }
    }

    pub fn commit(mut self) {
        self.db.execute("COMMIT;");
        self.committed = true;
    }
}

impl<'a> Drop for DatabaseTransaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            eprintln!("Transaction not committed! Rolling back changes.");
            self.db.execute("ROLLBACK;");
        }
    }
}
```

If the transaction is never committed (due to an error, early return `?`, or panic), the destructor automatically rolls back the database!

---

## 3. Extension Traits

Rust does not allow monkey-patching, but you can add methods to external standard types using **Extension Traits**:

```rust
pub trait StringUtilsExt {
    fn truncate_with_ellipsis(&self, max_chars: usize) -> String;
}

impl StringUtilsExt for str {
    fn truncate_with_ellipsis(&self, max_chars: usize) -> String {
        if self.chars().count() <= max_chars {
            self.to_string()
        } else {
            let truncated: String = self.chars().take(max_chars).collect();
            format!("{truncated}...")
        }
    }
}

fn main() {
    let text = "Comprehensive Rust Course";
    println!("{}", text.truncate_with_ellipsis(13)); // "Comprehensive..."
}
```

Callers simply import the trait (`use crate::StringUtilsExt;`) to activate the helper methods.

---

## Summary & Next Step

You have learned newtypes, RAII transaction guards, and extension traits. Next, we master one of Rust's most powerful architectural patterns in **The Typestate Pattern: Enforcing Invariants at Compile Time** in [03-typestate-pattern.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/03-typestate-pattern.md).
