use std::alloc::Layout;

use shapely_types::Characteristic;

use crate::{
    Def, Field, FieldFlags, MarkerTraits, OpaqueConst, Shape, Shapely, StructDef, TypeNameOpts,
    ValueVTable,
};

unsafe impl<T0, T1> Shapely for (T0, T1)
where
    T0: Shapely,
    T1: Shapely,
{
    const DUMMY: Self = (T0::DUMMY, T1::DUMMY);
    const SHAPE: &'static Shape = &const {
        use std::fmt;

        fn type_name<T0, T1>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
        where
            T0: Shapely,
            T1: Shapely,
        {
            if let Some(opts) = opts.for_children() {
                write!(f, "(")?;
                (T0::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ", ")?;
                (T1::SHAPE.vtable.type_name)(f, opts)?;
                write!(f, ")")
            } else {
                write!(f, "(⋯)")
            }
        }

        macro_rules! field {
            ($idx:tt, $ty:ty) => {
                Field {
                    name: stringify!($idx),
                    shape: <$ty>::SHAPE,
                    offset: std::mem::offset_of!((T0, T1), $idx),
                    flags: FieldFlags::EMPTY,
                }
            };
        }

        Shape {
            layout: Layout::new::<(T0, T1)>(),
            vtable: &ValueVTable {
                type_name: type_name::<T0, T1> as _,
                display: None,
                debug: const {
                    if Characteristic::Debug.all(&[T0::SHAPE, T1::SHAPE]) {
                        Some(|value, f| {
                            let value = unsafe { value.as_ref::<(T0, T1)>() };
                            write!(f, "(")?;
                            // First element
                            unsafe {
                                (T0::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.0),
                                    f,
                                )?;
                            }
                            write!(f, ", ")?;
                            // Second element
                            unsafe {
                                (T1::SHAPE.vtable.debug.unwrap_unchecked())(
                                    OpaqueConst::from_ref(&value.1),
                                    f,
                                )?;
                            }
                            write!(f, ")")
                        })
                    } else {
                        None
                    }
                },
                eq: if T0::SHAPE.vtable.eq.is_some() && T1::SHAPE.vtable.eq.is_some() {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1)>() };
                        let b = unsafe { b.as_ref::<(T0, T1)>() };

                        // Compare first elements
                        if !unsafe {
                            (T0::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        } {
                            return false;
                        }

                        // Compare second elements
                        unsafe {
                            (T1::SHAPE.vtable.eq.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        }
                    })
                } else {
                    None
                },
                ord: None,
                partial_ord: if T0::SHAPE.vtable.partial_ord.is_some()
                    && T1::SHAPE.vtable.partial_ord.is_some()
                {
                    Some(|a, b| {
                        let a = unsafe { a.as_ref::<(T0, T1)>() };
                        let b = unsafe { b.as_ref::<(T0, T1)>() };

                        // Compare first elements
                        let ord0 = unsafe {
                            (T0::SHAPE.vtable.partial_ord.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.0),
                                OpaqueConst::from_ref(&b.0),
                            )
                        };

                        match ord0 {
                            Some(std::cmp::Ordering::Equal) => {}
                            Some(order) => return Some(order),
                            None => return None,
                        }

                        // If first elements are equal, compare second elements
                        unsafe {
                            (T1::SHAPE.vtable.partial_ord.unwrap_unchecked())(
                                OpaqueConst::from_ref(&a.1),
                                OpaqueConst::from_ref(&b.1),
                            )
                        }
                    })
                } else {
                    None
                },
                hash: if T0::SHAPE.vtable.hash.is_some() && T1::SHAPE.vtable.hash.is_some() {
                    Some(|value, state, hasher| {
                        let value = unsafe { value.as_ref::<(T0, T1)>() };

                        // Hash first element
                        unsafe {
                            (T0::SHAPE.vtable.hash.unwrap_unchecked())(
                                OpaqueConst::from_ref(&value.0),
                                state,
                                hasher,
                            )
                        };

                        // Hash second element
                        unsafe {
                            (T1::SHAPE.vtable.hash.unwrap_unchecked())(
                                OpaqueConst::from_ref(&value.1),
                                state,
                                hasher,
                            )
                        };
                    })
                } else {
                    None
                },
                drop_in_place: Some(|ptr| {
                    unsafe { ptr.drop_in_place::<(T0, T1)>() };
                }),
                parse: None,
                try_from: None,
                default_in_place: Some(|ptr| unsafe { ptr.write((T0::DUMMY, T1::DUMMY)) }),
                clone_into: if T0::SHAPE.vtable.clone_into.is_some()
                    && T1::SHAPE.vtable.clone_into.is_some()
                {
                    Some(|src, dst| unsafe {
                        let src_tuple = src.as_ref::<(T0, T1)>();

                        let refs = (
                            OpaqueConst::from_ref(&src_tuple.0),
                            OpaqueConst::from_ref(&src_tuple.1),
                        );

                        let clone_into = (
                            T0::SHAPE.vtable.clone_into.unwrap_unchecked(),
                            T1::SHAPE.vtable.clone_into.unwrap_unchecked(),
                        );

                        (clone_into.0)(refs.0, dst.field_uninit(0));
                        (clone_into.1)(refs.1, dst.field_uninit(1));

                        dst.assume_init()
                    })
                } else {
                    None
                },
                marker_traits: {
                    let mut traits = MarkerTraits::empty();

                    // SEND
                    if T0::SHAPE.vtable.marker_traits.contains(MarkerTraits::SEND)
                        && T1::SHAPE.vtable.marker_traits.contains(MarkerTraits::SEND)
                    {
                        traits = traits.union(MarkerTraits::SEND);
                    }

                    // SYNC
                    if T0::SHAPE.vtable.marker_traits.contains(MarkerTraits::SYNC)
                        && T1::SHAPE.vtable.marker_traits.contains(MarkerTraits::SYNC)
                    {
                        traits = traits.union(MarkerTraits::SYNC);
                    }

                    // EQ
                    if T0::SHAPE.vtable.marker_traits.contains(MarkerTraits::EQ)
                        && T1::SHAPE.vtable.marker_traits.contains(MarkerTraits::EQ)
                    {
                        traits = traits.union(MarkerTraits::EQ);
                    }

                    traits
                },
            },
            def: Def::Tuple(StructDef {
                fields: &const { [field!(0, T0), field!(1, T1)] },
            }),
        }
    };
}

