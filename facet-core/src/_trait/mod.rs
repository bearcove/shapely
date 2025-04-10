#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

pub use crate::types::*;

mod impls;

mod macros;
pub use macros::*;

/// Allows querying the [`Shape`] of a type, which in turn lets us inspect any fields, build a value of
/// this type progressively, etc.
///
/// # Safety
///
/// If you implement this wrong, all the safe abstractions in `facet-peek`, `facet-poke`,
/// all the serializers, deserializers, the entire ecosystem is unsafe.
///
/// You're responsible for describing the type layout properly, and annotating all the invariants.
pub unsafe trait Facet: Sized {
    /// The shape of this type
    const SHAPE: &'static Shape;

    /// Returns true if the type of `self` is equal to the type of `other`
    fn type_eq<Other: Facet>() -> bool {
        Self::SHAPE == Other::SHAPE
    }
}

/// Extension trait to provide `is_type` and `assert_type`
pub trait ShapeExt {
    /// Check if this shape is of the given type
    fn is_type<Other: Facet>(&'static self) -> bool;

    /// Assert that this shape is of the given type, panicking if it's not
    fn assert_type<Other: Facet>(&'static self);
}

impl ShapeExt for Shape {
    /// Check if this shape is of the given type
    fn is_type<Other: Facet>(&'static self) -> bool {
        self == Other::SHAPE
    }

    /// Assert that this shape is of the given type, panicking if it's not
    fn assert_type<Other: Facet>(&'static self) {
        assert!(
            self.is_type::<Other>(),
            "Type mismatch: expected {}, found {self}",
            Other::SHAPE,
        );
    }
}
