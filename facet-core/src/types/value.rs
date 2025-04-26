use crate::{
    TypedPtrUninit,
    ptr::{PtrConst, PtrMut, PtrUninit},
};
use bitflags::bitflags;
use core::{cmp::Ordering, marker::PhantomData, mem};

use crate::Shape;

use super::UnsizedError;

//======== Type Information ========

/// A function that formats the name of a type.
///
/// This helps avoid allocations, and it takes options.
pub type TypeNameFn = fn(f: &mut core::fmt::Formatter, opts: TypeNameOpts) -> core::fmt::Result;

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
        match self.recurse_ttl.cmp(&0) {
            Ordering::Greater => Some(Self {
                recurse_ttl: self.recurse_ttl - 1,
            }),
            Ordering::Less => Some(Self {
                recurse_ttl: self.recurse_ttl,
            }),
            Ordering::Equal => None,
        }
    }
}

//======== Invariants ========

/// Function to validate the invariants of a value. If it returns false, the value is considered invalid.
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
pub type InvariantsFn = for<'mem> unsafe fn(value: PtrConst<'mem>) -> bool;
/// Function to validate the invariants of a value. If it returns false, the value is considered invalid.
pub type InvariantsFnTyped<T> = fn(value: &T) -> bool;

//======== Memory Management ========

/// Function to drop a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
/// After calling this function, the memory pointed to by `value` should not be accessed again
/// until it is properly reinitialized.
pub type DropInPlaceFn = for<'mem> unsafe fn(value: PtrMut<'mem>) -> PtrUninit<'mem>;

/// Function to clone a value into another already-allocated value
///
/// # Safety
///
/// The `source` parameter must point to aligned, initialized memory of the correct type.
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. The function returns the same pointer wrapped in an [`PtrMut`].
pub type CloneIntoFn =
    for<'src, 'dst> unsafe fn(source: PtrConst<'src>, target: PtrUninit<'dst>) -> PtrMut<'dst>;
/// Function to clone a value into another already-allocated value
pub type CloneIntoFnTyped<T> =
    for<'src, 'dst> fn(source: &'src T, target: TypedPtrUninit<'dst, T>) -> &'dst mut T;

/// Function to set a value to its default in-place
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. The function returns the same pointer wrapped in an [`PtrMut`].
pub type DefaultInPlaceFn = for<'mem> unsafe fn(target: PtrUninit<'mem>) -> PtrMut<'mem>;
/// Function to set a value to its default in-place
pub type DefaultInPlaceFnTyped<T> = for<'mem> fn(target: TypedPtrUninit<'mem, T>) -> &'mem mut T;

//======== Conversion ========

/// Function to parse a value from a string.
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Ok` with the
/// same pointer wrapped in an [`PtrMut`]. If parsing fails, it returns `Err` with an error.
pub type ParseFn =
    for<'mem> unsafe fn(s: &str, target: PtrUninit<'mem>) -> Result<PtrMut<'mem>, ParseError>;
/// Function to parse a value from a string.
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
pub type ParseFnTyped<T> =
    for<'mem> fn(s: &str, target: TypedPtrUninit<'mem, T>) -> Result<&'mem mut T, ParseError>;

/// Error returned by [`ParseFn`]
#[non_exhaustive]
#[derive(Debug)]
pub enum ParseError {
    /// Generic error message
    Generic(&'static str),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::Generic(msg) => write!(f, "Parse failed: {}", msg),
        }
    }
}

impl core::error::Error for ParseError {}

/// Function to try converting from another type
///
/// # Safety
///
/// The `target` parameter has the correct layout and alignment, but points to
/// uninitialized memory. If this function succeeds, it should return `Ok` with the
/// same pointer wrapped in an [`PtrMut`]. If conversion fails, it returns `Err` with an error.
pub type TryFromFn = for<'src, 'mem> unsafe fn(
    source: PtrConst<'src>,
    source_shape: &'static Shape,
    target: PtrUninit<'mem>,
) -> Result<PtrMut<'mem>, TryFromError>;
/// Function to try converting from another type
pub type TryFromFnTyped<T> = for<'src, 'mem> fn(
    source: &'src T,
    source_shape: &'static Shape,
    target: TypedPtrUninit<'mem, T>,
) -> Result<&'mem mut T, TryFromError>;

/// Error type for TryFrom conversion failures
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum TryFromError {
    /// Generic conversion error
    Generic(&'static str),
    /// The target shape doesn't implement conversion from any source shape (no try_from in vtable)
    Unimplemented,
    /// The target shape has a conversion implementation, but it doesn't support converting from this specific source shape
    UnsupportedSourceShape {
        /// The source shape that failed to convert
        src_shape: &'static Shape,

        /// The shapes that the `TryFrom` implementation supports
        expected: &'static [&'static Shape],
    },
    /// `!Sized` type
    Unsized,
}

impl core::fmt::Display for TryFromError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TryFromError::Generic(msg) => write!(f, "{}", msg),
            TryFromError::Unimplemented => write!(
                f,
                "Shape doesn't implement any conversions (no try_from function)",
            ),
            TryFromError::UnsupportedSourceShape {
                src_shape: source_shape,
                expected,
            } => {
                write!(f, "Incompatible types: {} (expected one of ", source_shape)?;
                for (index, sh) in expected.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", sh)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            TryFromError::Unsized => write!(f, "Unsized type"),
        }
    }
}

impl core::error::Error for TryFromError {}

impl From<UnsizedError> for TryFromError {
    fn from(_value: UnsizedError) -> Self {
        Self::Unsized
    }
}

/// Function to convert a transparent/newtype wrapper into its inner type.
///
/// This is used for types that wrap another type (like smart pointers, newtypes, etc.)
/// where the wrapper can be unwrapped to access the inner value. Primarily used during serialization.
///
/// # Safety
///
/// This function is unsafe because it operates on raw pointers.
///
/// The `src_ptr` must point to a valid, initialized instance of the wrapper type.
/// The `dst` pointer must point to valid, uninitialized memory suitable for holding an instance
/// of the inner type.
///
/// The function will return a pointer to the initialized inner value.
pub type TryIntoInnerFn = for<'src, 'dst> unsafe fn(
    src_ptr: PtrConst<'src>,
    dst: PtrUninit<'dst>,
) -> Result<PtrMut<'dst>, TryIntoInnerError>;
/// Function to convert a transparent/newtype wrapper into its inner type.
///
/// This is used for types that wrap another type (like smart pointers, newtypes, etc.)
/// where the wrapper can be unwrapped to access the inner value. Primarily used during serialization.
pub type TryIntoInnerFnTyped<T> = for<'src, 'dst> fn(
    src_ptr: &'src T,
    dst: TypedPtrUninit<'dst, T>,
) -> Result<&'dst mut T, TryIntoInnerError>;

/// Error type returned by [`TryIntoInnerFn`] when attempting to extract
/// the inner value from a wrapper type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryIntoInnerError {
    /// Indicates that the inner value cannot be extracted at this time,
    /// such as when a mutable borrow is already active.
    Unavailable,
    /// Indicates that another unspecified error occurred during extraction.
    Other(&'static str),
}

impl core::fmt::Display for TryIntoInnerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TryIntoInnerError::Unavailable => {
                write!(f, "inner value is unavailable for extraction")
            }
            TryIntoInnerError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl core::error::Error for TryIntoInnerError {}

/// Function to borrow the inner value from a transparent/newtype wrapper without copying.
///
/// This is used for types that wrap another type (like smart pointers, newtypes, etc.)
/// to efficiently access the inner value without transferring ownership.
///
/// # Safety
///
/// This function is unsafe because it operates on raw pointers.
///
/// The `src_ptr` must point to a valid, initialized instance of the wrapper type.
/// The returned pointer points to memory owned by the wrapper and remains valid
/// as long as the wrapper is valid and not mutated.
pub type TryBorrowInnerFn =
    for<'src> unsafe fn(src_ptr: PtrConst<'src>) -> Result<PtrConst<'src>, TryBorrowInnerError>;
/// Function to borrow the inner value from a transparent/newtype wrapper without copying.
///
/// This is used for types that wrap another type (like smart pointers, newtypes, etc.)
/// to efficiently access the inner value without transferring ownership.
pub type TryBorrowInnerFnTyped<T> =
    for<'src> fn(src_ptr: &'src T) -> Result<&'src T, TryBorrowInnerError>;

/// Error type returned by [`TryBorrowInnerFn`] when attempting to borrow
/// the inner value from a wrapper type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryBorrowInnerError {
    /// Indicates that the inner value cannot be borrowed at this time,
    /// such as when a mutable borrow is already active.
    Unavailable,
    /// Indicates an other, unspecified error occurred during the borrow attempt.
    /// The contained string provides a description of the error.
    Other(&'static str),
}

impl core::fmt::Display for TryBorrowInnerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TryBorrowInnerError::Unavailable => {
                write!(f, "inner value is unavailable for borrowing")
            }
            TryBorrowInnerError::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl core::error::Error for TryBorrowInnerError {}

//======== Comparison ========

/// Function to check if two values are partially equal
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type PartialEqFn = for<'l, 'r> unsafe fn(left: PtrConst<'l>, right: PtrConst<'r>) -> bool;
/// Function to check if two values are partially equal
pub type PartialEqFnTyped<T> = fn(left: &T, right: &T) -> bool;

/// Function to compare two values and return their ordering if comparable
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type PartialOrdFn =
    for<'l, 'r> unsafe fn(left: PtrConst<'l>, right: PtrConst<'r>) -> Option<Ordering>;
/// Function to compare two values and return their ordering if comparable
pub type PartialOrdFnTyped<T> = fn(left: &T, right: &T) -> Option<Ordering>;

/// Function to compare two values and return their ordering
///
/// # Safety
///
/// Both `left` and `right` parameters must point to aligned, initialized memory of the correct type.
pub type CmpFn = for<'l, 'r> unsafe fn(left: PtrConst<'l>, right: PtrConst<'r>) -> Ordering;
/// Function to compare two values and return their ordering
pub type CmpFnTyped<T> = fn(left: &T, right: &T) -> Ordering;

//======== Hashing ========

/// Function to hash a value
///
/// # Safety
///
/// The `value` parameter must point to aligned, initialized memory of the correct type.
/// The hasher pointer must be a valid pointer to a Hasher trait object.
pub type HashFn = for<'mem> unsafe fn(
    value: PtrConst<'mem>,
    hasher_this: PtrMut<'mem>,
    hasher_write_fn: HasherWriteFn,
);
/// Function to hash a value
pub type HashFnTyped<T> =
    for<'mem> fn(value: &'mem T, hasher_this: PtrMut<'mem>, hasher_write_fn: HasherWriteFn);

/// Function to write bytes to a hasher
///
/// # Safety
///
/// The `hasher_self` parameter must be a valid pointer to a hasher
pub type HasherWriteFn = for<'mem> unsafe fn(hasher_self: PtrMut<'mem>, bytes: &[u8]);
/// Function to write bytes to a hasher
pub type HasherWriteFnTyped<T> = for<'mem> fn(hasher_self: &'mem mut T, bytes: &[u8]);

/// Provides an implementation of [`core::hash::Hasher`] for a given hasher pointer and write function
///
/// See [`HashFn`] for more details on the parameters.
pub struct HasherProxy<'a> {
    hasher_this: PtrMut<'a>,
    hasher_write_fn: HasherWriteFn,
}

impl<'a> HasherProxy<'a> {
    /// Create a new `HasherProxy` from a hasher pointer and a write function
    ///
    /// # Safety
    ///
    /// The `hasher_this` parameter must be a valid pointer to a Hasher trait object.
    /// The `hasher_write_fn` parameter must be a valid function pointer.
    pub unsafe fn new(hasher_this: PtrMut<'a>, hasher_write_fn: HasherWriteFn) -> Self {
        Self {
            hasher_this,
            hasher_write_fn,
        }
    }
}

impl core::hash::Hasher for HasherProxy<'_> {
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
        /// Indicates that the type implements the [`Unpin`] marker trait
        const UNPIN = 1 << 4;
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
    for<'mem> unsafe fn(value: PtrConst<'mem>, f: &mut core::fmt::Formatter) -> core::fmt::Result;
/// Function to format a value for display
///
/// If both [`DisplayFn`] and [`ParseFn`] are set, we should be able to round-trip the value.
pub type DisplayFnTyped<T> = fn(value: &T, f: &mut core::fmt::Formatter) -> core::fmt::Result;

/// Function to format a value for debug.
/// If this returns None, the shape did not implement Debug.
pub type DebugFn =
    for<'mem> unsafe fn(value: PtrConst<'mem>, f: &mut core::fmt::Formatter) -> core::fmt::Result;
/// Function to format a value for debug.
/// If this returns None, the shape did not implement Debug.
pub type DebugFnTyped<T> = fn(value: &T, f: &mut core::fmt::Formatter) -> core::fmt::Result;

/// VTable for common operations that can be performed on any shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
#[non_exhaustive]
pub struct ValueVTable {
    /// cf. [`TypeNameFn`]
    pub type_name: TypeNameFn,
    /// Marker traits implemented by the type
    // FIXME: move out of vtable, it's not really... functions.
    // Belongs in Shape directly.
    pub marker_traits: MarkerTraits,

    /// cf. [`DropInPlaceFn`] — if None, drops without side-effects
    pub drop_in_place: Option<DropInPlaceFn>,

    /// cf. [`InvariantsFn`]
    pub invariants: Option<InvariantsFn>,

    /// cf. [`DisplayFn`]
    pub display: Option<DisplayFn>,

    /// cf. [`DebugFn`]
    pub debug: Option<DebugFn>,

    /// cf. [`DefaultInPlaceFn`]
    pub default_in_place: Option<DefaultInPlaceFn>,

    /// cf. [`CloneIntoFn`]
    pub clone_into: Option<CloneIntoFn>,

    /// cf. [`PartialEqFn`] for equality comparison
    pub eq: Option<PartialEqFn>,

    /// cf. [`PartialOrdFn`] for partial ordering comparison
    pub partial_ord: Option<PartialOrdFn>,

    /// cf. [`CmpFn`] for total ordering
    pub ord: Option<CmpFn>,

    /// cf. [`HashFn`]
    pub hash: Option<HashFn>,

    /// cf. [`ParseFn`]
    pub parse: Option<ParseFn>,

    /// cf. [`TryFromFn`]
    ///
    /// This also acts as a "TryFromInner" — you can use it to go:
    ///
    ///   * `String` => `Utf8PathBuf`
    ///   * `String` => `Uuid`
    ///   * `T` => `Option<T>`
    ///   * `T` => `Arc<T>`
    ///   * `T` => `NonZero<T>`
    ///   * etc.
    ///
    pub try_from: Option<TryFromFn>,

    /// cf. [`TryIntoInnerFn`]
    ///
    /// This is used by transparent types to convert the wrapper type into its inner value.
    /// Primarily used during serialization.
    pub try_into_inner: Option<TryIntoInnerFn>,

    /// cf. [`TryBorrowInnerFn`]
    ///
    /// This is used by transparent types to efficiently access the inner value without copying.
    pub try_borrow_inner: Option<TryBorrowInnerFn>,
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

    /// Check if the type implements the [`Unpin`] marker trait
    pub fn is_unpin(&self) -> bool {
        self.marker_traits.contains(MarkerTraits::UNPIN)
    }

    /// Creates a new [`ValueVTableBuilder`]
    pub const fn builder<T>() -> ValueVTableBuilder<T> {
        ValueVTableBuilder::new()
    }
}

/// A typed view of a [`ValueVTable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VTableView<T>(&'static ValueVTable, PhantomData<T>);

impl<'a, T: crate::Facet<'a>> VTableView<T> {
    /// Fetches the vtable for the type.
    pub fn of() -> Self {
        Self(T::SHAPE.vtable, PhantomData)
    }

    /// cf. [`TypeNameFn`]
    #[inline(always)]
    pub fn type_name(self) -> TypeNameFn {
        self.0.type_name
    }

    /// cf. [`InvariantsFn`]
    #[inline(always)]
    pub fn invariants(self) -> Option<InvariantsFnTyped<T>> {
        self.0.invariants.map(|invariants| unsafe {
            mem::transmute::<InvariantsFn, InvariantsFnTyped<T>>(invariants)
        })
    }

    /// cf. [`DisplayFn`]
    #[inline(always)]
    pub fn display(self) -> Option<DisplayFnTyped<T>> {
        self.0
            .display
            .map(|display| unsafe { mem::transmute::<DisplayFn, DisplayFnTyped<T>>(display) })
    }

    /// cf. [`DebugFn`]
    #[inline(always)]
    pub fn debug(self) -> Option<DebugFnTyped<T>> {
        self.0
            .debug
            .map(|debug| unsafe { mem::transmute::<DebugFn, DebugFnTyped<T>>(debug) })
    }

    /// cf. [`DefaultInPlaceFn`]
    #[inline(always)]
    pub fn default_in_place(self) -> Option<DefaultInPlaceFnTyped<T>> {
        self.0.default_in_place.map(|default_in_place| unsafe {
            mem::transmute::<DefaultInPlaceFn, DefaultInPlaceFnTyped<T>>(default_in_place)
        })
    }

    /// cf. [`CloneIntoFn`]
    #[inline(always)]
    pub fn clone_into(self) -> Option<CloneIntoFnTyped<T>> {
        self.0.clone_into.map(|clone_into| unsafe {
            mem::transmute::<CloneIntoFn, CloneIntoFnTyped<T>>(clone_into)
        })
    }

    /// cf. [`PartialEqFn`] for equality comparison
    #[inline(always)]
    pub fn eq(self) -> Option<PartialEqFnTyped<T>> {
        self.0
            .eq
            .map(|eq| unsafe { mem::transmute::<PartialEqFn, PartialEqFnTyped<T>>(eq) })
    }

    /// cf. [`PartialOrdFn`] for partial ordering comparison
    #[inline(always)]
    pub fn partial_ord(self) -> Option<PartialOrdFnTyped<T>> {
        self.0.partial_ord.map(|partial_ord| unsafe {
            mem::transmute::<PartialOrdFn, PartialOrdFnTyped<T>>(partial_ord)
        })
    }

    /// cf. [`CmpFn`] for total ordering
    #[inline(always)]
    pub fn ord(self) -> Option<CmpFnTyped<T>> {
        self.0
            .ord
            .map(|ord| unsafe { mem::transmute::<CmpFn, CmpFnTyped<T>>(ord) })
    }

    /// cf. [`HashFn`]
    #[inline(always)]
    pub fn hash(self) -> Option<HashFnTyped<T>> {
        self.0
            .hash
            .map(|hash| unsafe { mem::transmute::<HashFn, HashFnTyped<T>>(hash) })
    }

    /// cf. [`ParseFn`]
    #[inline(always)]
    pub fn parse(self) -> Option<ParseFnTyped<T>> {
        self.0
            .parse
            .map(|parse| unsafe { mem::transmute::<ParseFn, ParseFnTyped<T>>(parse) })
    }

    /// cf. [`TryFromFn`]
    ///
    /// This also acts as a "TryFromInner" — you can use it to go:
    ///
    ///   * `String` => `Utf8PathBuf`
    ///   * `String` => `Uuid`
    ///   * `T` => `Option<T>`
    ///   * `T` => `Arc<T>`
    ///   * `T` => `NonZero<T>`
    ///   * etc.
    ///
    #[inline(always)]
    pub fn try_from(self) -> Option<TryFromFnTyped<T>> {
        self.0
            .try_from
            .map(|try_from| unsafe { mem::transmute::<TryFromFn, TryFromFnTyped<T>>(try_from) })
    }

    /// cf. [`TryIntoInnerFn`]
    ///
    /// This is used by transparent types to convert the wrapper type into its inner value.
    /// Primarily used during serialization.
    #[inline(always)]
    pub fn try_into_inner(self) -> Option<TryIntoInnerFnTyped<T>> {
        self.0.try_into_inner.map(|try_into_inner| unsafe {
            mem::transmute::<TryIntoInnerFn, TryIntoInnerFnTyped<T>>(try_into_inner)
        })
    }

    /// cf. [`TryBorrowInnerFn`]
    ///
    /// This is used by transparent types to efficiently access the inner value without copying.
    #[inline(always)]
    pub fn try_borrow_inner(self) -> Option<TryBorrowInnerFnTyped<T>> {
        self.0.try_borrow_inner.map(|try_borrow_inner| unsafe {
            mem::transmute::<TryBorrowInnerFn, TryBorrowInnerFnTyped<T>>(try_borrow_inner)
        })
    }
}

/// Builds a [`ValueVTable`]
pub struct ValueVTableBuilder<T> {
    type_name: Option<TypeNameFn>,
    display: Option<DisplayFnTyped<T>>,
    debug: Option<DebugFnTyped<T>>,
    default_in_place: Option<DefaultInPlaceFnTyped<T>>,
    clone_into: Option<CloneIntoFnTyped<T>>,
    marker_traits: MarkerTraits,
    eq: Option<PartialEqFnTyped<T>>,
    partial_ord: Option<PartialOrdFnTyped<T>>,
    ord: Option<CmpFnTyped<T>>,
    hash: Option<HashFnTyped<T>>,
    drop_in_place: Option<DropInPlaceFn>,
    invariants: Option<InvariantsFnTyped<T>>,
    parse: Option<ParseFnTyped<T>>,
    try_from: Option<TryFromFnTyped<T>>,
    try_into_inner: Option<TryIntoInnerFnTyped<T>>,
    try_borrow_inner: Option<TryBorrowInnerFnTyped<T>>,
    _pd: PhantomData<T>,
}

impl<T> ValueVTableBuilder<T> {
    /// Creates a new [`ValueVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            type_name: None,
            display: None,
            debug: None,
            default_in_place: None,
            clone_into: None,
            marker_traits: MarkerTraits::empty(),
            eq: None,
            partial_ord: None,
            ord: None,
            hash: None,
            drop_in_place: None,
            invariants: None,
            parse: None,
            try_from: None,
            try_into_inner: None,
            try_borrow_inner: None,
            _pd: PhantomData,
        }
    }

    /// Sets the type name function for this builder.
    pub const fn type_name(mut self, type_name: TypeNameFn) -> Self {
        self.type_name = Some(type_name);
        self
    }

    /// Sets the display function for this builder.
    pub const fn display(mut self, display: DisplayFnTyped<T>) -> Self {
        self.display = Some(display);
        self
    }

    /// Sets the display function for this builder if Some.
    pub const fn display_maybe(mut self, display: Option<DisplayFnTyped<T>>) -> Self {
        self.display = display;
        self
    }

    /// Sets the debug function for this builder.
    pub const fn debug(mut self, debug: DebugFnTyped<T>) -> Self {
        self.debug = Some(debug);
        self
    }

    /// Sets the debug function for this builder if Some.
    pub const fn debug_maybe(mut self, debug: Option<DebugFnTyped<T>>) -> Self {
        self.debug = debug;
        self
    }

    /// Sets the default_in_place function for this builder.
    pub const fn default_in_place(mut self, default_in_place: DefaultInPlaceFnTyped<T>) -> Self {
        self.default_in_place = Some(default_in_place);
        self
    }

    /// Sets the default_in_place function for this builder if Some.
    pub const fn default_in_place_maybe(
        mut self,
        default_in_place: Option<DefaultInPlaceFnTyped<T>>,
    ) -> Self {
        self.default_in_place = default_in_place;
        self
    }

    /// Sets the clone_into function for this builder.
    pub const fn clone_into(mut self, clone_into: CloneIntoFnTyped<T>) -> Self {
        self.clone_into = Some(clone_into);
        self
    }

    /// Sets the clone_into function for this builder if Some.
    pub const fn clone_into_maybe(mut self, clone_into: Option<CloneIntoFnTyped<T>>) -> Self {
        self.clone_into = clone_into;
        self
    }

    /// Sets the marker traits for this builder.
    pub const fn marker_traits(mut self, marker_traits: MarkerTraits) -> Self {
        self.marker_traits = marker_traits;
        self
    }

    /// Sets the eq function for this builder.
    pub const fn eq(mut self, eq: PartialEqFnTyped<T>) -> Self {
        self.eq = Some(eq);
        self
    }

    /// Sets the eq function for this builder if Some.
    pub const fn eq_maybe(mut self, eq: Option<PartialEqFnTyped<T>>) -> Self {
        self.eq = eq;
        self
    }

    /// Sets the partial_ord function for this builder.
    pub const fn partial_ord(mut self, partial_ord: PartialOrdFnTyped<T>) -> Self {
        self.partial_ord = Some(partial_ord);
        self
    }

    /// Sets the partial_ord function for this builder if Some.
    pub const fn partial_ord_maybe(mut self, partial_ord: Option<PartialOrdFnTyped<T>>) -> Self {
        self.partial_ord = partial_ord;
        self
    }

    /// Sets the ord function for this builder.
    pub const fn ord(mut self, ord: CmpFnTyped<T>) -> Self {
        self.ord = Some(ord);
        self
    }

    /// Sets the ord function for this builder if Some.
    pub const fn ord_maybe(mut self, ord: Option<CmpFnTyped<T>>) -> Self {
        self.ord = ord;
        self
    }

    /// Sets the hash function for this builder.
    pub const fn hash(mut self, hash: HashFnTyped<T>) -> Self {
        self.hash = Some(hash);
        self
    }

    /// Sets the hash function for this builder if Some.
    pub const fn hash_maybe(mut self, hash: Option<HashFnTyped<T>>) -> Self {
        self.hash = hash;
        self
    }

    /// Overwrites the drop_in_place function for this builder.
    ///
    /// This is usually not necessary, the builder builder will default this to the appropriate type.
    pub const fn drop_in_place(mut self, drop_in_place: DropInPlaceFn) -> Self {
        self.drop_in_place = Some(drop_in_place);
        self
    }

    /// Sets the invariants function for this builder.
    pub const fn invariants(mut self, invariants: InvariantsFnTyped<T>) -> Self {
        self.invariants = Some(invariants);
        self
    }

    /// Sets the parse function for this builder.
    pub const fn parse(mut self, parse: ParseFnTyped<T>) -> Self {
        self.parse = Some(parse);
        self
    }

    /// Sets the parse function for this builder if Some.
    pub const fn parse_maybe(mut self, parse: Option<ParseFnTyped<T>>) -> Self {
        self.parse = parse;
        self
    }

    /// Sets the try_from function for this builder.
    pub const fn try_from(mut self, try_from: TryFromFnTyped<T>) -> Self {
        self.try_from = Some(try_from);
        self
    }

    /// Sets the try_into_inner function for this builder.
    pub const fn try_into_inner(mut self, try_into_inner: TryIntoInnerFnTyped<T>) -> Self {
        self.try_into_inner = Some(try_into_inner);
        self
    }

    /// Sets the borrow_inner function for this builder.
    pub const fn try_borrow_inner(mut self, try_borrow_inner: TryBorrowInnerFnTyped<T>) -> Self {
        self.try_borrow_inner = Some(try_borrow_inner);
        self
    }

    /// Builds the [`ValueVTable`] from the current state of the builder.
    pub const fn build(self) -> ValueVTable {
        ValueVTable {
            type_name: self.type_name.unwrap(),
            marker_traits: self.marker_traits,
            invariants: unsafe {
                mem::transmute::<Option<InvariantsFnTyped<T>>, Option<InvariantsFn>>(
                    self.invariants,
                )
            },
            display: unsafe {
                mem::transmute::<Option<DisplayFnTyped<T>>, Option<DisplayFn>>(self.display)
            },
            debug: unsafe {
                mem::transmute::<Option<DebugFnTyped<T>>, Option<DebugFn>>(self.debug)
            },
            default_in_place: unsafe {
                mem::transmute::<Option<DefaultInPlaceFnTyped<T>>, Option<DefaultInPlaceFn>>(
                    self.default_in_place,
                )
            },
            clone_into: unsafe {
                mem::transmute::<Option<CloneIntoFnTyped<T>>, Option<CloneIntoFn>>(self.clone_into)
            },
            eq: unsafe {
                mem::transmute::<Option<PartialEqFnTyped<T>>, Option<PartialEqFn>>(self.eq)
            },
            partial_ord: unsafe {
                mem::transmute::<Option<PartialOrdFnTyped<T>>, Option<PartialOrdFn>>(
                    self.partial_ord,
                )
            },
            ord: unsafe { mem::transmute::<Option<CmpFnTyped<T>>, Option<CmpFn>>(self.ord) },
            hash: unsafe { mem::transmute::<Option<HashFnTyped<T>>, Option<HashFn>>(self.hash) },
            parse: unsafe {
                mem::transmute::<Option<ParseFnTyped<T>>, Option<ParseFn>>(self.parse)
            },
            try_from: unsafe {
                mem::transmute::<Option<TryFromFnTyped<T>>, Option<TryFromFn>>(self.try_from)
            },
            try_into_inner: unsafe {
                mem::transmute::<Option<TryIntoInnerFnTyped<T>>, Option<TryIntoInnerFn>>(
                    self.try_into_inner,
                )
            },
            try_borrow_inner: unsafe {
                mem::transmute::<Option<TryBorrowInnerFnTyped<T>>, Option<TryBorrowInnerFn>>(
                    self.try_borrow_inner,
                )
            },
            drop_in_place: if let Some(drop_in_place) = self.drop_in_place {
                Some(drop_in_place)
            } else if mem::needs_drop::<T>() {
                Some(|value| unsafe { value.drop_in_place::<T>() })
            } else {
                None
            },
        }
    }
}
