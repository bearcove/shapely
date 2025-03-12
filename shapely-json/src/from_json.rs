use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use self_cell::{self_cell, MutBorrow};
use shapely::{error, trace, warn, Partial};

pub fn from_json<'input, 's>(
    partial: Partial<'s>,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    use shapely::{Innards, Scalar};

    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    self_cell!(
        struct FieldOf<'s> {
            owner: MutBorrow<Partial<'s>>,

            #[covariant]
            dependent: Partial,
        }
    );

    // Define our state machine states
    enum DeserializeState<'s> {
        Value { partial: Partial<'s> },
        Struct { partial: Partial<'s>, first: bool },
        FieldOf(FieldOf<'s>),
    }

    // Create our stack and initialize with the root value
    let mut stack = Vec::new();
    // Note: Partial is NOT clone and should never be cloned
    // We should never use std::mem::replace or std::mem::take
    stack.push(DeserializeState::Value { partial });

    while let Some(state) = stack.pop() {
        match state {
            DeserializeState::Value { mut partial } => {
                let shape_desc = partial.shape();
                let shape = shape_desc.get();
                trace!("Deserializing value with shape:\n{shape:?}");

                match &shape.innards {
                    Innards::Scalar(scalar) => {
                        let slot = partial.scalar_slot().expect("Scalar slot");
                        trace!(
                            "Deserializing \x1b[1;36mscalar\x1b[0m, \x1b[1;35m{scalar:?}\x1b[0m"
                        );

                        match scalar {
                            Scalar::String => slot.fill(parser.parse_string()?),
                            Scalar::U64 => slot.fill(parser.parse_u64()?),
                            // Add other scalar types as needed
                            _ => {
                                warn!("Unsupported scalar type: {scalar:?}");
                                return Err(parser.make_error(JsonParseErrorKind::Custom(
                                    format!("Unsupported scalar type: {scalar:?}"),
                                )));
                            }
                        }
                        partial.build_in_place();
                    }
                    Innards::Struct { .. } => {
                        trace!("Deserializing \x1b[1;36mstruct\x1b[0m");
                        // Push the struct state on the stack to continue after we process the fields
                        stack.push(DeserializeState::Struct {
                            partial,
                            first: true,
                        });
                    }
                    // Add support for other shapes (Array, Transparent) as needed
                    _ => {
                        error!("Unsupported shape: {shape}");
                        return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Unsupported shape: {:?}",
                            shape.innards
                        ))));
                    }
                }
            }
            DeserializeState::Struct { partial, first } => {
                let key = if first {
                    parser.expect_object_start()?
                } else {
                    parser.parse_object_key()?
                };

                if let Some(key) = key {
                    trace!("Processing struct key: \x1b[1;33m{key}\x1b[0m");
                    stack.push(DeserializeState::FieldOf(FieldOf::new(
                        MutBorrow::new(partial),
                        |p| p.borrow_mut().slot_by_name(&key).unwrap().into_partial(),
                    )));
                } else {
                    // No more fields, we're done with this struct
                    trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");

                    // TODO: this would be a good place to decide what to do about unset fields? Is this
                    // where we finally get to use `set_default`?

                    partial.build_in_place();
                }
            }
            DeserializeState::FieldOf(fo) => {
                // mh
            }
        }
    }
    Ok(())
}
