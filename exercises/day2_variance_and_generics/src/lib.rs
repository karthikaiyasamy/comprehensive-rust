//! # Deep Dive: Variance, Monomorphization, and Type Layouts
//!
//! This module provides hands-on demonstrations and tests for:
//! 1. **Niche Optimization**: Zero-cost `Option<T>` for pointers and `NonZero*` types.
//! 2. **Fat Pointers vs Monomorphization**: Memory layout of `dyn Trait` vs static dispatch.
//! 3. **Type-Safe Newtypes**: Eliminating primitive obsession at zero runtime cost.
//! 4. **Variance with PhantomData**: Covariance vs Invariance in lifetime-parameterized types.

use std::marker::PhantomData;
use std::num::NonZeroU64;
use std::ops::Add;
use std::ptr::NonNull;

// =========================================================================
// 1. Zero-Cost Newtypes
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsdCents(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EurCents(pub i64);

impl Add for UsdCents {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        UsdCents(self.0 + rhs.0)
    }
}

impl Add for EurCents {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        EurCents(self.0 + rhs.0)
    }
}

// =========================================================================
// 2. Monomorphization vs Dynamic Dispatch (`dyn Trait`)
// =========================================================================

pub trait Formatter {
    fn format(&self, input: &str) -> String;
}

pub struct UpperCaseFormatter;
impl Formatter for UpperCaseFormatter {
    fn format(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

pub struct BracketFormatter;
impl Formatter for BracketFormatter {
    fn format(&self, input: &str) -> String {
        format!("[{input}]")
    }
}

/// Static Dispatch: Monomorphized at compile time.
/// The compiler generates dedicated native code for each concrete F.
/// Zero runtime cost, inlinable, no vtable lookup.
pub fn format_static<F: Formatter>(formatter: &F, input: &str) -> String {
    formatter.format(input)
}

/// Dynamic Dispatch: Resolved at runtime via a 16-byte Fat Pointer.
/// [ data_ptr (8 bytes) | vtable_ptr (8 bytes) ]
pub fn format_dynamic(formatter: &dyn Formatter, input: &str) -> String {
    formatter.format(input)
}

// =========================================================================
// 3. Variance & PhantomData
// =========================================================================

/// Covariant over lifetime `'a` and type `T`:
/// Because it behaves like an immutable reference `&'a T`,
/// if `'a: 'b` (lifetime `'a` outlives `'b`), then
/// `CovariantWrapper<'a, T>` can be used where `CovariantWrapper<'b, T>` is expected.
pub struct CovariantWrapper<'a, T> {
    _marker: PhantomData<&'a T>,
}

impl<'a, T> CovariantWrapper<'a, T> {
    pub fn new() -> Self {
        CovariantWrapper {
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Default for CovariantWrapper<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Invariant over `T`:
/// Because `&'a mut T` allows both reading AND writing `T`,
/// subtyping is disabled for `T` to prevent storing a supertype where a subtype is required.
pub struct InvariantWrapper<'a, T> {
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> InvariantWrapper<'a, T> {
    pub fn new() -> Self {
        InvariantWrapper {
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Default for InvariantWrapper<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_newtype_zero_runtime_overhead() {
        // A Newtype has the EXACT same size in memory as its wrapped primitive
        assert_eq!(size_of::<UsdCents>(), size_of::<i64>());
        assert_eq!(size_of::<EurCents>(), size_of::<i64>());
        assert_eq!(size_of::<UsdCents>(), 8);

        let salary = UsdCents(500_000);
        let bonus = UsdCents(50_000);
        let total = salary + bonus;
        assert_eq!(total, UsdCents(550_000));

        // Compiler prevents accidentally mixing currencies:
        // let eur = EurCents(100);
        // let _ = salary + eur; // COMPILE ERROR: mismatched types!
    }

    #[test]
    fn test_niche_filling_optimization() {
        // In Java: Optional<Long> requires a 16-24 byte heap object + 8 byte reference.
        // In Rust: Because references and NonZero types cannot be 0x0,
        // the compiler uses 0x0 as the `None` sentinel variant at zero cost!

        // 1. Reference niche: Option<&T> is exactly 8 bytes (same as &T)
        assert_eq!(size_of::<&str>(), 16); // fat pointer: ptr (8) + len (8)
        assert_eq!(size_of::<Option<&str>>(), 16); // Still 16! None is null ptr

        assert_eq!(size_of::<&i32>(), 8);
        assert_eq!(size_of::<Option<&i32>>(), 8); // Still 8 bytes!

        // 2. NonZero niche: Option<NonZeroU64> is exactly 8 bytes (same as u64)
        assert_eq!(size_of::<NonZeroU64>(), 8);
        assert_eq!(size_of::<Option<NonZeroU64>>(), 8);

        // 3. NonNull niche: Option<NonNull<u8>> is exactly 8 bytes
        assert_eq!(size_of::<NonNull<u8>>(), 8);
        assert_eq!(size_of::<Option<NonNull<u8>>>(), 8);

        // 4. Contrast with standard primitive without niche:
        // u64 has all 2^64 bit patterns valid, so Option<u64> needs a 1-byte tag + 7 bytes padding = 16 bytes!
        assert_eq!(size_of::<u64>(), 8);
        assert_eq!(size_of::<Option<u64>>(), 16);
    }

    #[test]
    fn test_fat_pointer_dynamic_dispatch_layout() {
        let upper = UpperCaseFormatter;
        let bracket = BracketFormatter;

        // Static dispatch:
        assert_eq!(format_static(&upper, "hello"), "HELLO");
        assert_eq!(format_static(&bracket, "hello"), "[hello]");

        // Dynamic dispatch:
        let trait_ref: &dyn Formatter = &upper;
        assert_eq!(format_dynamic(trait_ref, "rust"), "RUST");

        // Proof: Trait object reference is a 16-byte Fat Pointer
        // Word 1: Pointer to data (&upper)
        // Word 2: Pointer to vtable (UpperCaseFormatter VTable)
        assert_eq!(size_of::<&dyn Formatter>(), 16);

        // The concrete struct itself is a Zero-Sized Type:
        assert_eq!(size_of::<UpperCaseFormatter>(), 0);
        assert_eq!(size_of::<BracketFormatter>(), 0);
    }

    #[test]
    fn test_lifetime_covariance() {
        // Static string outlives any local scope ('static : 'a)
        let static_str: &'static str = "permanent";

        fn check_covariance<'a>(_w: CovariantWrapper<'a, &'a str>) {
            // Compiles because CovariantWrapper is covariant over 'a
        }

        let w: CovariantWrapper<'static, &'static str> = CovariantWrapper::new();
        check_covariance(w);
    }
}
