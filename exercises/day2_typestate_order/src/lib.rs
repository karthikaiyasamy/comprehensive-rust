//! # Typestate Pattern Exercise: E-Commerce Order Lifecycle
//!
//! In Java, state transitions are frequently tracked via an enum field:
//! ```java
//! public class Order {
//!     private OrderState state;
//!     public void pay() {
//!         if (state != OrderState.PLACED) throw new IllegalStateException("Invalid state");
//!         this.state = OrderState.PAID;
//!     }
//! }
//! ```
//! In Rust, the Typestate Pattern uses **Zero-Sized Types (ZSTs)** and **Affine Types (Moves)**
//! to enforce valid lifecycle transitions at compile time with zero runtime overhead.
//! Invalid operations fail to compile, eliminating entire classes of runtime exceptions.

use std::marker::PhantomData;

// --- State Markers (Zero-Sized Types) ---
#[derive(Debug, PartialEq, Eq)]
pub struct Draft;

#[derive(Debug, PartialEq, Eq)]
pub struct Placed;

#[derive(Debug, PartialEq, Eq)]
pub struct Paid {
    pub payment_id: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Shipped {
    pub tracking_number: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Delivered;

#[derive(Debug, PartialEq, Eq)]
pub struct Cancelled {
    pub reason: String,
}

// --- Order Item ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderItem {
    pub sku: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
}

// --- Generic Order Struct ---
#[derive(Debug, PartialEq, Eq)]
pub struct Order<State> {
    pub id: u64,
    pub customer_id: u64,
    pub items: Vec<OrderItem>,
    pub state_data: State,
    _marker: PhantomData<State>,
}

// --- Draft State Operations ---
impl Order<Draft> {
    pub fn new(id: u64, customer_id: u64) -> Self {
        Order {
            id,
            customer_id,
            items: Vec::new(),
            state_data: Draft,
            _marker: PhantomData,
        }
    }

    pub fn add_item(mut self, item: OrderItem) -> Self {
        self.items.push(item);
        self
    }

    /// Transitions order from Draft to Placed.
    /// Fails if the cart is empty.
    pub fn place(self) -> Result<Order<Placed>, (Self, &'static str)> {
        if self.items.is_empty() {
            Err((self, "Cannot place an empty order"))
        } else {
            Ok(Order {
                id: self.id,
                customer_id: self.customer_id,
                items: self.items,
                state_data: Placed,
                _marker: PhantomData,
            })
        }
    }
}

// --- Placed State Operations ---
impl Order<Placed> {
    pub fn total_cents(&self) -> u64 {
        self.items
            .iter()
            .map(|item| item.unit_price_cents * item.quantity as u64)
            .sum()
    }

    /// Transitions order from Placed to Paid.
    /// Consumes the Placed order so it cannot be double-paid.
    pub fn pay(self, payment_id: impl Into<String>) -> Order<Paid> {
        Order {
            id: self.id,
            customer_id: self.customer_id,
            items: self.items,
            state_data: Paid {
                payment_id: payment_id.into(),
            },
            _marker: PhantomData,
        }
    }

    /// Cancellation is permitted while in the Placed state.
    pub fn cancel(self, reason: impl Into<String>) -> Order<Cancelled> {
        Order {
            id: self.id,
            customer_id: self.customer_id,
            items: self.items,
            state_data: Cancelled {
                reason: reason.into(),
            },
            _marker: PhantomData,
        }
    }
}

// --- Paid State Operations ---
impl Order<Paid> {
    pub fn payment_id(&self) -> &str {
        &self.state_data.payment_id
    }

    /// Transitions order from Paid to Shipped.
    pub fn ship(self, tracking_number: impl Into<String>) -> Order<Shipped> {
        Order {
            id: self.id,
            customer_id: self.customer_id,
            items: self.items,
            state_data: Shipped {
                tracking_number: tracking_number.into(),
            },
            _marker: PhantomData,
        }
    }

    /// Paid orders can still be cancelled (e.g. customer requested refund before shipping).
    pub fn refund_and_cancel(self, reason: impl Into<String>) -> Order<Cancelled> {
        Order {
            id: self.id,
            customer_id: self.customer_id,
            items: self.items,
            state_data: Cancelled {
                reason: reason.into(),
            },
            _marker: PhantomData,
        }
    }
}

// --- Shipped State Operations ---
impl Order<Shipped> {
    pub fn tracking_number(&self) -> &str {
        &self.state_data.tracking_number
    }

    /// Transitions order from Shipped to Delivered.
    /// Notice: There is NO cancel() method on Order<Shipped>!
    /// Once shipped, cancellation is physically uncallable at compile time.
    pub fn deliver(self) -> Order<Delivered> {
        Order {
            id: self.id,
            customer_id: self.customer_id,
            items: self.items,
            state_data: Delivered,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path_lifecycle() {
        let order = Order::new(101, 42)
            .add_item(OrderItem {
                sku: "RUST-BOOK".to_string(),
                quantity: 2,
                unit_price_cents: 3500,
            })
            .add_item(OrderItem {
                sku: "FERRIS-PLUSH".to_string(),
                quantity: 1,
                unit_price_cents: 2000,
            });

        // Transition: Draft -> Placed
        let placed = order.place().expect("order should place successfully");
        assert_eq!(placed.total_cents(), 9000);

        // Transition: Placed -> Paid
        let paid = placed.pay("tx_stripe_999888");
        assert_eq!(paid.payment_id(), "tx_stripe_999888");

        // Transition: Paid -> Shipped
        let shipped = paid.ship("FEDEX-TRK-774411");
        assert_eq!(shipped.tracking_number(), "FEDEX-TRK-774411");

        // Transition: Shipped -> Delivered
        let delivered = shipped.deliver();
        assert_eq!(delivered.state_data, Delivered);
    }

    #[test]
    fn test_empty_draft_cannot_be_placed() {
        let draft = Order::new(102, 55);
        let place_result = draft.place();
        assert!(place_result.is_err());
        let (recovered_draft, err_msg) = place_result.unwrap_err();
        assert_eq!(err_msg, "Cannot place an empty order");

        // We recovered ownership of the draft! We can add an item and retry:
        let placed = recovered_draft
            .add_item(OrderItem {
                sku: "WIDGET-01".to_string(),
                quantity: 1,
                unit_price_cents: 500,
            })
            .place()
            .expect("should now succeed");

        assert_eq!(placed.total_cents(), 500);
    }

    #[test]
    fn test_placed_order_cancellation() {
        let order = Order::new(103, 77)
            .add_item(OrderItem {
                sku: "LAPTOP-STAND".to_string(),
                quantity: 1,
                unit_price_cents: 4500,
            })
            .place()
            .unwrap();

        let cancelled = order.cancel("Customer changed mind");
        assert_eq!(cancelled.state_data.reason, "Customer changed mind");
    }

    #[test]
    fn test_zero_runtime_overhead_proof() {
        use std::mem::size_of;

        // Draft and Delivered are zero-sized types (0 bytes)
        assert_eq!(size_of::<Draft>(), 0);
        assert_eq!(size_of::<Delivered>(), 0);

        // Order<Draft> and Order<Delivered> have identical memory layout
        assert_eq!(size_of::<Order<Draft>>(), size_of::<Order<Delivered>>());
    }
}
