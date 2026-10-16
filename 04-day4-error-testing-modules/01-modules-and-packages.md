# Chapter 19: Packages, Crates, and Modules

As codebases grow, structuring code into organized namespaces, controlling public API visibility, and managing compilation units is paramount. Rust provides a clean module hierarchy system.

---

## 1. Terminology: Packages, Crates, and Modules

Rust organizes code into three distinct hierarchical layers:

```
+-----------------------------------------------------------------+
|  Package (Defined by `Cargo.toml`)                              |
|  Contains one or more crates (at most 1 library, any # of bins) |
|                                                                 |
|  +-----------------------------------------------------------+  |
|  |  Crate (Compilation Unit: generates a binary or library)  |  |
|  |  Root: `src/lib.rs` (library) or `src/main.rs` (binary)   |  |
|  |                                                           |  |
|  |  +-----------------------------------------------------+  |  |
|  |  |  Modules (Items organized in tree hierarchy)        |  |  |
|  |  |  `mod network { mod packet { ... } }`              |  |  |
|  |  +-----------------------------------------------------+  |  |
|  +-----------------------------------------------------------+  |
+-----------------------------------------------------------------+
```

---

## 2. The Module Filesystem Hierarchy

In Rust 2018 and 2021 editions, file paths map directly to modules without needing clumsy `mod.rs` files everywhere:

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs          # Crate root
    ├── network.rs       # Declares `mod network;`
    └── network/         # Submodules of `network`
        ├── client.rs    # `network::client`
        └── server.rs    # `network::server`
```

### In `src/main.rs`:
```rust
// Declares that the submodule `network` exists in `network.rs` or `network/`
mod network;

fn main() {
    network::server::start();
}
```

### In `src/network.rs`:
```rust
pub mod client;
pub mod server;
```

---

## 3. Visibility Modifiers: Encapsulation & Privacy

In Rust, **all items (functions, structs, traits, enums) are strictly private by default**.
Rust provides fine-grained visibility modifiers to expose items only where appropriate:

| Modifier | Accessibility / Scope |
| :--- | :--- |
| *(no keyword)* | Private to the current module and its immediate submodules. |
| `pub` | Public to the entire world (external crates and callers). |
| `pub(crate)` | Visible anywhere within the **current crate**, but private to external dependencies. |
| `pub(super)` | Visible to the **parent module**. |
| `pub(in path)` | Visible only within a specific ancestor module path. |

### Example: Visibility in Practice
```rust
mod outer {
    pub(crate) fn internal_helper() {
        println!("Visible anywhere in this crate!");
    }

    pub mod inner {
        pub(super) fn parent_only() {
            println!("Only outer and inner can call me!");
        }

        pub fn public_api() {
            parent_only(); // OK!
        }
    }
}
```

---

## 4. `use`, `super`, `self`, and Re-Exporting (`pub use`)

To avoid writing long paths like `crate::network::client::connect`, bring items into scope with `use`:

```rust
use crate::network::client::connect;

fn main() {
    connect("127.0.0.1:8080");
}
```

### 4.1 Relative Paths: `super` and `self`
- `self`: refers to the current module.
- `super`: refers to the parent module (similar to `..` in a filesystem).

```rust
mod tests {
    use super::*; // Import all public and private items from parent module
}
```

### 4.2 Re-Exporting with `pub use`
Re-exporting allows you to organize your internal files logically while presenting an ergonomic, flat public API to callers:

```rust
// Inside src/lib.rs:
mod internal_parser;
mod internal_ast;

// Re-export key types at root level:
pub use internal_parser::Parser;
pub use internal_ast::Node;
```

---

## 5. Exercise: Modules for a GUI Library

### Problem Description
Structure a modular GUI widget library with clean encapsulation:
- Create a `gui` module containing:
  - A submodule `widgets` with a `Button` struct and a `Label` struct.
  - A trait `Widget` with a method `draw(&self)`.
  - A `Window` struct that holds a collection of `Box<dyn Widget>`.
  - Ensure fields are private while constructors and methods are properly exposed via `pub`.

### Solution & Deep-Dive

```rust
pub mod gui {
    pub trait Widget {
        fn draw(&self);
    }

    pub mod widgets {
        use super::Widget;

        pub struct Button {
            label: String,
        }

        impl Button {
            pub fn new(label: impl Into<String>) -> Self {
                Button { label: label.into() }
            }
        }

        impl Widget for Button {
            fn draw(&self) {
                println!("[Button: {}]", self.label);
            }
        }

        pub struct Label {
            text: String,
        }

        impl Label {
            pub fn new(text: impl Into<String>) -> Self {
                Label { text: text.into() }
            }
        }

        impl Widget for Label {
            fn draw(&self) {
                println!("Label: {}", self.text);
            }
        }
    }

    pub struct Window {
        title: String,
        widgets: Vec<Box<dyn Widget>>,
    }

    impl Window {
        pub fn new(title: impl Into<String>) -> Self {
            Window {
                title: title.into(),
                widgets: Vec::new(),
            }
        }

        pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
            self.widgets.push(widget);
        }

        pub fn draw(&self) {
            println!("=== Window: {} ===", self.title);
            for widget in &self.widgets {
                widget.draw();
            }
        }
    }
}

fn main() {
    use gui::widgets::{Button, Label};
    use gui::Window;

    let mut win = Window::new("Settings Dialog");
    win.add_widget(Box::new(Label::new("Configure your preferences:")));
    win.add_widget(Box::new(Button::new("Save")));
    win.add_widget(Box::new(Button::new("Cancel")));

    win.draw();
}
```

---

## Summary & Next Step

You have mastered package layout, module trees, privacy boundaries, and re-exporting. Next, we build quality software using **Testing: Unit Tests, Integration Tests, and Clippy** in [02-testing.md](file:///Users/karthik/dev/rust_workspace/comprehensive-rust/04-day4-error-testing-modules/02-testing.md).
