use bitflags::bitflags;
use shapely_opaque::{Opaque, OpaqueConst, OpaqueUninit};
use std::cmp::Ordering;

use crate::Shape;

//======== Type Information ========

/// A function that formats the name of a type.
///
/// This helps avoid allocations, and it takes options.
pub type TypeNameFn = fn(f: &mut std::fmt::Formatter, opts: TypeNameOpts) -> std::fmt::Result;

/// Options for formatting the name of a type
#[non_exhaustive]
#[derive(Clone, Copy)]
pub struct TypeNameOpts {
    /// as long as this is > 0, keep formatting the type parameters
    /// when it reaches 0, format type parameters as `...`
    /// if negative, all type parameters are formatted
    pub recurse_ttl: isize,
}

impl Default for TypeNameOpts {
    fn default() -> Self {
        Self { recurse_ttl: -1 }
    }
}

impl TypeNameOpts {
    /// Create a new `NameOpts` for which none of the type parameters are formatted
    pub fn none() -> Self {
        Self { recurse_ttl: 0 }
    }

    /// Create a new `NameOpts` for which only the direct children are formatted
    pub fn one() -> Self {
        Self { recurse_ttl: 1 }
    }

    /// Create a new `NameOpts` for which all type parameters are formatted
    pub fn infinite() -> Self {
        Self { recurse_ttl: -1 }
    }

    /// Decrease the `recurse_ttl` — if it's != 0, returns options to pass when
    /// formatting children type parameters.
    ///
    /// If this returns `None` and you have type parameters, you should render a
    /// `…` (unicode ellipsis) character instead of your list of types.
    ///
    /// See the implementation for `Vec` for examples.
    pub fn for_children(&self) -> Option<Self> {
        if self.recurse_ttl > 0 {
            Some(Self {
                recurse_ttl: self.recurse_ttl - 1,
            })
        } else if self.recurse_ttl < 0 {
            Some(Self {
                recurse_ttl: self.recurse_ttl,
            })
        } else {
            None
        }
    }
}

//======== Memory Management ========

/// Function to drop a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DropInPlaceFn = for<'mem> unsafe fn(value: Opaque<'mem>);

/// Generates a [`DropInPlaceFn`] for a concrete type
pub const fn drop_in_place_fn_for<T>() -> Option<DropInPlaceFn> {
    Some(|value: Opaque<'_>| unsafe {
        value.drop_in_place::<T>();
    })
}

/// Function to clone a value into another already-allocated value
///
/// # Safety
///
/// The `source` parameter must point to aligned, initialized memory of the correct type.
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. The function returns the same pointer wrapped in an [`Opaque`].
pub type CloneIntoFn = for<'src, 'dst> unsafe fn(
    source: OpaqueConst<'src>,
    target: OpaqueUninit<'dst>,
) -> Opaque<'dst>;

/// Generates a [`CloneInPlaceFn`] for a concrete type
pub const fn clone_into_fn_for<T: Clone>() -> Option<CloneIntoFn> {
    Some(|source: OpaqueConst<'_>, target: OpaqueUninit<'_>| unsafe {
        let source_val = source.as_ref::<T>();
        target.write(source_val.clone())
    })
}

/// Function to set a value to its default in-place
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. The function returns the same pointer wrapped in an [`Opaque`].
pub type DefaultInPlaceFn = for<'mem> unsafe fn(target: OpaqueUninit<'mem>) -> Opaque<'mem>;

/// Generates a [`DefaultInPlaceFn`] for a concrete type
pub const fn default_in_place_fn_for<T: Default>() -> Option<DefaultInPlaceFn> {
    Some(|target: OpaqueUninit<'_>| unsafe { target.write(T::default()) })
}

//======== Conversion ========

/// Function to parse a value from a string.
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Ok` with the
/// same pointer wrapped in an [`Opaque`]. If parsing fails, it returns `Err` with an error.
pub type ParseFn =
    for<'mem> unsafe fn(s: &str, target: OpaqueUninit<'mem>) -> Result<Opaque<'mem>, ParseError>;

/// Generates a [`ParseFn`] for a concrete type
pub const fn parse_fn_for<T: std::str::FromStr>() -> Option<ParseFn> {
    Some(|s: &str, target: OpaqueUninit<'_>| unsafe {
        match s.parse::<T>() {
            Ok(value) => Ok(target.write(value)),
            Err(_) => Err(ParseError::Generic("failed to parse string")),
        }
    })
}

/// Error returned by [`ParseFn`]
#[non_exhaustive]
#[derive(Debug)]
pub enum ParseError {
    /// Generic error message
    Generic(&'static str),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Generic(msg) => write!(f, "Parse failed: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Function to try converting from another type
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Ok` with the
/// same pointer wrapped in an [`Opaque`]. If conversion fails, it returns `Err` with an error.
pub type TryFromFn = for<'src, 'mem> unsafe fn(
    source: OpaqueConst<'src>,
    target: OpaqueUninit<'mem>,
) -> Result<Opaque<'mem>, TryFromError>;

/// Error type for TryFrom conversion failures
#[non_exhaustive]
#[derive(Debug)]
pub enum TryFromError {
    /// Generic conversion error
    Generic(&'static str),
    /// The target shape doesn't implement conversion from any source shape (no try_from in vtable)
    Unimplemented(&'static Shape),
    /// The target shape has a conversion implementation, but it doesn't support converting from this specific source shape
    Incompatible {
        /// The source shape that we tried to convert from
        source: &'static Shape,
        /// The target shape that we tried to convert to
        target: &'static Shape,
    },
}

impl std::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryFromError::Generic(msg) => write!(f, "Conversion failed: {}", msg),
            TryFromError::Unimplemented(shape) => write!(
                f,
                "Conversion failed: Shape {} doesn't implement any conversions (no try_from function)",
                shape
            ),
            TryFromError::Incompatible { source, target } => write!(
                f,
                "Conversion failed: Cannot convert from shape {} to shape {}",
                source, target
            ),
        }
    }
}

impl std::error::Error for TryFromError {}

//======== Comparison ========

/// Function to check if two values are partially equal
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type PartialEqFn = for<'l, 'r> unsafe fn(left: OpaqueConst<'l>, right: OpaqueConst<'r>) -> bool;

/// Generates a [`PartialEqFn`] for a concrete type
pub const fn partial_eq_fn_for<T: PartialEq>() -> Option<PartialEqFn> {
    Some(|left: OpaqueConst<'_>, right: OpaqueConst<'_>| -> bool {
        let left_val = unsafe { left.as_ref::<T>() };
        let right_val = unsafe { right.as_ref::<T>() };
        left_val == right_val
    })
}

/// Function to compare two values and return their ordering if comparable
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type PartialOrdFn =
    for<'l, 'r> unsafe fn(left: OpaqueConst<'l>, right: OpaqueConst<'r>) -> Option<Ordering>;

/// Generates a [`PartialOrdFn`] for a concrete type
pub const fn partial_ord_fn_for<T: PartialOrd>() -> Option<PartialOrdFn> {
    Some(
        |left: OpaqueConst<'_>, right: OpaqueConst<'_>| -> Option<Ordering> {
            let left_val = unsafe { left.as_ref::<T>() };
            let right_val = unsafe { right.as_ref::<T>() };
            left_val.partial_cmp(right_val)
        },
    )
}

/// Function to compare two values and return their ordering
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type CmpFn = for<'l, 'r> unsafe fn(left: OpaqueConst<'l>, right: OpaqueConst<'r>) -> Ordering;

/// Generates a [`CmpFn`] for a concrete type
pub const fn cmp_fn_for<T: Ord>() -> Option<CmpFn> {
    Some(
        |left: OpaqueConst<'_>, right: OpaqueConst<'_>| -> Ordering {
            let left_val = unsafe { left.as_ref::<T>() };
            let right_val = unsafe { right.as_ref::<T>() };
            left_val.cmp(right_val)
        },
    )
}

//======== Hashing ========

/// Function to hash a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
/// The hasher pointer must be a valid pointer to a Hasher trait object.
pub type HashFn = for<'mem> unsafe fn(
    value: OpaqueConst<'mem>,
    hasher_this: Opaque<'mem>,
    hasher_write_fn: HasherWriteFn,
);

/// Generates a [`HashFn`] for a concrete type
pub const fn hash_fn_for<T: std::hash::Hash>() -> Option<HashFn> {
    Some(
        |value: OpaqueConst<'_>, hasher_this: Opaque<'_>, hasher_write_fn: HasherWriteFn| unsafe {
            let val = value.as_ref::<T>();
            val.hash(&mut HasherProxy::new(hasher_this, hasher_write_fn));
        },
    )
}

/// Function to write bytes to a hasher
///
/// # Safety
///
/// The `hasher_self` parameter must be a valid pointer to a hasher
pub type HasherWriteFn = for<'mem> unsafe fn(hasher_self: Opaque<'mem>, bytes: &[u8]);

/// Provides an implementation of [`std::hash::Hasher`] for a given hasher pointer and write function
///
/// See [`HashFn`] for more details on the parameters.
///
/// Example usage (for a type that already implements `Hasher`)
///
/// ```rust,ignore
/// hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
///     value
///         .as_ref::<Self>()
///         .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
/// }),
/// ```
pub struct HasherProxy<'a> {
    hasher_this: Opaque<'a>,
    hasher_write_fn: HasherWriteFn,
}

impl<'a> HasherProxy<'a> {
    /// Create a new `HasherProxy` from a hasher pointer and a write function
    ///
    /// # Safety
    ///
    /// The `hasher_this` parameter must be a valid pointer to a Hasher trait object.
    /// The `hasher_write_fn` parameter must be a valid function pointer.
    pub unsafe fn new(hasher_this: Opaque<'a>, hasher_write_fn: HasherWriteFn) -> Self {
        Self {
            hasher_this,
            hasher_write_fn,
        }
    }
}

impl<'a> std::hash::Hasher for HasherProxy<'a> {
    fn finish(&self) -> u64 {
        unimplemented!("finish is not needed for this implementation")
    }
    fn write(&mut self, bytes: &[u8]) {
        unsafe { (self.hasher_write_fn)(self.hasher_this, bytes) }
    }
}

//======== Marker Traits ========

bitflags! {
    /// Bitflags for common marker traits that a type may implement
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MarkerTraits: u8 {
        /// Indicates that the type implements the [`Eq`] marker trait
        const EQ = 1 << 0;
        /// Indicates that the type implements the [`Send`] marker trait
        const SEND = 1 << 1;
        /// Indicates that the type implements the [`Sync`] marker trait
        const SYNC = 1 << 2;
        /// Indicates that the type implements the [`Copy`] marker trait
        const COPY = 1 << 3;
    }
}

//======== Display and Debug ========

/// Function to format a value for display
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DisplayFn =
    for<'mem> unsafe fn(value: OpaqueConst<'mem>, f: &mut std::fmt::Formatter) -> std::fmt::Result;

/// Generates a [`DisplayFn`] for a concrete type
pub const fn display_fn_for<T: std::fmt::Display>() -> Option<DisplayFn> {
    Some(
        |value: OpaqueConst<'_>, f: &mut std::fmt::Formatter| -> std::fmt::Result {
            let val = unsafe { value.as_ref::<T>() };
            write!(f, "{val}")
        },
    )
}

/// Function to format a value for debug.
/// If this returns None, the shape did not implement Debug.
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type DebugFn =
    for<'mem> unsafe fn(value: OpaqueConst<'mem>, f: &mut std::fmt::Formatter) -> std::fmt::Result;

/// Generates a [`DebugFn`] for a concrete type
pub const fn debug_fn_for<T: std::fmt::Debug>() -> Option<DebugFn> {
    Some(
        |value: OpaqueConst<'_>, f: &mut std::fmt::Formatter| -> std::fmt::Result {
            let val = unsafe { value.as_ref::<T>() };
            write!(f, "{val:?}")
        },
    )
}

/// VTable for common operations that can be performed on any shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueVTable {
    /// cf. [`TypeNameFn`]
    pub type_name: TypeNameFn,

    /// cf. [`DisplayFn`]
    pub display: Option<DisplayFn>,

    /// cf. [`DebugFn`]
    pub debug: Option<DebugFn>,

    /// cf. [`DefaultInPlaceFn`]
    pub default_in_place: Option<DefaultInPlaceFn>,

    /// cf. [`CloneInPlaceFn`]
    pub clone_into: Option<CloneIntoFn>,

    /// Marker traits implemented by the type
    // FIXME: move out of vtable, it's not really... functions.
    // Belongs in Shape directly.
    pub marker_traits: MarkerTraits,

    /// cf. [`PartialEqFn`] for equality comparison
    pub eq: Option<PartialEqFn>,

    /// cf. [`PartialOrdFn`] for partial ordering comparison
    pub partial_ord: Option<PartialOrdFn>,

    /// cf. [`CmpFn`] for total ordering
    pub ord: Option<CmpFn>,

    /// cf. [`HashFn`]
    pub hash: Option<HashFn>,

    /// cf. [`DropInPlaceFn`] — if None, drops without side-effects
    pub drop_in_place: Option<DropInPlaceFn>,

    /// cf. [`ParseFn`]
    pub parse: Option<ParseFn>,

    /// cf. [`TryFromFn`]
    pub try_from: Option<TryFromFn>,
}

impl ValueVTable {
    /// Check if the type implements the [`Eq`] marker trait
    pub fn is_eq(&self) -> bool {
        self.marker_traits.contains(MarkerTraits::EQ)
    }

    /// Check if the type implements the [`Send`] marker trait
    pub fn is_send(&self) -> bool {
        self.marker_traits.contains(MarkerTraits::SEND)
    }

    /// Check if the type implements the [`Sync`] marker trait
    pub fn is_sync(&self) -> bool {
        self.marker_traits.contains(MarkerTraits::SYNC)
    }

    /// Check if the type implements the [`Copy`] marker trait
    pub fn is_copy(&self) -> bool {
        self.marker_traits.contains(MarkerTraits::COPY)
    }
}
