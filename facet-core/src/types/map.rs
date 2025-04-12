use crate::opaque::{Opaque, OpaqueConst, OpaqueUninit};

use super::Shape;

/// Fields for map types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct MapDef {
    /// vtable for interacting with the map
    pub vtable: &'static MapVTable,
    /// shape of the keys in the map
    pub k: &'static Shape,
    /// shape of the values in the map
    pub v: &'static Shape,
}

impl MapDef {
    /// Returns a builder for MapDef
    pub const fn builder() -> MapDefBuilder {
        MapDefBuilder::new()
    }
}

/// Builder for MapDef
pub struct MapDefBuilder {
    vtable: Option<&'static MapVTable>,
    k: Option<&'static Shape>,
    v: Option<&'static Shape>,
}

impl MapDefBuilder {
    /// Creates a new MapDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            vtable: None,
            k: None,
            v: None,
        }
    }

    /// Sets the vtable for the MapDef
    pub const fn vtable(mut self, vtable: &'static MapVTable) -> Self {
        self.vtable = Some(vtable);
        self
    }

    /// Sets the key shape for the MapDef
    pub const fn k(mut self, k: &'static Shape) -> Self {
        self.k = Some(k);
        self
    }

    /// Sets the value shape for the MapDef
    pub const fn v(mut self, v: &'static Shape) -> Self {
        self.v = Some(v);
        self
    }

    /// Builds the MapDef
    pub const fn build(self) -> MapDef {
        MapDef {
            vtable: self.vtable.unwrap(),
            k: self.k.unwrap(),
            v: self.v.unwrap(),
        }
    }
}

/// Initialize a map in place with a given capacity
///
/// # Safety
///
/// The `map` parameter must point to uninitialized memory of sufficient size.
/// The function must properly initialize the memory.
pub type MapInitInPlaceWithCapacityFn =
    unsafe fn(map: OpaqueUninit, capacity: usize) -> Result<Opaque, ()>;

/// Insert a key-value pair into the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
/// `key` and `value` are moved out of (with [`core::ptr::read`]) — they should be deallocated
/// afterwards but NOT dropped.
pub type MapInsertFn =
    for<'map, 'key, 'value> unsafe fn(map: Opaque<'map>, key: Opaque<'key>, value: Opaque<'value>);

/// Get the number of entries in the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapLenFn = for<'map> unsafe fn(map: OpaqueConst<'map>) -> usize;

/// Check if the map contains a key
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapContainsKeyFn =
    for<'map, 'key> unsafe fn(map: OpaqueConst<'map>, key: OpaqueConst<'key>) -> bool;

/// Get pointer to a value for a given key, returns None if not found
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapGetValuePtrFn = for<'map, 'key> unsafe fn(
    map: OpaqueConst<'map>,
    key: OpaqueConst<'key>,
) -> Option<OpaqueConst<'map>>;

/// Get an iterator over the map
///
/// # Safety
///
/// The `map` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterFn = for<'map> unsafe fn(map: OpaqueConst<'map>) -> Opaque<'map>;

/// Get the next key-value pair from the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterNextFn =
    for<'iter> unsafe fn(iter: Opaque<'iter>) -> Option<(OpaqueConst<'iter>, OpaqueConst<'iter>)>;

/// Deallocate the iterator
///
/// # Safety
///
/// The `iter` parameter must point to aligned, initialized memory of the correct type.
pub type MapIterDeallocFn = for<'iter> unsafe fn(iter: Opaque<'iter>);

/// VTable for an iterator over a map
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub struct MapIterVTable {
    /// cf. [`MapIterNextFn`]
    pub next: MapIterNextFn,

    /// cf. [`MapIterDeallocFn`]
    pub dealloc: MapIterDeallocFn,
}

impl MapIterVTable {
    /// Returns a builder for MapIterVTable
    pub const fn builder() -> MapIterVTableBuilder {
        MapIterVTableBuilder::new()
    }
}

/// Builds a [`MapIterVTable`]
pub struct MapIterVTableBuilder {
    next: Option<MapIterNextFn>,
    dealloc: Option<MapIterDeallocFn>,
}

impl MapIterVTableBuilder {
    /// Creates a new [`MapIterVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            next: None,
            dealloc: None,
        }
    }

    /// Sets the next field
    pub const fn next(mut self, f: MapIterNextFn) -> Self {
        self.next = Some(f);
        self
    }

    /// Sets the dealloc field
    pub const fn dealloc(mut self, f: MapIterDeallocFn) -> Self {
        self.dealloc = Some(f);
        self
    }

    /// Builds the [`MapIterVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> MapIterVTable {
        MapIterVTable {
            next: self.next.unwrap(),
            dealloc: self.dealloc.unwrap(),
        }
    }
}

/// Virtual table for a Map<K, V>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct MapVTable {
    /// cf. [`MapInitInPlaceWithCapacityFn`]
    pub init_in_place_with_capacity_fn: MapInitInPlaceWithCapacityFn,

    /// cf. [`MapInsertFn`]
    pub insert_fn: MapInsertFn,

    /// cf. [`MapLenFn`]
    pub len_fn: MapLenFn,

    /// cf. [`MapContainsKeyFn`]
    pub contains_key_fn: MapContainsKeyFn,

    /// cf. [`MapGetValuePtrFn`]
    pub get_value_ptr_fn: MapGetValuePtrFn,

    /// cf. [`MapIterFn`]
    pub iter_fn: MapIterFn,

    /// Virtual table for map iterator operations
    pub iter_vtable: MapIterVTable,
}

impl MapVTable {
    /// Returns a builder for MapVTable
    pub const fn builder() -> MapVTableBuilder {
        MapVTableBuilder::new()
    }
}

/// Builds a [`MapVTable`]
pub struct MapVTableBuilder {
    init_in_place_with_capacity_fn: Option<MapInitInPlaceWithCapacityFn>,
    insert_fn: Option<MapInsertFn>,
    len_fn: Option<MapLenFn>,
    contains_key_fn: Option<MapContainsKeyFn>,
    get_value_ptr_fn: Option<MapGetValuePtrFn>,
    iter_fn: Option<MapIterFn>,
    iter_vtable: Option<MapIterVTable>,
}

impl MapVTableBuilder {
    /// Creates a new [`MapVTableBuilder`] with all fields set to `None`.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            init_in_place_with_capacity_fn: None,
            insert_fn: None,
            len_fn: None,
            contains_key_fn: None,
            get_value_ptr_fn: None,
            iter_fn: None,
            iter_vtable: None,
        }
    }

    /// Sets the init_in_place_with_capacity_fn field
    pub const fn init_in_place_with_capacity(mut self, f: MapInitInPlaceWithCapacityFn) -> Self {
        self.init_in_place_with_capacity_fn = Some(f);
        self
    }

    /// Sets the insert_fn field
    pub const fn insert(mut self, f: MapInsertFn) -> Self {
        self.insert_fn = Some(f);
        self
    }

    /// Sets the len_fn field
    pub const fn len(mut self, f: MapLenFn) -> Self {
        self.len_fn = Some(f);
        self
    }

    /// Sets the contains_key_fn field
    pub const fn contains_key(mut self, f: MapContainsKeyFn) -> Self {
        self.contains_key_fn = Some(f);
        self
    }

    /// Sets the get_value_ptr_fn field
    pub const fn get_value_ptr(mut self, f: MapGetValuePtrFn) -> Self {
        self.get_value_ptr_fn = Some(f);
        self
    }

    /// Sets the iter_fn field
    pub const fn iter(mut self, f: MapIterFn) -> Self {
        self.iter_fn = Some(f);
        self
    }

    /// Sets the iter_vtable field
    pub const fn iter_vtable(mut self, vtable: MapIterVTable) -> Self {
        self.iter_vtable = Some(vtable);
        self
    }

    /// Builds the [`MapVTable`] from the current state of the builder.
    ///
    /// # Panics
    ///
    /// This method will panic if any of the required fields are `None`.
    pub const fn build(self) -> MapVTable {
        MapVTable {
            init_in_place_with_capacity_fn: self.init_in_place_with_capacity_fn.unwrap(),
            insert_fn: self.insert_fn.unwrap(),
            len_fn: self.len_fn.unwrap(),
            contains_key_fn: self.contains_key_fn.unwrap(),
            get_value_ptr_fn: self.get_value_ptr_fn.unwrap(),
            iter_fn: self.iter_fn.unwrap(),
            iter_vtable: self.iter_vtable.unwrap(),
        }
    }
}
