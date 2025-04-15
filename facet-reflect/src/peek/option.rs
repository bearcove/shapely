use facet_core::{OptionDef, OptionVTable};

/// Lets you read from an option (implements read-only option operations)
#[derive(Clone, Copy)]
pub struct PeekOption<'mem> {
    /// the underlying value
    pub(crate) value: crate::Peek<'mem>,

    /// the definition of the option
    pub(crate) def: OptionDef,
}

impl<'mem> PeekOption<'mem> {
    /// Returns the option definition
    #[inline(always)]
    pub fn def(self) -> OptionDef {
        self.def
    }

    /// Returns the option vtable
    #[inline(always)]
    pub fn vtable(self) -> &'static OptionVTable {
        self.def.vtable
    }

    /// Returns whether the option is Some
    #[inline]
    pub fn is_some(self) -> bool {
        unsafe { (self.vtable().is_some_fn)(self.value.data()) }
    }

    /// Returns whether the option is None
    #[inline]
    pub fn is_none(self) -> bool {
        !self.is_some()
    }

    /// Returns the inner value as a Peek if the option is Some, None otherwise
    pub fn value(self) -> Option<crate::Peek<'mem>> {
        unsafe {
            (self.vtable().get_value_fn)(self.value.data()).map(|inner_data| crate::Peek {
                data: inner_data,
                shape: self.def.t(),
            })
        }
    }
}
