use super::Struct;

/// Fields for enum types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct EnumDef {
    /// representation of the enum (u8, u16, etc.)
    pub repr: EnumRepr,

    /// all variants for this enum
    pub variants: &'static [Variant],
}

impl EnumDef {
    /// Returns a builder for EnumDef
    pub const fn builder() -> EnumDefBuilder {
        EnumDefBuilder::new()
    }
}

/// Builder for EnumDef
pub struct EnumDefBuilder {
    repr: Option<EnumRepr>,
    variants: Option<&'static [Variant]>,
}

impl EnumDefBuilder {
    /// Creates a new EnumDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            repr: None,
            variants: None,
        }
    }

    /// Sets the representation for the EnumDef
    pub const fn repr(mut self, repr: EnumRepr) -> Self {
        self.repr = Some(repr);
        self
    }

    /// Sets the variants for the EnumDef
    pub const fn variants(mut self, variants: &'static [Variant]) -> Self {
        self.variants = Some(variants);
        self
    }

    /// Builds the EnumDef
    pub const fn build(self) -> EnumDef {
        EnumDef {
            repr: self.repr.unwrap(),
            variants: self.variants.unwrap(),
        }
    }
}

/// Describes a variant of an enum
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct Variant {
    /// Name of the variant, e.g. `Foo` for `enum FooBar { Foo, Bar }`
    pub name: &'static str,

    /// Discriminant value (if available). Might fit in a u8, etc.
    pub discriminant: i64,

    /// Fields for this variant (empty if unit, number-named if tuple).
    /// IMPORTANT: the offset for the fields already takes into account the size & alignment of the
    /// discriminant.
    pub fields: Struct,

    /// Doc comment for the variant
    pub doc: &'static [&'static str],
}

impl Variant {
    /// Returns a builder for Variant
    pub const fn builder() -> VariantBuilder {
        VariantBuilder::new()
    }
}

/// Builder for Variant
pub struct VariantBuilder {
    name: Option<&'static str>,
    discriminant: Option<i64>,
    fields: Option<Struct>,
    offset: Option<usize>,
    doc: &'static [&'static str],
}

impl VariantBuilder {
    /// Creates a new VariantBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            name: None,
            discriminant: None,
            fields: None,
            offset: None,
            doc: &[],
        }
    }

    /// Sets the name for the Variant
    pub const fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the discriminant for the Variant
    pub const fn discriminant(mut self, discriminant: i64) -> Self {
        self.discriminant = Some(discriminant);
        self
    }

    /// Sets the fields for the Variant
    pub const fn fields(mut self, fields: Struct) -> Self {
        self.fields = Some(fields);
        self
    }

    /// Sets the offset for the Variant
    pub const fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the doc comment for the Variant
    pub const fn doc(mut self, doc: &'static [&'static str]) -> Self {
        self.doc = doc;
        self
    }

    /// Builds the Variant
    pub const fn build(self) -> Variant {
        Variant {
            name: self.name.unwrap(),
            discriminant: self.discriminant.unwrap(),
            fields: self.fields.unwrap(),
            doc: self.doc,
        }
    }
}

/// All possible representations for Rust enums — ie. the type/size of the discriminant
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub enum EnumRepr {
    /// u8 representation (#[repr(u8)])
    U8,
    /// u16 representation (#[repr(u16)])
    U16,
    /// u32 representation (#[repr(u32)])
    U32,
    /// u64 representation (#[repr(u64)])
    U64,
    /// usize representation (#[repr(usize)])
    USize,
    /// i8 representation (#[repr(i8)])
    I8,
    /// i16 representation (#[repr(i16)])
    I16,
    /// i32 representation (#[repr(i32)])
    I32,
    /// i64 representation (#[repr(i64)])
    I64,
    /// isize representation (#[repr(isize)])
    ISize,
}

impl EnumRepr {
    /// Returns the enum representation for the given discriminant type
    ///
    /// NOTE: only supports unsigned discriminants
    ///
    /// # Panics
    ///
    /// Panics if the size of the discriminant size is not 1, 2, 4, or 8 bytes.
    pub const fn from_discriminant_size<T>() -> Self {
        match core::mem::size_of::<T>() {
            1 => EnumRepr::U8,
            2 => EnumRepr::U16,
            4 => EnumRepr::U32,
            8 => EnumRepr::U64,
            _ => panic!("Invalid enum size"),
        }
    }
}
