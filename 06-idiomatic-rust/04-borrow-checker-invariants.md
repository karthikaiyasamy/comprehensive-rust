# Chapter 33: Borrow Checker Invariants: Split Borrows & PhantomData

The borrow checker operates with surgical precision, but understanding its exact structural boundaries allows you to design safe architectures without fighting the compiler.

---

## 1. Split Borrows

The borrow checker analyzes references at the **field level** for local structs:

```rust
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut p = Point3D { x: 1, y: 2, z: 3 };

    // Disjoint field borrows are completely VALID:
    let x_ref = &mut p.x;
    let y_ref = &mut p.y;
    let z_ref = &p.z;

    *x_ref += 10;
    *y_ref += 20;
    println!("Point: x={}, y={}, z={}", *x_ref, *y_ref, *z_ref);
}
```

### The Method Receiver Trap
However, if you define a method with `&mut self`, calling that method borrows the **entire struct**:

```rust
impl Point3D {
    fn update_x(&mut self) { self.x += 1; }
    fn read_y(&self) -> i32 { self.y }
}

fn bad_example(p: &mut Point3D) {
    // let y = p.read_y(); // Borrows entire `p` immutably
    // p.update_x();       // COMPILE ERROR: cannot borrow `*p` as mutable because it is also borrowed as immutable!
}
```

#### The Idiomatic Solution: Free Functions or Passing Fields
```rust
fn update_x_only(x: &mut i32) { *x += 1; }

fn good_example(p: &mut Point3D) {
    let y = p.y; // Read field
    update_x_only(&mut p.x); // Mutate only field x
}
```

---

## 2. Splitting Slices: `split_at_mut`

Can you borrow two elements of the same vector mutably at the same time?
- Direct indexing (`&mut v[0]` and `&mut v[1]`) fails because the index expression borrows the entire vector `v`.
- The standard library provides `split_at_mut`:

```rust
fn main() {
    let mut numbers = [10, 20, 30, 40, 50];

    let (left, right) = numbers.split_at_mut(2);
    // left: &mut [10, 20]
    // right: &mut [30, 40, 50]

    left[0] = 99;
    right[0] = 88;

    println!("Numbers: {numbers:?}"); // [99, 20, 88, 40, 50]
}
```

---

## 3. `PhantomData<T>`: Communicating Invariants to the Compiler

When creating a generic type that logically owns or borrows `T`, but does not store `T` directly in a struct field (e.g., storing a raw pointer `*const T`), the compiler will reject it because generic parameters must be used.

`std::marker::PhantomData<T>` acts as a zero-sized marker telling the compiler: *"Treat this struct as if it owns or references `T` for variance, drop check, and auto-trait purposes, but do not allocate any bytes in memory"*:

```rust
use std::marker::PhantomData;

struct CustomIter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>, // Informs compiler of lifetime 'a and reference type &T
}
```

---

## Summary & Next Step

You have learned split borrows, slice splitting, and zero-sized `PhantomData` markers. Next, we explore security and authorization patterns in **Token Types and Capability-Based Security** in [05-token-types-and-capabilities.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/06-idiomatic-rust/05-token-types-and-capabilities.md).
