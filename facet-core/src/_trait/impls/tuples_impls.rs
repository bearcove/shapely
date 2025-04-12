// //! GENERATED: DO NOT EDIT — this file is generated from `tuples_impls.rs.j2`
// //! file in the `facet-codegen` crate.
// //!
// //! Edit the template and run `just codegen` to update.

// use core::{alloc::Layout, fmt};

// use crate::{
//     Characteristic, ConstTypeId, Def, Facet, Field, FieldFlags, Fields, MarkerTraits, OpaqueConst,
//     Shape, StructKind, TypeNameOpts, ValueVTable,
// };

// #[inline(always)]
// pub fn write_type_name_list(
//     f: &mut fmt::Formatter<'_>,
//     opts: TypeNameOpts,
//     open: &'static str,
//     delimiter: &'static str,
//     close: &'static str,
//     shapes: &'static [&'static Shape],
// ) -> fmt::Result {
//     f.pad(open)?;
//     if let Some(opts) = opts.for_children() {
//         for (index, shape) in shapes.iter().enumerate() {
//             if index > 0 {
//                 f.pad(delimiter)?;
//             }
//             shape.write_type_name(f, opts)?;
//         }
//     } else {
//         write!(f, "⋯")?;
//     }
//     f.pad(close)?;
//     Ok(())
// }

// macro_rules! field {
//     ($idx:tt, $ty:ty,) => {
//         Field::builder()
//             .name(stringify!($idx))
//             .shape($crate::shape_of(&|t: $ty| t.$idx))
//             .offset(core::mem::offset_of!($ty, $idx))
//             .flags(FieldFlags::EMPTY)
//             .build()
//     };
// }

// unsafe impl<T0> Facet for (T0,)
// where
//     T0: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
//         where
//             T0: Facet,
//         {
//             write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE])
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0,)>())
//             .layout(Layout::new::<(T0,)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[T0::SHAPE]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0,)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0,)>() };
//                             let b = unsafe { b.as_ref::<(T0,)>() };

//                             // Compare last element
//                             unsafe {
//                                 (T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.0 as *const T0),
//                                     OpaqueConst::new(&b.0 as *const T0),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(&const { [field!(0, (T0,),)] })
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1> Facet for (T0, T1)
// where
//     T0: Facet,
//     T1: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//         {
//             write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE, T1::SHAPE])
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1)>())
//             .layout(Layout::new::<(T0, T1)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.1 as *const T1),
//                                     OpaqueConst::new(&b.1 as *const T1),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(&const { [field!(0, (T0, T1,),), field!(1, (T0, T1,),)] })
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2> Facet for (T0, T1, T2)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//         {
//             write_type_name_list(f, opts, "(", ", ", ")", &[T0::SHAPE, T1::SHAPE, T2::SHAPE])
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2)>())
//             .layout(Layout::new::<(T0, T1, T2)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1, T2)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.2 as *const T2),
//                                     OpaqueConst::new(&b.2 as *const T2),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2,),),
//                                 field!(1, (T0, T1, T2,),),
//                                 field!(2, (T0, T1, T2,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3> Facet for (T0, T1, T2, T3)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3)>())
//             .layout(Layout::new::<(T0, T1, T2, T3)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1, T2, T3)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.3 as *const T3),
//                                     OpaqueConst::new(&b.3 as *const T3),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3,),),
//                                 field!(1, (T0, T1, T2, T3,),),
//                                 field!(2, (T0, T1, T2, T3,),),
//                                 field!(3, (T0, T1, T2, T3,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4> Facet for (T0, T1, T2, T3, T4)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4>(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[T0::SHAPE, T1::SHAPE, T2::SHAPE, T3::SHAPE, T4::SHAPE],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.4 as *const T4),
//                                     OpaqueConst::new(&b.4 as *const T4),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4,),),
//                                 field!(1, (T0, T1, T2, T3, T4,),),
//                                 field!(2, (T0, T1, T2, T3, T4,),),
//                                 field!(3, (T0, T1, T2, T3, T4,),),
//                                 field!(4, (T0, T1, T2, T3, T4,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5> Facet for (T0, T1, T2, T3, T4, T5)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4, T5)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.5 as *const T5),
//                                     OpaqueConst::new(&b.5 as *const T5),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6> Facet for (T0, T1, T2, T3, T4, T5, T6)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4, T5, T6)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5, T6)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.6 as *const T6),
//                                     OpaqueConst::new(&b.6 as *const T6),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7> Facet for (T0, T1, T2, T3, T4, T5, T6, T7)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
//     T7: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6, T7>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//             T7: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                     T7::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4, T5, T6, T7)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6, T7>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                         T7::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value =
//                                 unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.7 as *const T7;
//                                 (T7::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 6
//                             unsafe {
//                                 let a_ptr = &a.6 as *const T6;
//                                 let b_ptr = &b.6 as *const T6;
//                                 if !(T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T7::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.7 as *const T7),
//                                     OpaqueConst::new(&b.7 as *const T7),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                                 field!(7, (T0, T1, T2, T3, T4, T5, T6, T7,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Facet for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
//     T7: Facet,
//     T8: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//             T7: Facet,
//             T8: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                     T7::SHAPE,
//                     T8::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                         T7::SHAPE,
//                         T8::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value =
//                                 unsafe { value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.7 as *const T7;
//                                 (T7::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.8 as *const T8;
//                                 (T8::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };
//                             let b = unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 6
//                             unsafe {
//                                 let a_ptr = &a.6 as *const T6;
//                                 let b_ptr = &b.6 as *const T6;
//                                 if !(T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 7
//                             unsafe {
//                                 let a_ptr = &a.7 as *const T7;
//                                 let b_ptr = &b.7 as *const T7;
//                                 if !(T7::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T8::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.8 as *const T8),
//                                     OpaqueConst::new(&b.8 as *const T8),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                                 field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Facet
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
//     T7: Facet,
//     T8: Facet,
//     T9: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//             T7: Facet,
//             T8: Facet,
//             T9: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                     T7::SHAPE,
//                     T8::SHAPE,
//                     T9::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                         T7::SHAPE,
//                         T8::SHAPE,
//                         T9::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe {
//                                 value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>()
//                             };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.7 as *const T7;
//                                 (T7::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.8 as *const T8;
//                                 (T8::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.9 as *const T9;
//                                 (T9::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a =
//                                 unsafe { a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>() };
//                             let b =
//                                 unsafe { b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>() };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 6
//                             unsafe {
//                                 let a_ptr = &a.6 as *const T6;
//                                 let b_ptr = &b.6 as *const T6;
//                                 if !(T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 7
//                             unsafe {
//                                 let a_ptr = &a.7 as *const T7;
//                                 let b_ptr = &b.7 as *const T7;
//                                 if !(T7::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 8
//                             unsafe {
//                                 let a_ptr = &a.8 as *const T8;
//                                 let b_ptr = &b.8 as *const T8;
//                                 if !(T8::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T9::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.9 as *const T9),
//                                     OpaqueConst::new(&b.9 as *const T9),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                                 field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Facet
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
//     T7: Facet,
//     T8: Facet,
//     T9: Facet,
//     T10: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//             T7: Facet,
//             T8: Facet,
//             T9: Facet,
//             T10: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                     T7::SHAPE,
//                     T8::SHAPE,
//                     T9::SHAPE,
//                     T10::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(
//                 T0,
//                 T1,
//                 T2,
//                 T3,
//                 T4,
//                 T5,
//                 T6,
//                 T7,
//                 T8,
//                 T9,
//                 T10,
//             )>())
//             .layout(Layout::new::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                         T7::SHAPE,
//                         T8::SHAPE,
//                         T9::SHAPE,
//                         T10::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe {
//                                 value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>()
//                             };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.7 as *const T7;
//                                 (T7::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.8 as *const T8;
//                                 (T8::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.9 as *const T9;
//                                 (T9::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.10 as *const T10;
//                                 (T10::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe {
//                                 a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>()
//                             };
//                             let b = unsafe {
//                                 b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>()
//                             };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 6
//                             unsafe {
//                                 let a_ptr = &a.6 as *const T6;
//                                 let b_ptr = &b.6 as *const T6;
//                                 if !(T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 7
//                             unsafe {
//                                 let a_ptr = &a.7 as *const T7;
//                                 let b_ptr = &b.7 as *const T7;
//                                 if !(T7::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 8
//                             unsafe {
//                                 let a_ptr = &a.8 as *const T8;
//                                 let b_ptr = &b.8 as *const T8;
//                                 if !(T8::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 9
//                             unsafe {
//                                 let a_ptr = &a.9 as *const T9;
//                                 let b_ptr = &b.9 as *const T9;
//                                 if !(T9::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T10::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.10 as *const T10),
//                                     OpaqueConst::new(&b.10 as *const T10),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                                 field!(10, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
// unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Facet
//     for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
// where
//     T0: Facet,
//     T1: Facet,
//     T2: Facet,
//     T3: Facet,
//     T4: Facet,
//     T5: Facet,
//     T6: Facet,
//     T7: Facet,
//     T8: Facet,
//     T9: Facet,
//     T10: Facet,
//     T11: Facet,
// {
//     const SHAPE: &'static Shape = &const {
//         fn type_name<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>(
//             f: &mut fmt::Formatter,
//             opts: TypeNameOpts,
//         ) -> fmt::Result
//         where
//             T0: Facet,
//             T1: Facet,
//             T2: Facet,
//             T3: Facet,
//             T4: Facet,
//             T5: Facet,
//             T6: Facet,
//             T7: Facet,
//             T8: Facet,
//             T9: Facet,
//             T10: Facet,
//             T11: Facet,
//         {
//             write_type_name_list(
//                 f,
//                 opts,
//                 "(",
//                 ", ",
//                 ")",
//                 &[
//                     T0::SHAPE,
//                     T1::SHAPE,
//                     T2::SHAPE,
//                     T3::SHAPE,
//                     T4::SHAPE,
//                     T5::SHAPE,
//                     T6::SHAPE,
//                     T7::SHAPE,
//                     T8::SHAPE,
//                     T9::SHAPE,
//                     T10::SHAPE,
//                     T11::SHAPE,
//                 ],
//             )
//         }

//         Shape::builder()
//             .id(ConstTypeId::of::<(
//                 T0,
//                 T1,
//                 T2,
//                 T3,
//                 T4,
//                 T5,
//                 T6,
//                 T7,
//                 T8,
//                 T9,
//                 T10,
//                 T11,
//             )>())
//             .layout(Layout::new::<(
//                 T0,
//                 T1,
//                 T2,
//                 T3,
//                 T4,
//                 T5,
//                 T6,
//                 T7,
//                 T8,
//                 T9,
//                 T10,
//                 T11,
//             )>())
//             .vtable(
//                 &const {
//                     let mut builder = ValueVTable::builder()
//                         .type_name(type_name::<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>)
//                         .marker_traits(MarkerTraits::empty());

//                     if Characteristic::Eq.all(&[
//                         T0::SHAPE,
//                         T1::SHAPE,
//                         T2::SHAPE,
//                         T3::SHAPE,
//                         T4::SHAPE,
//                         T5::SHAPE,
//                         T6::SHAPE,
//                         T7::SHAPE,
//                         T8::SHAPE,
//                         T9::SHAPE,
//                         T10::SHAPE,
//                         T11::SHAPE,
//                     ]) {
//                         builder = builder.debug(|value, f| {
//                             let value = unsafe {
//                                 value.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
//                             };
//                             write!(f, "(")?;
//                             unsafe {
//                                 let ptr = &value.0 as *const T0;
//                                 (T0::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.1 as *const T1;
//                                 (T1::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.2 as *const T2;
//                                 (T2::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.3 as *const T3;
//                                 (T3::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.4 as *const T4;
//                                 (T4::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.5 as *const T5;
//                                 (T5::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.6 as *const T6;
//                                 (T6::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.7 as *const T7;
//                                 (T7::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.8 as *const T8;
//                                 (T8::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.9 as *const T9;
//                                 (T9::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.10 as *const T10;
//                                 (T10::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ", ")?;
//                             unsafe {
//                                 let ptr = &value.11 as *const T11;
//                                 (T11::SHAPE.vtable.debug.unwrap_unchecked())(
//                                     OpaqueConst::new(ptr),
//                                     f,
//                                 )
//                             }?;
//                             write!(f, ")")
//                         });

//                         builder = builder.eq(|a, b| {
//                             let a = unsafe {
//                                 a.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
//                             };
//                             let b = unsafe {
//                                 b.as_ref::<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>()
//                             };

//                             // Compare element 0
//                             unsafe {
//                                 let a_ptr = &a.0 as *const T0;
//                                 let b_ptr = &b.0 as *const T0;
//                                 if !(T0::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 1
//                             unsafe {
//                                 let a_ptr = &a.1 as *const T1;
//                                 let b_ptr = &b.1 as *const T1;
//                                 if !(T1::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 2
//                             unsafe {
//                                 let a_ptr = &a.2 as *const T2;
//                                 let b_ptr = &b.2 as *const T2;
//                                 if !(T2::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 3
//                             unsafe {
//                                 let a_ptr = &a.3 as *const T3;
//                                 let b_ptr = &b.3 as *const T3;
//                                 if !(T3::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 4
//                             unsafe {
//                                 let a_ptr = &a.4 as *const T4;
//                                 let b_ptr = &b.4 as *const T4;
//                                 if !(T4::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 5
//                             unsafe {
//                                 let a_ptr = &a.5 as *const T5;
//                                 let b_ptr = &b.5 as *const T5;
//                                 if !(T5::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 6
//                             unsafe {
//                                 let a_ptr = &a.6 as *const T6;
//                                 let b_ptr = &b.6 as *const T6;
//                                 if !(T6::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 7
//                             unsafe {
//                                 let a_ptr = &a.7 as *const T7;
//                                 let b_ptr = &b.7 as *const T7;
//                                 if !(T7::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 8
//                             unsafe {
//                                 let a_ptr = &a.8 as *const T8;
//                                 let b_ptr = &b.8 as *const T8;
//                                 if !(T8::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 9
//                             unsafe {
//                                 let a_ptr = &a.9 as *const T9;
//                                 let b_ptr = &b.9 as *const T9;
//                                 if !(T9::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare element 10
//                             unsafe {
//                                 let a_ptr = &a.10 as *const T10;
//                                 let b_ptr = &b.10 as *const T10;
//                                 if !(T10::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(a_ptr),
//                                     OpaqueConst::new(b_ptr),
//                                 ) {
//                                     return false;
//                                 }
//                             }

//                             // Compare last element
//                             unsafe {
//                                 (T11::SHAPE.vtable.eq.unwrap_unchecked())(
//                                     OpaqueConst::new(&a.11 as *const T11),
//                                     OpaqueConst::new(&b.11 as *const T11),
//                                 )
//                             }
//                         });
//                     }

//                     builder.build()
//                 },
//             )
//             .def(Def::Struct({
//                 Struct::builder()
//                     .kind(StructKind::Tuple)
//                     .fields(
//                         &const {
//                             [
//                                 field!(0, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(1, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(2, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(3, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(4, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(5, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(6, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(7, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(8, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(9, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(10, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                                 field!(11, (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),),
//                             ]
//                         },
//                     )
//                     .build()
//             }))
//             .build()
//     };
// }
