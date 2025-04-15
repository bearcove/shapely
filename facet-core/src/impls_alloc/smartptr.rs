use core::alloc::Layout;

use crate::{
    ConstTypeId, Def, Facet, KnownSmartPointer, Opaque, PtrConst, SmartPointerDef,
    SmartPointerFlags, SmartPointerVTable, value_vtable,
};

unsafe impl<T: Facet> Facet for alloc::sync::Arc<T> {
    const SHAPE: &'static crate::Shape = &const {
        crate::Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::SmartPointer(
                SmartPointerDef::builder()
                    .pointee(T::SHAPE)
                    .flags(SmartPointerFlags::ATOMIC)
                    .known(KnownSmartPointer::Arc)
                    .weak(|| <alloc::sync::Weak<T> as Facet>::SHAPE)
                    .vtable(
                        &const {
                            SmartPointerVTable::builder()
                                .borrow_fn(|opaque| {
                                    let ptr = Self::as_ptr(unsafe { opaque.get() });
                                    PtrConst::new(ptr)
                                })
                                .new_into_fn(|this, ptr| {
                                    let t = unsafe { ptr.read::<T>() };
                                    let arc = alloc::sync::Arc::new(t);
                                    unsafe { this.put(arc) }
                                })
                                .downgrade_fn(|strong, weak| unsafe {
                                    weak.put(alloc::sync::Arc::downgrade(strong.get::<Self>()))
                                })
                                .build()
                        },
                    )
                    .build(),
            ))
            .vtable(value_vtable!(alloc::sync::Arc<T>, |f, _opts| write!(
                f,
                "Arc"
            )))
            .build()
    };
}

unsafe impl<T: Facet> Facet for alloc::sync::Weak<T> {
    const SHAPE: &'static crate::Shape = &const {
        crate::Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::SmartPointer(
                SmartPointerDef::builder()
                    .pointee(T::SHAPE)
                    .flags(SmartPointerFlags::ATOMIC.union(SmartPointerFlags::WEAK))
                    .known(KnownSmartPointer::ArcWeak)
                    .strong(|| <alloc::sync::Arc<T> as Facet>::SHAPE)
                    .vtable(
                        &const {
                            SmartPointerVTable::builder()
                                .upgrade_into_fn(|weak, strong| unsafe {
                                    Some(strong.put(weak.get::<Self>().upgrade()?))
                                })
                                .build()
                        },
                    )
                    .build(),
            ))
            .vtable(value_vtable!(alloc::sync::Arc<T>, |f, _opts| write!(
                f,
                "Arc"
            )))
            .build()
    };
}

unsafe impl<T> Facet for Opaque<alloc::sync::Arc<T>> {
    const SHAPE: &'static crate::Shape = &const {
        crate::Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::SmartPointer(
                SmartPointerDef::builder()
                    .flags(SmartPointerFlags::ATOMIC)
                    .known(KnownSmartPointer::Arc)
                    .vtable(
                        &const {
                            SmartPointerVTable::builder()
                                .borrow_fn(|opaque| {
                                    let ptr =
                                        alloc::sync::Arc::<T>::as_ptr(unsafe { opaque.get() });
                                    PtrConst::new(ptr)
                                })
                                .new_into_fn(|this, ptr| {
                                    let t = unsafe { ptr.read::<T>() };
                                    let arc = alloc::sync::Arc::new(t);
                                    unsafe { this.put(arc) }
                                })
                                .build()
                        },
                    )
                    .build(),
            ))
            .vtable(value_vtable!(alloc::sync::Arc<T>, |f, _opts| write!(
                f,
                "Arc"
            )))
            .build()
    };
}
