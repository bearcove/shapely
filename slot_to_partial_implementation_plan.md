# Implementation Plan: Slot-to-Partial Conversion Mechanism

## Problem Statement

The current JSON deserializer in the Shapely project has an issue where it doesn't properly track parent-child relationships and key mappings. The `key` variable in the `StructField` state is unused, and there's no mechanism to connect the deserialized field value back to the parent structure.

The solution is to implement a mechanism to convert a Slot into a Partial, which would allow for a more direct deserialization workflow:
1. Obtain a Slot for a struct field using `slot_by_name`
2. Convert the Slot to a Partial
3. Deserialize data directly into that Partial
4. The Partial would automatically update the parent structure when built

## Implementation Details

### 1. Modify the Origin Enum and Partial Struct

```rust
pub enum Origin<'s> {
    Owned {
        slot: Option<Slot<'s>>,  // New field to track the source slot
    },
    Borrowed {
        slot: Option<Slot<'s>>,  // New field to track the source slot
    },
}

pub struct Partial<'s> {
    // Change all fields to private (not pub(crate))
    addr: NonNull<u8>,
    origin: Origin<'s>,
    init_set: InitSet64,
    shape: ShapeDesc,
}
```

This change:
- Renames `HeapAllocated` to `Owned` for clarity
- Adds an optional `slot` field to both variants to track the source slot
- Removes the unused `parent` and `init_mark` fields from the `Borrowed` variant
- Makes all fields in `Partial` private to prevent them from being changed after they've been built

### 2. Add Mark as Initialized Method to Slot

```rust
impl<'s> Slot<'s> {
    /// Mark this slot as initialized without filling it with a value
    /// This is used when a Partial is built directly into the slot's memory location
    pub fn mark_as_initialized(&self) {
        match &self.dest {
            Destination::Ptr { init_mark, .. } => {
                init_mark.set();
            },
            Destination::HashMap { .. } => {
                // For HashMap slots, marking as initialized doesn't make sense
                // without actually inserting a value
                panic!("Cannot mark a HashMap slot as initialized without a value");
            }
        }
    }
}
```

This new method allows marking a slot as initialized without filling it with a value, which is needed when a Partial is built directly into the slot's memory location.

### 3. Implement Slot-to-Partial Conversion Method

```rust
impl<'s> Slot<'s> {
    /// Convert this slot into a Partial that writes directly to the slot's memory location
    pub fn to_partial(&self) -> Partial<'s> {
        match &self.dest {
            Destination::Ptr { ptr, init_mark } => {
                // For struct fields, create a borrowed Partial
                // First, uninitialize the field if it was already initialized
                if init_mark.get() {
                    if let Some(drop_fn) = self.shape.get().drop_in_place {
                        unsafe {
                            drop_fn(ptr.as_ptr());
                        }
                    }
                    // Reset the init mark
                    init_mark.unset();
                }
                
                Partial {
                    addr: *ptr,
                    origin: Origin::Borrowed {
                        slot: Some(self.clone()),
                    },
                    init_set: Default::default(),
                    shape: self.shape,
                }
            },
            Destination::HashMap { map, key } => {
                // For HashMap entries, we need to allocate a new Partial
                // and ensure it's properly inserted into the map when built
                let mut partial = Partial::alloc(self.shape);
                
                // Update the origin to include the slot
                match &mut partial.origin {
                    Origin::Owned { slot } => {
                        *slot = Some(self.clone());
                    },
                    _ => unreachable!("Partial::alloc always creates Owned origin"),
                }
                
                partial
            }
        }
    }
}
```

This method converts a Slot into a Partial that writes directly to the slot's memory location. It handles struct fields and HashMap entries differently:

- For struct fields (Destination::Ptr):
  - Uninitialize the field if it was already initialized
  - Create a borrowed Partial that writes directly to the slot's memory location
  - Set the origin to Borrowed with a reference to the slot

- For HashMap entries (Destination::HashMap):
  - Create a new owned Partial
  - Update the origin to include a reference to the slot
  - When the Partial is built, it will call `slot.fill_from_partial`

### 4. Update the Build Methods for Partial

```rust
impl Partial<'_> {
    pub fn build<T: Shapely>(self) -> T {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        // Match on origin to handle all cases consistently
        match &self.origin {
            Origin::Borrowed { .. } => {
                panic!("Cannot call build() on a Borrowed Partial. Use build_in_place() instead.");
            },
            Origin::Owned { slot: Some(_) } => {
                panic!("Cannot call build() on a Partial with an associated slot. Use build_in_place() instead.");
            },
            Origin::Owned { slot: None } => {
                // This is the only valid case - owned without a slot
                let result = unsafe {
                    let ptr = self.addr.as_ptr() as *const T;
                    std::ptr::read(ptr)
                };
                
                self.deallocate();
                std::mem::forget(self);
                result
            }
        }
    }

    pub fn build_boxed<T: Shapely>(self) -> Box<T> {
        self.assert_all_fields_initialized();
        self.assert_matching_shape::<T>();

        // Match on origin to handle all cases consistently
        match &self.origin {
            Origin::Borrowed { .. } => {
                panic!("Cannot call build_boxed() on a Borrowed Partial. Use build_in_place() instead.");
            },
            Origin::Owned { slot: Some(_) } => {
                panic!("Cannot call build_boxed() on a Partial with an associated slot. Use build_in_place() instead.");
            },
            Origin::Owned { slot: None } => {
                // This is the only valid case - owned without a slot
                let boxed = unsafe { Box::from_raw(self.addr.as_ptr() as *mut T) };
                std::mem::forget(self);
                boxed
            }
        }
    }

    pub fn build_in_place(mut self) {
        self.assert_all_fields_initialized();

        match &self.origin {
            Origin::Borrowed { slot } => {
                // If this Partial was created from a Slot, mark it as initialized
                if let Some(slot) = slot {
                    // For Borrowed Partials, we're building directly into the slot's memory location
                    slot.mark_as_initialized();
                }
            },
            Origin::Owned { slot } => {
                // If this Partial was created from a Slot, use fill_from_partial
                if let Some(slot) = slot {
                    slot.fill_from_partial(self.clone());
                } else {
                    panic!("Cannot build in place for owned Partial without a slot");
                }
            },
        }

        // Prevent field drops when the Partial is dropped
        std::mem::forget(self);
    }
}
```

These changes:
- Refactor the build methods to have a consistent structure
- Make `build()` and `build_boxed()` panic when:
  - The origin is `Borrowed`
  - The origin is `Owned` with a slot
- Only allow `build()` and `build_boxed()` when the origin is `Owned` without a slot
- Update `build_in_place()` to handle both `Borrowed` and `Owned` origins with slots

### 5. Update the Drop Implementation for Partial

```rust
impl Drop for Partial<'_> {
    fn drop(&mut self) {
        // Existing field dropping logic remains unchanged
        match self.shape.get().innards {
            crate::Innards::Struct { fields } => {
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(i, field)| {
                        if self.init_set.is_set(i) {
                            Some((field, field.shape.get().drop_in_place?))
                        } else {
                            None
                        }
                    })
                    .for_each(|(field, drop_fn)| {
                        unsafe {
                            drop_fn(self.addr.byte_add(field.offset).as_ptr());
                        }
                    })
            }
            crate::Innards::Scalar(_) => {
                if self.init_set.is_set(0) {
                    if let Some(drop_fn) = self.shape.get().drop_in_place {
                        unsafe {
                            drop_fn(self.addr.as_ptr());
                        }
                    }
                }
            }
            _ => {}
        }

        // Only deallocate if we own the memory
        match &self.origin {
            Origin::Owned { .. } => self.deallocate(),
            Origin::Borrowed { .. } => {
                // For borrowed memory, we don't deallocate
            },
        }
    }
}
```

The `Drop` implementation still handles dropping initialized fields for all variants, but only deallocates for `Owned` origins.

### 6. Update the JSON Deserializer

```rust
DeserializeState::StructContinue { mut partial, first } => {
    let key = if first {
        parser.expect_object_start()?
    } else {
        parser.parse_object_key()?
    };

    if let Some(key) = key {
        trace!("Processing struct key: \x1b[1;33m{key}\x1b[0m");
        let slot = match partial.slot_by_name(&key) {
            Ok(slot) => slot,
            Err(_) => {
                return Err(parser.make_error(JsonParseErrorKind::UnknownField(key)));
            }
        };

        // Convert the slot to a partial
        let partial_field = slot.to_partial();

        // Push the field state which will deserialize the value
        stack.push(DeserializeState::StructField {
            parent_partial: partial,
            key,  // We still keep the key for debugging/tracing
            partial_field,
        });
    } else {
        // No more fields, we're done with this struct
        trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");
    }
}
```

The JSON deserializer is updated to use the new Slot-to-Partial conversion instead of allocating a new Partial.

### 7. Comprehensive Test Suite

We'll create a comprehensive test suite to verify the behavior of the Slot-to-Partial conversion, focusing on:

1. Proper initialization/uninitialization of fields
2. Correct handling of both struct fields and HashMap entries
3. Memory safety (to be verified with MIRI)
4. Edge cases like nested structures and partial initialization

The tests will include:
- Converting a Slot to a Partial and then building it in place
- Dropping a partially initialized Partial created from a Slot
- Verifying that the parent structure is correctly updated
- Testing with complex nested structures

## Implementation Flow

1. First, implement the changes to the `Origin` enum and `Partial` struct
2. Add the `mark_as_initialized` method to `Slot`
3. Implement the `to_partial` method on `Slot`
4. Update the `Drop`, `build`, `build_in_place`, and `build_boxed` implementations for `Partial`
5. Update the JSON deserializer to use the new functionality
6. Create and run the comprehensive test suite, including MIRI tests

## Potential Challenges

1. **HashMap Handling**: The implementation for HashMap slots might be more complex, as we need to ensure that the deserialized value is properly inserted into the HashMap.

2. **Lifetime Management**: We need to carefully manage lifetimes to ensure that the Slot remains valid for the lifetime of the Partial.

3. **Memory Safety**: We need to ensure that the memory is properly managed, especially when uninitializing and reinitializing fields.

4. **Error Handling**: We need to ensure that errors during deserialization are properly propagated and don't leave the data structure in an inconsistent state.