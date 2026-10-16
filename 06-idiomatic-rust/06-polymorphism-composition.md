# Chapter 35: Polymorphism: Composition Over Inheritance

A common stumbling block for engineers transitioning from C++, Java, or Python is the absence of classical class inheritance (`class Dog extends Animal`). Rust firmly embraces the Gang of Four principle: **"Favor object composition over class inheritance"**.

---

## 1. The Pitfalls of Class Inheritance

In object-oriented inheritance hierarchies:
- **Fragile Base Class Problem:** Modifying a base class method can silently break assumptions in derived subclasses.
- **Deep Hierarchies:** Real-world entities rarely fit into rigid, single-inheritance taxonomy trees.
- **Tight Coupling:** Subclasses inherit both data fields and behavior, making unit testing and refactoring difficult.

---

## 2. The Rust Paradigm: Struct Composition + Traits

Instead of inheriting fields, embed structs inside one another and share behavior through traits:

```rust
// Shared behavior
trait Flyable {
    fn fly(&self);
}

trait Quackable {
    fn quack(&self);
}

// Concrete component structs
struct Wings;
impl Wings {
    fn flap(&self) { println!("Flapping wings!"); }
}

struct Bill;
impl Bill {
    fn vocalize(&self) { println!("Quack!"); }
}

// Composition: Duck contains Wings and Bill
struct Duck {
    wings: Wings,
    bill: Bill,
}

impl Flyable for Duck {
    fn fly(&self) {
        self.wings.flap();
    }
}

impl Quackable for Duck {
    fn quack(&self) {
        self.bill.vocalize();
    }
}
```

---

## 3. The Strategy Pattern in Rust

You can implement the **Strategy Pattern** using generic trait bounds (static dispatch) or trait objects (dynamic dispatch):

```rust
trait CompressionStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
}

struct Gzip;
impl CompressionStrategy for Gzip {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing with Gzip...");
        data.to_vec()
    }
}

struct Snappy;
impl CompressionStrategy for Snappy {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing with Snappy (low latency)...");
        data.to_vec()
    }
}

// Context struct using generic strategy (zero runtime overhead!)
struct Archive<S: CompressionStrategy> {
    strategy: S,
}

impl<S: CompressionStrategy> Archive<S> {
    fn new(strategy: S) -> Self {
        Archive { strategy }
    }

    fn package(&self, data: &[u8]) -> Vec<u8> {
        self.strategy.compress(data)
    }
}
```

---

## 4. Static vs. Dynamic Dispatch Trade-off Matrix

| Dimension | Static Dispatch (`impl Trait`, Generics) | Dynamic Dispatch (`dyn Trait`) |
| :--- | :--- | :--- |
| **Dispatch Mechanism** | Direct function call or inline assembly | Indirect call via virtual table pointer (VTable) |
| **Inlining & Optimization**| Fully eligible for LLVM vectorization and inlining | Inlining across vtable boundary is prevented |
| **Heterogeneous Collections**| Cannot store different types in the same array | Can store different types (`Vec<Box<dyn Trait>>`) |
| **Binary Size** | Potential code bloat if monomorphized across many types | Single implementation in binary |
| **Execution Speed** | **Fastest** (sub-nanosecond direct jump) | Slightly slower (~few CPU cycles for pointer indirection) |

---

## Summary & Next Step

You have completed **Module 6: Idiomatic Rust and Modern Systems Architecture**! 

Now we enter the deepest, most rigorous realm of systems engineering: **Module 7: The Unsafe Rust Deep Dive** starting with [07-unsafe-deep-dive/01-understanding-unsafe.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/07-unsafe-deep-dive/01-understanding-unsafe.md).
