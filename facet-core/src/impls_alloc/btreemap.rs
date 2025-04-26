use core::write;

use alloc::{
    boxed::Box,
    collections::{BTreeMap, VecDeque},
};

use crate::{
    Def, Facet, MapDef, MapIterVTable, MapVTable, MarkerTraits, PtrConst, PtrMut, Shape,
    ValueVTable,
};

struct BTreeMapIterator<'mem, K> {
    map: PtrConst<'mem>,
    keys: VecDeque<&'mem K>,
}

unsafe impl<'a, K, V> Facet<'a> for BTreeMap<K, V>
where
    K: Facet<'a> + core::cmp::Eq + core::cmp::Ord,
    V: Facet<'a>,
{
    const SHAPE: &'static crate::Shape = &const {
        Shape::builder_for_sized::<Self>()
            .type_params(&[
                crate::TypeParam {
                    name: "K",
                    shape: || K::SHAPE,
                },
                crate::TypeParam {
                    name: "V",
                    shape: || V::SHAPE,
                },
            ])
            .vtable(
                &const {
                    let mut builder = ValueVTable::builder::<Self>()
                        .marker_traits({
                            let arg_dependent_traits = MarkerTraits::SEND
                                .union(MarkerTraits::SYNC)
                                .union(MarkerTraits::EQ);
                            arg_dependent_traits
                                .intersection(V::SHAPE.vtable.marker_traits)
                                .intersection(K::SHAPE.vtable.marker_traits)
                                // only depends on `A` which we are not generic over (yet)
                                .union(MarkerTraits::UNPIN)
                        })
                        .type_name(|f, opts| {
                            if let Some(opts) = opts.for_children() {
                                write!(f, "BTreeMap<")?;
                                (K::SHAPE.vtable.type_name)(f, opts)?;
                                write!(f, ", ")?;
                                (V::SHAPE.vtable.type_name)(f, opts)?;
                                write!(f, ">")
                            } else {
                                write!(f, "BTreeMap<⋯>")
                            }
                        });

                    if K::SHAPE.vtable.debug.is_some() && V::SHAPE.vtable.debug.is_some() {
                        builder = builder.debug(|value, f| unsafe {
                            let value = value.get::<Self>();
                            let k_debug = K::SHAPE.vtable.debug.unwrap_unchecked();
                            let v_debug = V::SHAPE.vtable.debug.unwrap_unchecked();
                            write!(f, "{{")?;
                            for (i, (key, val)) in value.iter().enumerate() {
                                if i > 0 {
                                    write!(f, ", ")?;
                                }
                                (k_debug)(PtrConst::new(key as *const _), f)?;
                                write!(f, ": ")?;
                                (v_debug)(PtrConst::new(val as *const _), f)?;
                            }
                            write!(f, "}}")
                        })
                    }

                    builder =
                        builder.default_in_place(|target| unsafe { target.put(Self::default()) });
                    builder = builder.clone_into(|src, dst| unsafe { dst.put(src.get::<Self>()) });

                    if V::SHAPE.vtable.eq.is_some() {
                        builder = builder.eq(|a, b| unsafe {
                            let a = a.get::<Self>();
                            let b = b.get::<Self>();
                            let v_eq = V::SHAPE.vtable.eq.unwrap_unchecked();
                            a.len() == b.len()
                                && a.iter().all(|(key_a, val_a)| {
                                    b.get(key_a).is_some_and(|val_b| {
                                        (v_eq)(
                                            PtrConst::new(val_a as *const _),
                                            PtrConst::new(val_b as *const _),
                                        )
                                    })
                                })
                        });
                    }

                    if K::SHAPE.vtable.hash.is_some() && V::SHAPE.vtable.hash.is_some() {
                        builder = builder.hash(|value, hasher_this, hasher_write_fn| unsafe {
                            use crate::HasherProxy;
                            use core::hash::Hash;

                            let map = value.get::<Self>();
                            let k_hash = K::SHAPE.vtable.hash.unwrap_unchecked();
                            let v_hash = V::SHAPE.vtable.hash.unwrap_unchecked();
                            let mut hasher = HasherProxy::new(hasher_this, hasher_write_fn);
                            map.len().hash(&mut hasher);
                            for (k, v) in map {
                                (k_hash)(
                                    PtrConst::new(k as *const _),
                                    hasher_this,
                                    hasher_write_fn,
                                );
                                (v_hash)(
                                    PtrConst::new(v as *const _),
                                    hasher_this,
                                    hasher_write_fn,
                                );
                            }
                        });
                    }

                    builder.build()
                },
            )
            .def(Def::Map(
                MapDef::builder()
                    .k(K::SHAPE)
                    .v(V::SHAPE)
                    .vtable(
                        &const {
                            MapVTable::builder()
                                .init_in_place_with_capacity(|uninit, _capacity| unsafe {
                                    uninit.put(Self::new())
                                })
                                .insert(|ptr, key, value| unsafe {
                                    let map = ptr.as_mut::<Self>();
                                    let k = key.read::<K>();
                                    let v = value.read::<V>();
                                    map.insert(k, v);
                                })
                                .len(|ptr| unsafe {
                                    let map = ptr.get::<Self>();
                                    map.len()
                                })
                                .contains_key(|ptr, key| unsafe {
                                    let map = ptr.get::<Self>();
                                    map.contains_key(key.get())
                                })
                                .get_value_ptr(|ptr, key| unsafe {
                                    let map = ptr.get::<Self>();
                                    map.get(key.get()).map(|v| PtrConst::new(v as *const _))
                                })
                                .iter(|ptr| unsafe {
                                    let map = ptr.get::<Self>();
                                    let keys: VecDeque<&K> = map.keys().collect();
                                    let iter_state = Box::new(BTreeMapIterator { map: ptr, keys });
                                    PtrMut::new(Box::into_raw(iter_state) as *mut u8)
                                })
                                .iter_vtable(
                                    MapIterVTable::builder()
                                        .next(|iter_ptr| unsafe {
                                            let state =
                                                iter_ptr.as_mut::<BTreeMapIterator<'_, K>>();
                                            let map = state.map.get::<Self>();
                                            while let Some(key) = state.keys.pop_front() {
                                                if let Some(value) = map.get(key) {
                                                    return Some((
                                                        PtrConst::new(key as *const K),
                                                        PtrConst::new(value as *const V),
                                                    ));
                                                }
                                            }

                                            None
                                        })
                                        .dealloc(|iter_ptr| unsafe {
                                            drop(Box::from_raw(
                                                iter_ptr.as_ptr::<BTreeMapIterator<'_, K>>()
                                                    as *mut BTreeMapIterator<'_, K>,
                                            ))
                                        })
                                        .build(),
                                )
                                .build()
                        },
                    )
                    .build(),
            ))
            .build()
    };
}
