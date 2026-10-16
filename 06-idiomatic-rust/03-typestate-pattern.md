# Chapter 32: The Typestate Pattern: Compile-Time State Machines

In traditional object-oriented systems, state machines are managed via runtime state fields (e.g., `state: StateEnum`), requiring runtime assertion checks (`if self.state != State::Connected { return Err(...); }`). If an engineer forgets a check, the program crashes at runtime.

The **Typestate Pattern** encodes state directly into the Rust type system, ensuring that **invalid operations are rejected by the compiler**.

---

## 1. The Typestate Architecture

Instead of a runtime enum, each state is modeled as an empty **Zero-Sized Type (ZST)**:

```rust
// Marker states (ZSTs: occupy 0 bytes in memory)
struct Draft;
struct UnderReview;
struct Published;

// Generic struct parameterized by State
struct Post<State> {
    content: String,
    state: State,
}
```

---

## 2. Implementing Methods for Specific States

Methods are defined only on the state where they are valid:

```rust
// 1. Initial State: Only new() and add_text() exist
impl Post<Draft> {
    pub fn new() -> Self {
        Post {
            content: String::new(),
            state: Draft,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Consumes `Post<Draft>` and transitions to `Post<UnderReview>`
    pub fn request_review(self) -> Post<UnderReview> {
        Post {
            content: self.content,
            state: UnderReview,
        }
    }
}

// 2. Review State: Only approve() exists
impl Post<UnderReview> {
    // Consumes `Post<UnderReview>` and transitions to `Post<Published>`
    pub fn approve(self) -> Post<Published> {
        Post {
            content: self.content,
            state: Published,
        }
    }
}

// 3. Published State: Only content() inspection exists
impl Post<Published> {
    pub fn content(&self) -> &str {
        &self.content
    }
}
```

---

## 3. The Compiler Catches Order-of-Operation Bugs!

Let's see what happens when a developer tries to use the API incorrectly:

```rust
fn main() {
    let mut post = Post::new();
    post.add_text("Mastering Comprehensive Rust");

    // Attempting to inspect content while still a Draft:
    // println!("{}", post.content()); 
    // COMPILE ERROR: no method named `content` found for struct `Post<Draft>`!

    let review = post.request_review();

    // Attempting to add text while UnderReview:
    // review.add_text("Sneaky edit"); 
    // COMPILE ERROR: no method named `add_text` found for struct `Post<UnderReview>`!

    let published = review.approve();
    println!("Published: {}", published.content()); // Valid!
}
```

### Why Moving `self` Prevents Stale State:
Because state transition methods consume `self` by value, the previous state variable is **moved and invalidated**. You cannot transition `review.approve()` and then reuse `review`!

---

## Summary & Next Step

You have learned how to eliminate runtime state checks and build compile-time verified workflows using the Typestate pattern. Next, we explore advanced reference mechanics in **Borrow Checker Invariants, Split Borrows, and PhantomData** in [04-borrow-checker-invariants.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/06-idiomatic-rust/04-borrow-checker-invariants.md).
