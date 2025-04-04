//! Allows peeking (reading from) shapes

use shapely_trait::{Shapely, TypeNameOpts};

mod value;
pub use value::*;

mod struct_;
pub use struct_::*;

mod list;
pub use list::*;

mod map;
pub use map::*;

use shapely_trait::{Def, OpaqueConst, Shape};

/// Lets you peek at the innards of a value
///
/// It's possible (in some cases..) to escape the borrow checker by setting `'mem` to `'static`,
/// in which case, you're entirely on your own.
#[derive(Clone, Copy)]
pub enum Peek<'mem> {
    /// cf. [`PeekValue`]
    Value(PeekValue<'mem>),

    /// cf. [`PeekList`]
    List(PeekList<'mem>),

    /// cf. [`PeekMap`]
    Map(PeekMap<'mem>),

    /// cf. [`PeekStruct`]
    Struct(PeekStruct<'mem>),
}

impl<'mem> std::ops::Deref for Peek<'mem> {
    type Target = PeekValue<'mem>;

    fn deref(&self) -> &Self::Target {
        match self {
            Peek::Value(value) => value,
            Peek::List(list) => list,
            Peek::Map(map) => map,
            Peek::Struct(struct_) => struct_,
        }
    }
}

impl<'mem> Peek<'mem> {
    /// Creates a new peek from a reference to some initialized value that implements `Shapely`
    pub fn new<S: Shapely>(s: &'mem S) -> Self {
        // This is safe because we're creating an Opaque pointer to read-only data
        // The pointer will be valid for the lifetime 'mem
        let data = OpaqueConst::from_ref(s);
        unsafe { Self::unchecked_new(data, S::SHAPE) }
    }

    /// Creates a new peek, for easy manipulation of some opaque data.
    ///
    /// # Safety
    ///
    /// `data` must be initialized and well-aligned, and point to a value
    /// of the type described by `shape`.
    pub unsafe fn unchecked_new(data: OpaqueConst<'mem>, shape: &'static Shape) -> Self {
        let value = PeekValue::new(data, shape);
        match shape.def {
            Def::Struct(def) | Def::TupleStruct(def) | Def::Tuple(def) => {
                Peek::Struct(PeekStruct::new(value, def))
            }
            Def::Map(def) => Peek::Map(PeekMap::new(value, def)),
            Def::List(def) => Peek::List(PeekList::new(value, def)),
            Def::Scalar { .. } => Peek::Value(value),
            Def::Enum { .. } => todo!(),
        }
    }

    /// Coerce this to a value so we can use display, debug, etc.
    pub fn as_value(self) -> PeekValue<'mem> {
        match self {
            Self::Value(v) => v,
            Self::List(l) => *l,
            Self::Map(m) => *m,
            Self::Struct(s) => *s,
        }
    }
}

impl std::fmt::Debug for Peek<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_value();
        if value.debug(f).is_none() {
            value.type_name(f, TypeNameOpts::infinite())?;
            write!(f, "(⋯)")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Peek<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_value();
        if value.display(f).is_none() {
            value.type_name(f, TypeNameOpts::infinite())?;
            write!(f, "(⋯)")?;
        }
        Ok(())
    }
}
