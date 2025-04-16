use crate::*;
use core::alloc::Layout;
use typeid::ConstTypeId;

unsafe impl Facet for std::path::PathBuf {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(
                ScalarDef::builder()
                    .affinity(ScalarAffinity::path().build())
                    .build(),
            ))
            .vtable(value_vtable!((), |f, _opts| write!(f, "PathBuf")))
            .build()
    };
}

unsafe impl Facet for &std::path::Path {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::Scalar(
                ScalarDef::builder()
                    .affinity(ScalarAffinity::path().build())
                    .build(),
            ))
            .vtable(value_vtable!((), |f, _opts| write!(f, "Path")))
            .build()
    };
}
