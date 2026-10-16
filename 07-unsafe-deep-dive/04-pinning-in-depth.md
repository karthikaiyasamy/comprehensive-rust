# Chapter 39: Pinning in Depth: Self-Referential Types & Drop

Pinning is one of Rust's most sophisticated memory safety abstractions. It is the core mathematical contract that makes asynchronous `Future` state machines and self-referential data structures sound.

---

## 1. What is a "Move" in Rust?

In Rust, a move is literally a **bitwise memory copy (`memcpy`)**:
- When you pass a struct into a function by value, or return it, or push it into a vector, its bytes are copied to a new memory address.
- In safe Rust, types are assumed to be freely relocatable in memory.

### Why Moves Break Self-Referential Structs
Consider a struct that stores a buffer and an internal pointer pointing directly into that buffer:

```
Before Move (Address 0x1000):
+------------------------------------+
| buffer: "Hello World"              | <---+
| ptr: 0x1000 -----------------------+-----+ (Points to its own buffer!)
+------------------------------------+

After Move to new Stack Frame (Address 0x2000):
+------------------------------------+
| buffer: "Hello World"              |
| ptr: 0x1000 -----------------------+-----> [Old Memory at 0x1000: GARBAGE/DANGLING!]
+------------------------------------+
```
The internal pointer now points to the old, invalidated stack frame. Dereferencing `ptr` causes memory corruption!

---

## 2. The `Pin<P>` Type & The `Unpin` Trait

Rust solves this with `std::pin::Pin<P>`:

```rust
pub struct Pin<P> {
    pointer: P,
}
```

### The Pin Invariant Contract:
> **If a type `T` is wrapped in `Pin<P>`, the data `T` is guaranteed NEVER to be moved in memory from its current location until it is dropped.**

### 2.1 The `Unpin` Auto Trait
Almost all standard Rust types (`i32`, `String`, `Vec<T>`) do not contain self-referential pointers and don't care if they are moved. 
- The compiler automatically implements `Unpin` for these types.
- If `T: Unpin`, `Pin<&mut T>` can be freely converted back to `&mut T` without unsafe code.

### 2.2 Opting Out of `Unpin`: `PhantomPinned`
To create a struct that cannot be moved once pinned, embed `PhantomPinned`:

```rust
use std::marker::PhantomPinned;

struct SelfReferential {
    data: String,
    self_pointer: *const String,
    _marker: PhantomPinned, // Opts out of the Unpin trait!
}
```

---

## 3. Building a Sound Self-Referential Struct

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

pub struct SelfRefBuffer {
    data: String,
    slice_ptr: *const str,
    _pin: PhantomPinned,
}

impl SelfRefBuffer {
    pub fn new(text: String) -> Pin<Box<Self>> {
        let instance = SelfRefBuffer {
            data: text,
            slice_ptr: ptr::null(),
            _pin: PhantomPinned,
        };

        // 1. Allocate on heap and pin in place
        let mut boxed = Box::pin(instance);

        // 2. Initialize internal pointer to point to heap-pinned `data`:
        let self_ptr: *const SelfRefBuffer = &*boxed;
        let data_ref: &str = unsafe { &(*self_ptr).data };

        unsafe {
            let mut_ref = Pin::get_unchecked_mut(boxed.as_mut());
            mut_ref.slice_ptr = data_ref as *const str;
        }

        boxed
    }

    pub fn read_data(self: Pin<&Self>) -> &str {
        unsafe { &*self.slice_ptr }
    }
}

fn main() {
    let pinned = SelfRefBuffer::new(String::from("Safe self-referential data!"));
    println!("Read from pinned self-reference: {}", pinned.as_ref().read_data());
}
```

---

## Summary & Next Step

You have learned why moves break self-referential pointers, how `Pin` guarantees immovable memory, and how `PhantomPinned` enforces this at compile time. Next, we master native systems interoperability in **FFI and C Language Interoperability** in [05-ffi-and-language-interop.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/05-ffi-and-language-interop.md).
