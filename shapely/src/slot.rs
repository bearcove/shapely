use std::collections::HashMap;
use std::ptr::NonNull;

use crate::{InitFieldSlot, ShapeDesc, Shapely};

enum Destination {
    /// Writes directly to an (uninitialized) struct field
    StructField { field_addr: NonNull<()> },

    /// Inserts into a HashMap
    HashMap { map: NonNull<()>, key: String },
}

/// Allows writing into a struct field or inserting into a hash map.
pub struct Slot<'s> {
    /// where to write the value
    dest: Destination,

    /// shape of the field / hashmap value we're writing
    field_shape: ShapeDesc,

    /// lifetime marker
    init_field_slot: InitFieldSlot<'s>,
}

impl<'s> Slot<'s> {
    #[inline(always)]
    pub fn for_struct_field(
        field_addr: NonNull<()>,
        field_shape: ShapeDesc,
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
        map: NonNull<()>,
        field_shape: ShapeDesc,
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
        if self.field_shape != T::shape_desc() {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.field_shape.get(),
                T::shape()
            );
        }

        unsafe {
            match self.dest {
                Destination::StructField { field_addr } => {
                    let field_addr = field_addr.as_ptr();
                    if self.init_field_slot.is_init() {
                        std::ptr::drop_in_place(field_addr as *mut T);
                    }
                    std::ptr::write(field_addr as *mut T, value);
                }
                Destination::HashMap { map, key } => {
                    let map = &mut *(map.as_ptr() as *mut HashMap<String, T>);
                    map.insert(key, value);
                }
            }
        }
        self.init_field_slot.mark_as_init();
    }

    pub fn fill_from_partial(mut self, partial: crate::Partial<'_>) {
        if self.field_shape != partial.shape_desc() {
            panic!(
                "Attempted to fill a field with an incompatible shape.\n\
                Expected shape: {:?}\n\
                Actual shape: {:?}\n\
                This is undefined behavior and we're refusing to proceed.",
                self.field_shape.get(),
                partial.shape_desc().get()
            );
        }

        partial.check_initialization();

        unsafe {
            match self.dest {
                Destination::StructField { field_addr } => {
                    let field_addr = field_addr.as_ptr();
                    if self.init_field_slot.is_init() {
                        if let Some(drop_fn) = self.field_shape.get().drop_in_place {
                            drop_fn(field_addr);
                        }
                    }
                    std::ptr::copy_nonoverlapping(
                        partial.addr.as_ptr(),
                        field_addr,
                        self.field_shape.get().layout.size(),
                    );
                }
                Destination::HashMap { .. } => {
                    // TODO: Implement for HashMap
                    // I guess we need another field in the vtable?
                    panic!("fill_from_partial not implemented for HashMap");
                }
            }
        }
        self.init_field_slot.mark_as_init();
        std::mem::forget(partial);
    }
}
