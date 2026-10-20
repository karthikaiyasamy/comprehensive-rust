# Chapter 6: User-Defined Types: Structs, Enums, Const, and Static

Rust allows you to build expressive, type-safe data models using **named structs**, **tuple structs**, and **algebraic enums**. In this chapter, we explore how to construct these types, understand their memory layout, and leverage compiler optimizations like niche filling.

---

## 1. Named Structs

Named structs group related fields under explicit names:

```rust
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);
}
```

### 1.1 Field Init Shorthand
When a variable has the exact same name as a struct field, you can omit the colon and duplicate name:

```rust
fn create_person(name: String, age: u8) -> Person {
    Person { name, age } // Shorthand for Person { name: name, age: age }
}
```

### 1.2 Struct Update Syntax
You can construct a new instance from an existing one using `..base`:

```rust
let alice = Person {
    name: String::from("Alice"),
    ..peter // Copies or moves remaining fields from `peter`
};
```

---

## 2. Tuple Structs & The Newtype Pattern

Tuple structs have fields identified by index rather than name:

```rust
struct Point(i32, i32);

fn main() {
    let p = Point(10, 20);
    println!("x = {}, y = {}", p.0, p.1);
}
```

### The Newtype Idiom (Zero-Cost Wrappers)
A single-field tuple struct is known as a **newtype**. It creates a distinct type at compile time with **zero runtime overhead**:

```rust
struct Meters(f64);
struct Miles(f64);

fn calculate_speed(distance: Meters) { /* ... */ }

fn main() {
    let dist = Miles(5.0);
    // calculate_speed(dist); // COMPILE ERROR: mismatched types: expected `Meters`, found `Miles`
}
```
This prevents subtle unit-conversion bugs entirely at compile time.

---

## 3. Enums (Algebraic Data Types)

Rust enums are **sum types** (tagged unions). Unlike C enums which are merely integer constants, Rust enum variants can carry rich, heterogeneous payloads:

```rust
enum WebEvent {
    // 1. Unit variant (no payload)
    PageLoad,
    PageUnload,
    
    // 2. Tuple variant (unnamed fields)
    KeyPress(char),
    Paste(String),
    
    // 3. Struct variant (named fields)
    Click { x: i64, y: i64 },
}
```

### 3.1 Memory Layout of Enums
Under the hood, Rust stores an enum as a **discriminant** (tag) followed by space for the largest variant payload:

```
+-------------------+------------------------------------------+
|  Tag (1 byte)     |  Payload (sized to fit the largest       |
|  0: PageLoad      |           variant, with padding)         |
|  1: KeyPress      |                                          |
|  2: Click         |                                          |
+-------------------+------------------------------------------+
```

### 3.2 The Null Pointer Optimization (Niche Optimization)
How does Rust represent `Option<&T>` without wasting memory?
- In Rust, references `&T` can **never be 0 (null)**.
- The compiler recognizes address `0` as an unused bit pattern ("niche").
- It uses `0` to represent `None`, and any non-zero pointer to represent `Some(&T)`.

```rust
use std::mem::size_of;

fn main() {
    println!("size of &i32:         {}", size_of::<&i32>());         // 8 bytes
    println!("size of Option<&i32>:   {}", size_of::<Option<&i32>>()); // 8 bytes! (Zero overhead!)
}
```

---

## 4. Type Aliases

A type alias creates a new name for an existing type. They are completely interchangeable:

```rust
type Kilometers = f64;
type Thunk = Box<dyn Fn() -> i32 + Send + 'static>;
```

> [!NOTE]
> Type aliases do not provide type safety—`Kilometers` is still just an `f64`. If you want strict type safety, use a tuple struct newtype: `struct Kilometers(f64);`.

---

## 5. `const` vs. `static`

Rust distinguishes between compile-time inlined constants and globally allocated static memory:

```
+------------------------------------+------------------------------------+
|               `const`              |              `static`              |
+------------------------------------+------------------------------------+
| - Inlined at every call site       | - Fixed address in memory for the  |
| - Evaluated at compile time        |   entire lifetime of the program   |
| - No identity or unique address    | - Has a definite memory address    |
| - Prefer for numbers, masks, sizes | - Used for global state or FFI     |
+------------------------------------+------------------------------------+
```

### Example
```rust
const MAX_USERS: usize = 10_000;

// Const function evaluated at compile time
const fn compute_buffer_size() -> usize {
    MAX_USERS * 64
}

static SERVER_NAME: &str = "AuthenticationServer-v1";
```

---

## 6. Exercise: Elevator Events Modeling

### Problem Description
Design a type-safe data model for an elevator control system. You need to model:
1. Elevator directions (`Up`, `Down`).
2. Floors represented as `i32` (using a type alias `Floor`).
3. Buttons:
   - A lobby button on a specific floor with a requested direction (`LobbyCall(Direction, Floor)`).
   - An internal car floor button (`CarFloor(Floor)`).
4. Events that the elevator controller must handle:
   - `ButtonPressed(Button)`
   - `CarArrived(Floor)`
   - `CarDoorOpened`
   - `CarDoorClosed`
5. Construct helper functions for each event.

### Starter Code
```rust
#![allow(dead_code)]

#[derive(Debug)]
enum Event {
    // TODO: Define variants
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

// TODO: Define Button and helper functions
```

---

### Solution & Deep-Dive

```rust
#![allow(dead_code)]

/// Type alias for floors.
type Floor = i32;

/// A direction of travel.
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

/// A user-accessible button in the system.
#[derive(Debug, PartialEq, Eq)]
enum Button {
    /// Lobby button pressed from outside the elevator.
    LobbyCall(Direction, Floor),
    /// Floor button pressed inside the elevator car.
    CarFloor(Floor),
}

/// An event that the elevator controller must react to.
#[derive(Debug, PartialEq, Eq)]
enum Event {
    /// A button was pressed (either lobby or car).
    ButtonPressed(Button),
    /// The elevator car arrived at a specific floor.
    CarArrived(Floor),
    /// Car doors have finished opening.
    CarDoorOpened,
    /// Car doors have finished closing.
    CarDoorClosed,
}

/// Helper constructors
fn car_arrived(floor: Floor) -> Event {
    Event::CarArrived(floor)
}

fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

fn lobby_call_button_pressed(floor: Floor, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

fn car_floor_button_pressed(floor: Floor) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    let e1 = lobby_call_button_pressed(0, Direction::Up);
    let e2 = car_arrived(0);
    let e3 = car_door_opened();
    let e4 = car_floor_button_pressed(3);
    let e5 = car_door_closed();
    let e6 = car_arrived(3);

    println!("Events recorded:");
    println!("{e1:?}\n{e2:?}\n{e3:?}\n{e4:?}\n{e5:?}\n{e6:?}");
}
```

### Architectural Insights:
1. **Hierarchical Modeling:** Rather than having flat events like `LobbyCallUpButtonPressed(Floor)` and `LobbyCallDownButtonPressed(Floor)`, separating `Button` and `Direction` into structured nested enums makes the model clean and modular.
2. **`#[derive(Debug)]`:** This macro tells the compiler to automatically generate code for the `std::fmt::Debug` trait, enabling formatting with `{:?}`.

---

## Summary & Next Step

You have completed **Day 1: Rust Fundamentals**! You know how to build programs with types, control structures, references, slices, structs, and enums. 

Proceed to **Day 2: Types, Traits, and Generics** starting with [02-day2-types-and-traits/01-pattern-matching.md](file:///Users/karthik/dev/rust-projects/comprehensive-rust/comprehensive-rust/02-day2-types-and-traits/01-pattern-matching.md).
