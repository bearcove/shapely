use shapely_opaque::{Opaque, OpaqueConst, OpaqueUninit};

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
/// `key` and `value` are moved out of (with [`std::ptr::read`]) — they should be deallocated
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
pub type MapIterFn = for<'map> unsafe fn(map: OpaqueConst<'map>) -> OpaqueConst<'map>;

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
pub struct MapIterVTable {
    /// cf. [`MapIterNextFn`]
    pub next: MapIterNextFn,

    /// cf. [`MapIterDeallocFn`]
    pub dealloc: MapIterDeallocFn,
}

/// Virtual table for a Map<K, V>
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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
    pub iter_vtable_fn: MapIterVTable,
}
