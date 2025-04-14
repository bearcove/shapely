use crate::OpaqueConst;

/// Definition for scalar types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct ScalarDef {
    /// Affinity of the scalar — is spiritually more like a number, more like a string, something else?
    /// example: an IPv4 address is both. good luck.
    pub affinity: ScalarAffinity,
}

impl ScalarDef {
    /// Returns a builder for ScalarDef
    pub const fn builder() -> ScalarDefBuilder {
        ScalarDefBuilder::new()
    }
}

/// Builder for ScalarDef
#[derive(Default)]
pub struct ScalarDefBuilder {
    affinity: Option<ScalarAffinity>,
}

impl ScalarDefBuilder {
    /// Creates a new ScalarDefBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self { affinity: None }
    }

    /// Sets the affinity for the ScalarDef
    pub const fn affinity(mut self, affinity: ScalarAffinity) -> Self {
        self.affinity = Some(affinity);
        self
    }

    /// Builds the ScalarDef
    pub const fn build(self) -> ScalarDef {
        ScalarDef {
            affinity: self.affinity.unwrap(),
        }
    }
}

/// Scalar affinity: what a scalar spiritually is: a number, a string, a bool, something else
/// entirely?
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub enum ScalarAffinity {
    /// Number-like scalar affinity
    Number(NumberAffinity),
    /// Complex-Number-like scalar affinity
    ComplexNumber(ComplexNumberAffinity),
    /// String-like scalar affinity
    String(StringAffinity),
    /// Boolean scalar affinity
    Boolean(BoolAffinity),
    /// Empty scalar affinity
    Empty(EmptyAffinity),
    /// Socket address scalar affinity
    SocketAddr(SocketAddrAffinity),
    /// Ip Address scalar affinity
    IpAddr(IpAddrAffinity),
    /// UUID or UUID-like identifier, containing 16 bytes of information
    UUID(UuidAffinity),
    /// Timestamp or Datetime-like scalar affinity
    Time(TimeAffinity),
    /// Something you're not supposed to look inside of
    Opaque(OpaqueAffinity),
    /// Other scalar affinity
    Other(OtherAffinity),
    /// Character scalar affinity
    Char(CharAffinity),
}

impl ScalarAffinity {
    /// Returns a NumberAffinityBuilder
    pub const fn number() -> NumberAffinityBuilder {
        NumberAffinityBuilder::new()
    }

    /// Returns a ComplexNumberAffinityBuilder
    pub const fn complex_number() -> ComplexNumberAffinityBuilder {
        ComplexNumberAffinityBuilder::new()
    }

    /// Returns a StringAffinityBuilder
    pub const fn string() -> StringAffinityBuilder {
        StringAffinityBuilder::new()
    }

    /// Returns a BoolAffinityBuilder
    pub const fn boolean() -> BoolAffinityBuilder {
        BoolAffinityBuilder::new()
    }

    /// Returns an EmptyAffinityBuilder
    pub const fn empty() -> EmptyAffinityBuilder {
        EmptyAffinityBuilder::new()
    }

    /// Returns a SocketAddrAffinityBuilder
    pub const fn socket_addr() -> SocketAddrAffinityBuilder {
        SocketAddrAffinityBuilder::new()
    }

    /// Returns an IpAddrAffinityBuilder
    pub const fn ip_addr() -> IpAddrAffinityBuilder {
        IpAddrAffinityBuilder::new()
    }

    /// Returns an UuidAffinityBuilder
    pub const fn uuid() -> UuidAffinityBuilder {
        UuidAffinityBuilder::new()
    }

    /// Returns an TimeAffinityBuilder
    pub const fn time() -> TimeAffinityBuilder {
        TimeAffinityBuilder::new()
    }

    /// Returns an OpaqueAffinityBuilder
    pub const fn opaque() -> OpaqueAffinityBuilder {
        OpaqueAffinityBuilder::new()
    }

    /// Returns an OtherAffinityBuilder
    pub const fn other() -> OtherAffinityBuilder {
        OtherAffinityBuilder::new()
    }

    /// Returns a CharAffinityBuilder
    pub const fn char() -> CharAffinityBuilder {
        CharAffinityBuilder::new()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Affinities
//////////////////////////////////////////////////////////////////////////////////////////

/// Definition for number-like scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct NumberAffinity {
    /// Bit representation of numbers
    pub bits: NumberBits,

    /// Minimum representable value
    pub min: OpaqueConst<'static>,

    /// Maximum representable value
    pub max: OpaqueConst<'static>,

    /// Positive infinity representable value
    pub positive_infinity: Option<OpaqueConst<'static>>,

    /// Negative infinity representable value
    pub negative_infinity: Option<OpaqueConst<'static>>,

    /// Example NaN (Not a Number) value.
    /// Why sample? Because there are many NaN values, and we need to provide a representative one.
    pub nan_sample: Option<OpaqueConst<'static>>,

    /// Positive zero representation. If there's only one zero, only set this one.
    pub positive_zero: Option<OpaqueConst<'static>>,

    /// Negative zero representation
    pub negative_zero: Option<OpaqueConst<'static>>,

    /// "Machine epsilon" (<https://en.wikipedia.org/wiki/Machine_epsilon>), AKA relative
    /// approximation error, if relevant
    pub epsilon: Option<OpaqueConst<'static>>,
}

/// Represents whether a numeric type is signed or unsigned
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub enum Signedness {
    /// Signed numeric type
    Signed,
    /// Unsigned numeric type
    Unsigned,
}

/// Bit representation of numbers
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub enum NumberBits {
    /// Integer number limits with specified number of bits
    Integer {
        /// Number of bits in the integer representation
        bits: usize,
        /// Whether the integer is signed or unsigned
        sign: Signedness,
    },
    /// Floating-point number limits with specified sign, exponent and mantissa bits
    Float {
        /// Number of bits used for the sign (typically 1)
        sign_bits: usize,
        /// Number of bits used for the exponent
        exponent_bits: usize,
        /// Number of bits used for the mantissa (fraction part)
        mantissa_bits: usize,
        /// Floating-point numbers that are large enough to not be "in subnormal mode"
        /// have their mantissa represent a number between 1 (included) and 2 (excluded)
        /// This indicates whether the representation of the mantissa has the significant digit
        /// (always 1) explicitly written out
        has_explicit_first_mantissa_bit: bool,
    },
    /// Fixed-point number limits with specified integer and fractional bits
    Fixed {
        /// Number of bits used for the sign (typically 0 or 1)
        sign_bits: usize,
        /// Number of bits used for the integer part
        integer_bits: usize,
        /// Number of bits used for the fractional part
        fraction_bits: usize,
    },
    /// Decimal number limits with unsized-integer, scaling, and sign bits
    Decimal {
        /// Number of bits used for the sign (typically 0 or 1)
        sign_bits: usize,
        /// Number of bits used for the integer part
        integer_bits: usize,
        /// Number of bits used for the scale part
        scale_bits: usize,
    },
}

impl NumberAffinity {
    /// Returns a builder for NumberAffinity
    pub const fn builder() -> NumberAffinityBuilder {
        NumberAffinityBuilder::new()
    }
}

/// Builder for NumberAffinity
#[repr(C)]
pub struct NumberAffinityBuilder {
    limits: Option<NumberBits>,
    min: Option<OpaqueConst<'static>>,
    max: Option<OpaqueConst<'static>>,
    positive_infinity: Option<OpaqueConst<'static>>,
    negative_infinity: Option<OpaqueConst<'static>>,
    nan_sample: Option<OpaqueConst<'static>>,
    positive_zero: Option<OpaqueConst<'static>>,
    negative_zero: Option<OpaqueConst<'static>>,
    epsilon: Option<OpaqueConst<'static>>,
}

impl NumberAffinityBuilder {
    /// Creates a new NumberAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            limits: None,
            min: None,
            max: None,
            positive_infinity: None,
            negative_infinity: None,
            nan_sample: None,
            positive_zero: None,
            negative_zero: None,
            epsilon: None,
        }
    }

    /// Sets the number limits as integer with specified bits and sign
    pub const fn integer(mut self, bits: usize, sign: Signedness) -> Self {
        self.limits = Some(NumberBits::Integer { bits, sign });
        self
    }

    /// Sets the number limits as signed integer with specified bits
    pub const fn signed_integer(self, bits: usize) -> Self {
        self.integer(bits, Signedness::Signed)
    }

    /// Sets the number limits as unsigned integer with specified bits
    pub const fn unsigned_integer(self, bits: usize) -> Self {
        self.integer(bits, Signedness::Unsigned)
    }

    /// Sets the number limits as float with specified bits
    pub const fn float(
        mut self,
        sign_bits: usize,
        exponent_bits: usize,
        mantissa_bits: usize,
        has_explicit_first_mantissa_bit: bool,
    ) -> Self {
        self.limits = Some(NumberBits::Float {
            sign_bits,
            exponent_bits,
            mantissa_bits,
            has_explicit_first_mantissa_bit,
        });
        self
    }

    /// Sets the number limits as fixed-point with specified bits
    pub const fn fixed(
        mut self,
        sign_bits: usize,
        integer_bits: usize,
        fraction_bits: usize,
    ) -> Self {
        self.limits = Some(NumberBits::Fixed {
            sign_bits,
            integer_bits,
            fraction_bits,
        });
        self
    }

    /// Sets the min value for the NumberAffinity
    pub const fn min(mut self, min: OpaqueConst<'static>) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the max value for the NumberAffinity
    pub const fn max(mut self, max: OpaqueConst<'static>) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the positive infinity value for the NumberAffinity
    pub const fn positive_infinity(mut self, value: OpaqueConst<'static>) -> Self {
        self.positive_infinity = Some(value);
        self
    }

    /// Sets the negative infinity value for the NumberAffinity
    pub const fn negative_infinity(mut self, value: OpaqueConst<'static>) -> Self {
        self.negative_infinity = Some(value);
        self
    }

    /// Sets the NaN sample value for the NumberAffinity
    pub const fn nan_sample(mut self, value: OpaqueConst<'static>) -> Self {
        self.nan_sample = Some(value);
        self
    }

    /// Sets the positive zero value for the NumberAffinity
    pub const fn positive_zero(mut self, value: OpaqueConst<'static>) -> Self {
        self.positive_zero = Some(value);
        self
    }

    /// Sets the negative zero value for the NumberAffinity
    pub const fn negative_zero(mut self, value: OpaqueConst<'static>) -> Self {
        self.negative_zero = Some(value);
        self
    }

    /// Sets the relative uncertainty for the NumberAffinity
    pub const fn epsilon(mut self, value: OpaqueConst<'static>) -> Self {
        self.epsilon = Some(value);
        self
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Number(NumberAffinity {
            bits: self.limits.unwrap(),
            min: self.min.unwrap(),
            max: self.max.unwrap(),
            positive_infinity: self.positive_infinity,
            negative_infinity: self.negative_infinity,
            nan_sample: self.nan_sample,
            positive_zero: self.positive_zero,
            negative_zero: self.negative_zero,
            epsilon: self.epsilon,
        })
    }
}

/// Definition for string-like scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct ComplexNumberAffinity {
    /// hiding the actual enum in a non-pub element
    inner: ComplexNumberAffinityInner,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
enum ComplexNumberAffinityInner {
    /// represented as a+ib
    Cartesian {
        /// the underlying number affinity for both components
        /// (assuming they are the same seems reasonable)
        component: NumberAffinity,
    },
    /// represented as a*exp(ib)
    Polar {
        /// the number affinity for the absolute value
        absolute: NumberAffinity,
        /// the number affinity for the ...angle? bearing?
        bearing: NumberAffinity,
    },
}

impl ComplexNumberAffinity {
    /// Returns a builder for ComplexNumberAffinity
    pub const fn builder() -> ComplexNumberAffinityBuilder {
        ComplexNumberAffinityBuilder::new()
    }
}

/// Builder for ComplexNumberAffinity
#[repr(C)]
pub struct ComplexNumberAffinityBuilder {
    inner: ComplexNumberAffinityBuilderInner,
}

#[repr(C)]
enum ComplexNumberAffinityBuilderInner {
    Undefined,
    Cartesian {
        // note: this could have been a NumberAffinityBuilder,
        // but we want to be able to set this up from existing Number types
        component: NumberAffinity,
    },
    Polar {
        absolute: NumberAffinity,
        bearing: NumberAffinity,
    },
}

impl ComplexNumberAffinityBuilder {
    /// Creates a new ComplexNumberAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            inner: ComplexNumberAffinityBuilderInner::Undefined,
        }
    }

    /// sets the coordinates system to be cartesian
    pub const fn cartesian(self, component: NumberAffinity) -> Self {
        Self {
            inner: ComplexNumberAffinityBuilderInner::Cartesian { component },
        }
    }

    /// sets the coordinates system to be polar
    pub const fn polar(self, absolute: NumberAffinity, bearing: NumberAffinity) -> Self {
        Self {
            inner: ComplexNumberAffinityBuilderInner::Polar { absolute, bearing },
        }
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        use ComplexNumberAffinityBuilderInner as Inner;
        use ComplexNumberAffinityInner as AffInner;
        let inner = match self.inner {
            Inner::Undefined => panic!(),
            Inner::Cartesian { component } => AffInner::Cartesian { component },
            Inner::Polar { absolute, bearing } => AffInner::Polar { absolute, bearing },
        };
        ScalarAffinity::ComplexNumber(ComplexNumberAffinity { inner })
    }
}

/// Definition for string-like scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct StringAffinity {
    /// Maximum inline length
    pub max_inline_length: Option<usize>,
}

impl StringAffinity {
    /// Returns a builder for StringAffinity
    pub const fn builder() -> StringAffinityBuilder {
        StringAffinityBuilder::new()
    }
}

/// Builder for StringAffinity
#[repr(C)]
pub struct StringAffinityBuilder {
    max_inline_length: Option<usize>,
}

impl StringAffinityBuilder {
    /// Creates a new StringAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            max_inline_length: None,
        }
    }

    /// Sets the max_inline_length for the StringAffinity
    pub const fn max_inline_length(mut self, max_inline_length: usize) -> Self {
        self.max_inline_length = Some(max_inline_length);
        self
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::String(StringAffinity {
            max_inline_length: self.max_inline_length,
        })
    }
}

/// Definition for boolean scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct BoolAffinity {}

impl BoolAffinity {
    /// Returns a builder for BoolAffinity
    pub const fn builder() -> BoolAffinityBuilder {
        BoolAffinityBuilder::new()
    }
}

/// Builder for BoolAffinity
#[repr(C)]
pub struct BoolAffinityBuilder {}

impl BoolAffinityBuilder {
    /// Creates a new BoolAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Boolean(BoolAffinity {})
    }
}

/// Definition for empty scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct EmptyAffinity {}

impl EmptyAffinity {
    /// Returns a builder for EmptyAffinity
    pub const fn builder() -> EmptyAffinityBuilder {
        EmptyAffinityBuilder::new()
    }
}

/// Builder for EmptyAffinity
#[repr(C)]
pub struct EmptyAffinityBuilder {}

impl EmptyAffinityBuilder {
    /// Creates a new EmptyAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Empty(EmptyAffinity {})
    }
}

/// Definition for socket address scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct SocketAddrAffinity {}

impl SocketAddrAffinity {
    /// Returns a builder for SocketAddrAffinity
    pub const fn builder() -> SocketAddrAffinityBuilder {
        SocketAddrAffinityBuilder::new()
    }
}

/// Builder for SocketAddrAffinity
#[repr(C)]
pub struct SocketAddrAffinityBuilder {}

impl SocketAddrAffinityBuilder {
    /// Creates a new SocketAddrAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::SocketAddr(SocketAddrAffinity {})
    }
}

/// Definition for IP address scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct IpAddrAffinity {}

impl IpAddrAffinity {
    /// Returns a builder for IpAddrAffinity
    pub const fn builder() -> IpAddrAffinityBuilder {
        IpAddrAffinityBuilder::new()
    }
}

/// Builder for IpAddrAffinity
#[repr(C)]
pub struct IpAddrAffinityBuilder {}

impl IpAddrAffinityBuilder {
    /// Creates a new IpAddrAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::IpAddr(IpAddrAffinity {})
    }
}

/// Definition for UUID and UUID-like scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct UuidAffinity {}

impl UuidAffinity {
    /// Returns a builder for UuidAffinity
    pub const fn builder() -> UuidAffinityBuilder {
        UuidAffinityBuilder::new()
    }
}

/// Builder for UuidAffinity
#[repr(C)]
pub struct UuidAffinityBuilder {}

impl UuidAffinityBuilder {
    /// Creates a new UuidAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::UUID(UuidAffinity {})
    }
}

/// Definition for Datetime/Timestamp scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct TimeAffinity {
    /// What serves as the reference, or "time zero"
    /// for implementations that don't depend on an epoch in the traditionnal sense,
    /// the first moment of year 1AD can be used
    epoch: Option<OpaqueConst<'static>>,

    /// The first moment representable
    min: Option<OpaqueConst<'static>>,

    /// The last moment representable
    max: Option<OpaqueConst<'static>>,

    /// The moment immediately after the epoch,
    /// serving as a proxy for the smallest interval of time representable
    /// (do use None if this interval depends on when in time the interval occurs, e.g. if someone
    /// ever decides to store a timestamp on floating-point numbers)
    granularity: Option<OpaqueConst<'static>>,

    // TODO: the following solution leaves a LOT to desire.
    // Some examples of things where this breaks:
    // - leap years, day length in daylight savings, leap seconds
    // - datetime objects that seamlessly switch from Julian to Gregorian calendar
    //   - even worse if this transition is based on when a given country did, if there even is
    //   something that does this
    // - datetime objects that allow you to specify both individual Gregorian months and ISO 8601
    //   weeks (but of course not at the same time, which is the whole difficulty)
    /// For DateTime types made of interval elements some of which are optional
    /// (for instance, letting you say "the 1st of March" without specifying year, hours, etc.)
    /// Specify how long the interval elements (hour, minute, etc.) are
    /// (all represented as moments separated from the epoch by said intervals)
    /// the intervals MUST be of increasing length. (TODO bikeshedding for this line)
    interval_elements: Option<&'static [OpaqueConst<'static>]>,

    /// the minimum interval between timezone-local times which correspond to the same global time
    /// (planet-local time? I mean duh that's what global means right?)
    /// store a copy of the epoch for a lack of timezone support, and None for "it's more
    /// complicated than that".
    timezone_granularity: Option<OpaqueConst<'static>>,
}

impl TimeAffinity {
    /// Returns a builder for TimeAffinity
    pub const fn builder() -> TimeAffinityBuilder {
        TimeAffinityBuilder::new()
    }
}

/// Builder for UuidAffinity
#[repr(C)]
pub struct TimeAffinityBuilder {
    epoch: Option<OpaqueConst<'static>>,
    min: Option<OpaqueConst<'static>>,
    max: Option<OpaqueConst<'static>>,
    granularity: Option<OpaqueConst<'static>>,
    interval_elements: Option<&'static [OpaqueConst<'static>]>,
    timezone_granularity: Option<OpaqueConst<'static>>,
}

impl TimeAffinityBuilder {
    /// Creates a new UuidAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            epoch: None,
            min: None,
            max: None,
            granularity: None,
            interval_elements: None,
            timezone_granularity: None,
        }
    }

    /// Sets the epoch for the TimeAffinity
    pub const fn epoch(mut self, epoch: OpaqueConst<'static>) -> Self {
        self.epoch = Some(epoch);
        self
    }

    /// Sets the min value for the TimeAffinity
    pub const fn min(mut self, min: OpaqueConst<'static>) -> Self {
        self.min = Some(min);
        self
    }

    /// Sets the max value for the TimeAffinity
    pub const fn max(mut self, max: OpaqueConst<'static>) -> Self {
        self.max = Some(max);
        self
    }

    /// Sets the granularity for the TimeAffinity
    pub const fn granularity(mut self, granularity: OpaqueConst<'static>) -> Self {
        self.granularity = Some(granularity);
        self
    }

    /// Sets the interval elements for the TimeAffinity
    pub const fn interval_elements(
        mut self,
        interval_elements: &'static [OpaqueConst<'static>],
    ) -> Self {
        self.interval_elements = Some(interval_elements);
        self
    }

    /// Sets the timezone granularity for the TimeAffinity
    pub const fn timezone_granularity(
        mut self,
        timezone_granularity: OpaqueConst<'static>,
    ) -> Self {
        self.timezone_granularity = Some(timezone_granularity);
        self
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Time(TimeAffinity {
            epoch: self.epoch,
            min: self.min,
            max: self.max,
            granularity: self.granularity,
            interval_elements: self.interval_elements,
            timezone_granularity: self.timezone_granularity,
        })
    }
}

/// Definition for opaque scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct OpaqueAffinity {}

impl OpaqueAffinity {
    /// Returns a builder for OpaqueAffinity
    pub const fn builder() -> OpaqueAffinityBuilder {
        OpaqueAffinityBuilder::new()
    }
}

/// Builder for OpaqueAffinity
#[repr(C)]
pub struct OpaqueAffinityBuilder {}

impl OpaqueAffinityBuilder {
    /// Creates a new OpaqueAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Opaque(OpaqueAffinity {})
    }
}

/// Definition for other scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct OtherAffinity {}

impl OtherAffinity {
    /// Returns a builder for OtherAffinity
    pub const fn builder() -> OtherAffinityBuilder {
        OtherAffinityBuilder::new()
    }
}

/// Builder for OtherAffinity
#[repr(C)]
pub struct OtherAffinityBuilder {}

impl OtherAffinityBuilder {
    /// Creates a new OtherAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Other(OtherAffinity {})
    }
}

/// Definition for character scalar affinities
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
#[non_exhaustive]
pub struct CharAffinity {}

impl CharAffinity {
    /// Returns a builder for CharAffinity
    pub const fn builder() -> CharAffinityBuilder {
        CharAffinityBuilder::new()
    }
}

/// Builder for CharAffinity
#[repr(C)]
pub struct CharAffinityBuilder {}

impl CharAffinityBuilder {
    /// Creates a new CharAffinityBuilder
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Builds the ScalarAffinity
    pub const fn build(self) -> ScalarAffinity {
        ScalarAffinity::Char(CharAffinity {})
    }
}
