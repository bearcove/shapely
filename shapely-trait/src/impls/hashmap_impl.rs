use std::{
    alloc::Layout,
    collections::{HashMap, VecDeque},
    hash::{Hash, RandomState},
};

use shapely_opaque::OpaqueConst;

use crate::{
    Def, MapDef, MapIterVTable, MapVTable, MarkerTraits, ScalarDef, Shape, Shapely, ValueVTable,
    value_vtable,
};

struct HashMapIterator<'mem, K> {
    map: OpaqueConst<'mem>,
    keys: VecDeque<&'mem K>,
}

unsafe impl<K, V, S> Shapely for HashMap<K, V, S>
where
    K: Shapely + std::cmp::Eq + std::hash::Hash + 'static,
    V: Shapely + 'static,
    S: Shapely + Default,
{
    const DUMMY: Self = HashMap::with_hasher(S::DUMMY);
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<HashMap<K, V>>(),
        vtable: &ValueVTable {
            type_name: |f, opts| {
                if let Some(opts) = opts.for_children() {
                    write!(f, "HashMap<")?;
                    (K::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, ", ")?;
                    (V::SHAPE.vtable.type_name)(f, opts)?;
                    write!(f, ">")
                } else {
                    write!(f, "HashMap<⋯>")
                }
            },
            display: None,
            debug: if K::SHAPE.vtable.debug.is_some() && V::SHAPE.vtable.debug.is_some() {
                Some(|value, f| unsafe {
                    let value = value.as_ref::<HashMap<K, V>>();

                    let k_debug = K::SHAPE.vtable.debug.unwrap_unchecked();
                    let v_debug = V::SHAPE.vtable.debug.unwrap_unchecked();

                    write!(f, "{{")?;
                    for (i, (key, val)) in value.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        (k_debug)(OpaqueConst::from_ref(key), f)?;
                        write!(f, ": ")?;
                        (v_debug)(OpaqueConst::from_ref(val), f)?;
                    }
                    write!(f, "}}")
                })
            } else {
                None
            },
            default_in_place: Some(|target| unsafe { target.write(Self::default()) }),
            clone_into: Some(|src, dst| unsafe { dst.write(src.as_ref::<HashMap<K, V>>()) }),
            marker_traits: {
                let mut traits = MarkerTraits::empty();
                if K::SHAPE.vtable.marker_traits.contains(MarkerTraits::SEND)
                    && V::SHAPE.vtable.marker_traits.contains(MarkerTraits::SEND)
                {
                    traits = traits.union(MarkerTraits::SEND);
                }
                if K::SHAPE.vtable.marker_traits.contains(MarkerTraits::SYNC)
                    && V::SHAPE.vtable.marker_traits.contains(MarkerTraits::SYNC)
                {
                    traits = traits.union(MarkerTraits::SYNC);
                }
                if K::SHAPE.vtable.marker_traits.contains(MarkerTraits::EQ)
                    && V::SHAPE.vtable.marker_traits.contains(MarkerTraits::EQ)
                {
                    traits = traits.union(MarkerTraits::EQ);
                }
                traits
            },
            eq: if K::SHAPE.vtable.eq.is_some() && V::SHAPE.vtable.eq.is_some() {
                Some(|a, b| unsafe {
                    let a = a.as_ref::<HashMap<K, V>>();
                    let b = b.as_ref::<HashMap<K, V>>();

                    let k_eq = K::SHAPE.vtable.eq.unwrap_unchecked();
                    let v_eq = V::SHAPE.vtable.eq.unwrap_unchecked();

                    a.len() == b.len()
                        && a.iter().all(|(key_a, val_a)| {
                            b.get(key_a).is_some_and(|val_b| {
                                (k_eq)(OpaqueConst::from_ref(key_a), OpaqueConst::from_ref(key_a))
                                    && (v_eq)(
                                        OpaqueConst::from_ref(val_a),
                                        OpaqueConst::from_ref(val_b),
                                    )
                            })
                        })
                })
            } else {
                None
            },
            ord: None,
            partial_ord: None,
            hash: if K::SHAPE.vtable.hash.is_some() && V::SHAPE.vtable.hash.is_some() {
                Some(|value, hasher_this, hasher_write_fn| unsafe {
                    use crate::HasherProxy;
                    let map = value.as_ref::<HashMap<K, V>>();

                    let k_hash = K::SHAPE.vtable.hash.unwrap_unchecked();
                    let v_hash = V::SHAPE.vtable.hash.unwrap_unchecked();
                    let mut hasher = HasherProxy::new(hasher_this, hasher_write_fn);

                    // Hash length and entries
                    map.len().hash(&mut hasher);
                    for (k, v) in map {
                        (k_hash)(OpaqueConst::from_ref(k), hasher_this, hasher_write_fn);
                        (v_hash)(OpaqueConst::from_ref(v), hasher_this, hasher_write_fn);
                    }
                })
            } else {
                None
            },
            drop_in_place: Some(|value| unsafe {
                std::ptr::drop_in_place(value.as_mut_ptr::<HashMap<K, V>>());
            }),
            parse: None,
            try_from: None,
        },
        def: Def::Map(MapDef {
            k: K::SHAPE,
            v: V::SHAPE,
            vtable: &MapVTable {
                init_in_place_with_capacity_fn: |uninit, capacity| unsafe {
                    Ok(uninit.write(Self::with_capacity_and_hasher(capacity, S::default())))
                },
                insert_fn: |ptr, key, value| unsafe {
                    let map = ptr.as_mut_ptr::<HashMap<K, V>>();
                    let key = key.read::<K>();
                    let value = value.read::<V>();
                    map.insert(key, value);
                },
                len_fn: |ptr| unsafe {
                    let map = ptr.as_ref::<HashMap<K, V>>();
                    map.len()
                },
                contains_key_fn: |ptr, key| unsafe {
                    let map = ptr.as_ref::<HashMap<K, V>>();
                    map.contains_key(key.as_ref())
                },
                get_value_ptr_fn: |ptr, key| unsafe {
                    let map = ptr.as_ref::<HashMap<K, V>>();
                    map.get(key.as_ref())
                        .map(|v| OpaqueConst::new_unchecked(v as *const _))
                },
                iter_fn: |ptr| unsafe {
                    let map = ptr.as_ref::<HashMap<K, V>>();
                    let keys: VecDeque<&K> = map.keys().collect();
                    let iter_state = Box::new(HashMapIterator { map: ptr, keys });
                    OpaqueConst::new_unchecked(Box::into_raw(iter_state) as *mut u8)
                },
                iter_vtable_fn: MapIterVTable {
                    next: |iter_ptr| unsafe {
                        let state = iter_ptr.as_mut_ptr::<HashMapIterator<'_, K>>();
                        let map = state.map.as_ref::<HashMap<K, V>>();

                        while let Some(key) = state.keys.pop_front() {
                            if let Some(value) = map.get(key) {
                                return Some((
                                    OpaqueConst::new_unchecked(key as *const K),
                                    OpaqueConst::new_unchecked(value as *const V),
                                ));
                            }
                        }

                        None
                    },
                    dealloc: |iter_ptr| unsafe {
                        drop(Box::from_raw(iter_ptr.as_ptr::<HashMapIterator<'_, K>>()
                            as *mut HashMapIterator<'_, K>));
                    },
                },
            },
        }),
    };
}

#[allow(dead_code)]
struct RandomStateInnards {
    k0: u64,
    k1: u64,
}

unsafe impl Shapely for RandomState {
    const DUMMY: Self = unsafe { std::mem::transmute(RandomStateInnards { k0: 0, k1: 0 }) };
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Self>(),
            def: Def::Scalar(ScalarDef::of::<Self>()),
            vtable: value_vtable!((), |f, _opts| write!(f, "RandomState")),
        }
    };
}
