use crate::ptr::{PtrConst, PtrMut, PtrUninit};

use super::Shape;

/// Fields for set types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct SetDef {
    /// vtable for interacting with the set
    pub vtable: &'static SetVTable,
    /// shape of the values in the set
    pub t: fn() -> &'static Shape,
}

impl SetDef {
    /// Returns a builder for SetDef
    pub const fn builder() -> SetDefBuilder {
        SetDefBuilder::new()
    }

    /// Returns the shape of the items in the set
    pub fn t(&self) -> &'static Shape {
        (self.t)()
    }
}

/// Builder for SetDef
pub struct SetDefBuilder {
    vtable: Option<&'static SetVTable>,
    t: Option<fn() -> &'static Shape>,
}

impl SetDefBuilder {
    /// Creates a new SetDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            vtable: None,
            t: None,
        }
    }

    /// Sets the vtable for the SetDef
    pub const fn vtable(mut self, vtable: &'static SetVTable) -> Self {
        self.vtable = Some(vtable);
        self
    }

    /// Sets the value shape for the SetDef
    pub const fn t(mut self, t: fn() -> &'static Shape) -> Self {
        self.t = Some(t);
        self
    }

    /// Builds the SetDef
    pub const fn build(self) -> SetDef {
        SetDef {
            vtable: self.vtable.unwrap(),
            t: self.t.unwrap(),
        }
    }
}

/// Initialize a set in place with a given capacity
///
/// # Safety
///
/// The `set` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
pub type SetInitInPlaceWithCapacityFn =
    for<'mem> unsafe fn(set: PtrUninit<'mem>, capacity: usize) -> PtrMut<'mem>;

/// Insert a value in the set if not already contained, returning true
/// if the value wasn't present before
///
/// # Safety
///
/// The `set` parameter must point to aligned, initialized memory of the correct type.
/// `value` is moved out of (with [`core::ptr::read`]) — it should be deallocated afterwards (e.g.
/// with [`core::mem::forget`]) but NOT dropped.
pub type SetInsertFn =
    for<'set, 'value> unsafe fn(set: PtrMut<'set>, value: PtrMut<'value>) -> bool;

/// Get the number of values in the set
///
/// # Safety
///
/// The `set` parameter must point to aligned, initialized memory of the correct type.
pub type SetLenFn = for<'set> unsafe fn(set: PtrConst<'set>) -> usize;

/// Check if the set contains a value
///
/// # Safety
///
/// The `set` parameter must point to aligned, initialized memory of the correct type.
pub type SetContainsFn =
    for<'set, 'value> unsafe fn(set: PtrConst<'set>, value: PtrConst<'value>) -> bool;

/// Get an iterator over the set
///
/// # Safety
///
/// The `set` parameter must point to aligned, initialized memory of the correct type.
pub type SetIterFn = for<'set> unsafe fn(set: PtrConst<'set>) -> PtrMut<'set>;

/// Get the next value from the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type SetIterNextFn = for<'iter> unsafe fn(iter: PtrMut<'iter>) -> Option<PtrConst<'iter>>;

/// Deallocate the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type SetIterDeallocFn = for<'iter> unsafe fn(iter: PtrMut<'iter>);

/// VTable for an iterator over a set
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub struct SetIterVTable {
    /// cf. [`SetIterNextFn`]
    pub next: SetIterNextFn,

    /// cf. [`SetIterDeallocFn`]
    pub dealloc: SetIterDeallocFn,
}

impl SetIterVTable {
    /// Returns a builder for SetIterVTable
    pub const fn builder() -> SetIterVTableBuilder {
        SetIterVTableBuilder::new()
    }
}

/// Builds a [`SetIterVTable`]
pub struct SetIterVTableBuilder {
    next: Option<SetIterNextFn>,
    dealloc: Option<SetIterDeallocFn>,
}

impl SetIterVTableBuilder {
    /// Creates a new [`SetIterVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            next: None,
            dealloc: None,
        }
    }

    /// Sets the next field
    pub const fn next(mut self, f: SetIterNextFn) -> Self {
        self.next = Some(f);
        self
    }

    /// Sets the dealloc field
    pub const fn dealloc(mut self, f: SetIterDeallocFn) -> Self {
        self.dealloc = Some(f);
        self
    }

    /// Builds the [`SetIterVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> SetIterVTable {
        SetIterVTable {
            next: self.next.unwrap(),
            dealloc: self.dealloc.unwrap(),
        }
    }
}

/// Virtual table for a `Set<T>`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct SetVTable {
    /// cf. [`SetInitInPlaceWithCapacityFn`]
    pub init_in_place_with_capacity_fn: SetInitInPlaceWithCapacityFn,

    /// cf. [`SetInsertFn`]
    pub insert_fn: SetInsertFn,

    /// cf. [`SetLenFn`]
    pub len_fn: SetLenFn,

    /// cf. [`SetContainsFn`]
    pub contains_fn: SetContainsFn,

    /// cf. [`SetIterFn`]
    pub iter_fn: SetIterFn,

    /// Virtual table for set iterator operations
    pub iter_vtable: SetIterVTable,
}

impl SetVTable {
    /// Returns a builder for SetVTable
    pub const fn builder() -> SetVTableBuilder {
        SetVTableBuilder::new()
    }
}

/// Builds a [`SetVTable`]
pub struct SetVTableBuilder {
    init_in_place_with_capacity_fn: Option<SetInitInPlaceWithCapacityFn>,
    insert_fn: Option<SetInsertFn>,
    len_fn: Option<SetLenFn>,
    contains_fn: Option<SetContainsFn>,
    iter_fn: Option<SetIterFn>,
    iter_vtable: Option<SetIterVTable>,
}

impl SetVTableBuilder {
    /// Creates a new [`SetVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            init_in_place_with_capacity_fn: None,
            insert_fn: None,
            len_fn: None,
            contains_fn: None,
            iter_fn: None,
            iter_vtable: None,
        }
    }

    /// Sets the init_in_place_with_capacity_fn field
    pub const fn init_in_place_with_capacity(mut self, f: SetInitInPlaceWithCapacityFn) -> Self {
        self.init_in_place_with_capacity_fn = Some(f);
        self
    }

    /// Sets the insert_fn field
    pub const fn insert(mut self, f: SetInsertFn) -> Self {
        self.insert_fn = Some(f);
        self
    }

    /// Sets the len_fn field
    pub const fn len(mut self, f: SetLenFn) -> Self {
        self.len_fn = Some(f);
        self
    }

    /// Sets the contains_fn field
    pub const fn contains(mut self, f: SetContainsFn) -> Self {
        self.contains_fn = Some(f);
        self
    }

    /// Sets the iter_fn field
    pub const fn iter(mut self, f: SetIterFn) -> Self {
        self.iter_fn = Some(f);
        self
    }

    /// Sets the iter_vtable field
    pub const fn iter_vtable(mut self, vtable: SetIterVTable) -> Self {
        self.iter_vtable = Some(vtable);
        self
    }

    /// Builds the [`SetVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> SetVTable {
        SetVTable {
            init_in_place_with_capacity_fn: self.init_in_place_with_capacity_fn.unwrap(),
            insert_fn: self.insert_fn.unwrap(),
            len_fn: self.len_fn.unwrap(),
            contains_fn: self.contains_fn.unwrap(),
            iter_fn: self.iter_fn.unwrap(),
            iter_vtable: self.iter_vtable.unwrap(),
        }
    }
}
