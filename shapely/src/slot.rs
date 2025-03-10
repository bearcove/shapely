use std::collections::HashMap;

use crate::{InitFieldSlot, Shape, Shapely};

/// Type alias for user data pointer
type UserData = *mut u8;

/// Type alias for destination pointer
type StructField = *mut u8;

pub enum Destination {
    /// Writes directly to an (uninitialized) struct field
    StructField { field_addr: *mut u8 },

    /// Inserts into a HashMap
    HashMap { map: *mut u8, key: String },
}

/// Allows filling in a field of a struct, or inserting a value into a hashmap while deserializing.
pub struct Slot<'s> {
    /// where to write the value
    dest: Destination,

    /// shape of the field / hashmap value we're writing
    field_shape: fn() -> Shape,

    /// lifetime marker
    init_field_slot: InitFieldSlot<'s>,
}

impl<'s> Slot<'s> {
    #[inline(always)]
    pub fn for_struct_field(
        field_addr: *mut u8,
        field_shape: fn() -> Shape,
        init_field_slot: InitFieldSlot<'s>,
    ) -> Self {
        Self {
            dest: Destination::StructField { field_addr },
            field_shape,
            init_field_slot,
        }
    }

    #[inline(always)]
    pub fn for_hash_map(
        map: *mut u8,
        field_shape: fn() -> Shape,
        key: String,
        init_field_slot: InitFieldSlot<'s>,
    ) -> Self {
        Self {
            dest: Destination::HashMap { map, key },
            field_shape,
            init_field_slot,
        }
    }

    pub fn fill<T: Shapely>(mut self, value: T) {
        let value_shape = T::shape();
        if (self.field_shape)() != value_shape {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.field_shape, value_shape
            );
        }

        unsafe {
            match self.dest {
                Destination::StructField { field_addr } => {
                    std::ptr::write(field_addr as *mut T, value);
                }
                Destination::HashMap { map, key } => {
                    let map = &mut *(map as *mut HashMap<String, T>);
                    map.insert(key, value);
                }
            }
        }
        self.init_field_slot.mark_initialized();
    }
}
