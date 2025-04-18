use facet_derive_parse::*;
use std::borrow::Cow;

mod process_enum;
mod process_struct;

pub fn facet_derive(input: TokenStream) -> TokenStream {
    let mut i = input.to_token_iter();

    // Parse as TypeDecl
    match i.parse::<Cons<AdtDecl, EndOfStream>>() {
        Ok(it) => match it.first {
            AdtDecl::Struct(parsed) => process_struct::process_struct(parsed),
            AdtDecl::Enum(parsed) => process_enum::process_enum(parsed),
        },
        Err(err) => {
            panic!(
                "Could not parse type declaration: {}\nError: {}",
                input, err
            );
        }
    }
}

fn generics_split_for_impl(generics: Option<&GenericParams>) -> (String, String) {
    let Some(generics) = generics else {
        return ("".to_string(), "".to_string());
    };
    let mut generics_impl = Vec::new();
    let mut generics_target = Vec::new();

    for param in generics.params.0.iter() {
        match &param.value {
            GenericParam::Type {
                name,
                bounds,
                default: _,
            } => {
                let name = name.to_string();
                let mut impl_ = name.clone();
                if let Some(bounds) = bounds {
                    impl_.push_str(&format!(": {}", VerbatimDisplay(&bounds.second)));
                }
                generics_impl.push(impl_);
                generics_target.push(name);
            }
            GenericParam::Lifetime { name, bounds } => {
                let name = name.to_string();
                let mut impl_ = name.clone();
                if let Some(bounds) = bounds {
                    impl_.push_str(&format!(": {}", VerbatimDisplay(&bounds.second)));
                }
                generics_impl.push(impl_);
                generics_target.push(name);
            }
            GenericParam::Const {
                _const,
                name,
                _colon,
                typ,
                default: _,
            } => {
                let name = name.to_string();
                generics_impl.push(format!("const {}: {}", name, VerbatimDisplay(typ)));
                generics_target.push(name);
            }
        }
    }
    let generics_impl = generics_impl.join(", ");
    let generics_target = generics_target.join(", ");
    (generics_impl, generics_target)
}

/// Converts PascalCase to UPPER_SNAKE_CASE
pub(crate) fn to_upper_snake_case(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if c.is_uppercase() {
                if i > 0 {
                    acc.push('_');
                }
                acc.push(c.to_ascii_uppercase());
            } else {
                acc.push(c.to_ascii_uppercase());
            }
            acc
        })
}

/// Generate a static declaration that exports the crate
pub(crate) fn generate_static_decl(type_name: &str) -> String {
    format!(
        "#[used]\nstatic {}_SHAPE: &'static ::facet::Shape = <{} as ::facet::Facet>::SHAPE;",
        to_upper_snake_case(type_name),
        type_name
    )
}

pub(crate) fn build_maybe_doc(attrs: &[Attribute]) -> String {
    let doc_lines: Vec<_> = attrs
        .iter()
        .filter_map(|attr| match &attr.body.content {
            AttributeInner::Doc(doc_inner) => Some(doc_inner.value.value()),
            _ => None,
        })
        .collect();

    if doc_lines.is_empty() {
        String::new()
    } else {
        format!(r#".doc(&[{}])"#, doc_lines.join(","))
    }
}

/// Generates field definitions for a struct
///
/// `base_field_offset` applies a shift to the field offset, which is useful for
/// generating fields for a struct that is part of a #[repr(C)] enum.
pub(crate) fn gen_struct_field(
    field_name: &str,
    struct_name: &str,
    generics: &str,
    attrs: &[Attribute],
    base_field_offset: Option<&str>,
) -> String {
    // Determine field flags
    let mut flags = "::facet::FieldFlags::EMPTY";
    let mut attribute_list: Vec<String> = vec![];
    let mut doc_lines: Vec<&str> = vec![];
    let mut shape_of = "shape_of";
    for attr in attrs {
        match &attr.body.content {
            AttributeInner::Facet(facet_attr) => match &facet_attr.inner.content {
                FacetInner::Sensitive(_ksensitive) => {
                    flags = "::facet::FieldFlags::SENSITIVE";
                    attribute_list.push("::facet::FieldAttribute::Sensitive".to_string());
                }
                FacetInner::Invariants(_invariant_inner) => {
                    panic!("fields cannot have invariants")
                }
                FacetInner::Opaque(_kopaque) => {
                    shape_of = "shape_of_opaque";
                }
                FacetInner::Other(tt) => {
                    let attr_str = tt.tokens_to_string();
                    let attr_fmt = match attr_str.find('=') {
                        Some(equal_pos) if attr_str[..equal_pos].trim() == "rename" => {
                            let value = attr_str[equal_pos + 1..].trim().trim_matches('"');
                            format!(r#"::facet::FieldAttribute::Rename({:?})"#, value)
                        }
                        _ => {
                            format!(r#"::facet::FieldAttribute::Arbitrary({:?})"#, attr_str)
                        }
                    };
                    attribute_list.push(attr_fmt);
                }
            },
            AttributeInner::Doc(doc_inner) => doc_lines.push(doc_inner.value.value()),
            AttributeInner::Repr(_) => {
                // muffin
            }
            AttributeInner::Any(_) => {
                // muffin two
            }
        }
    }
    let attributes = attribute_list.join(",");

    let maybe_field_doc = if doc_lines.is_empty() {
        String::new()
    } else {
        format!(r#".doc(&[{}])"#, doc_lines.join(","))
    };

    let maybe_base_field_offset = base_field_offset
        .map(|offset| format!(" + {offset}"))
        .unwrap_or_default();

    // Generate each field definition
    format!(
        "::facet::Field::builder()
    .name(\"{field_name}\")
    .shape(|| ::facet::{shape_of}(&|s: &{struct_name}<{generics}>| &s.{field_name}))
    .offset(::core::mem::offset_of!({struct_name}<{generics}>, {field_name}){maybe_base_field_offset})
    .flags({flags})
    .attributes(&[{attributes}])
    {maybe_field_doc}
    .build()"
    )
}

fn build_where_clauses(
    where_clauses: Option<&WhereClauses>,
    generics: Option<&GenericParams>,
) -> String {
    let mut where_clauses_s: Vec<String> = vec![];
    if let Some(wc) = where_clauses {
        for c in &wc.clauses.0 {
            where_clauses_s.push(c.value.to_string())
        }
    }

    if let Some(generics) = generics {
        for p in &generics.params.0 {
            match &p.value {
                GenericParam::Lifetime { .. } => {
                    // ignore for now
                }
                GenericParam::Const { .. } => {
                    // ignore for now
                }
                GenericParam::Type { name, .. } => {
                    where_clauses_s.push(format!("{name}: ::facet::Facet"));
                }
            }
        }
    }

    if where_clauses_s.is_empty() {
        "".to_string()
    } else {
        format!("where {}", where_clauses_s.join(", "))
    }
}

fn build_type_params(generics: Option<&GenericParams>) -> String {
    let mut type_params_s: Vec<String> = vec![];
    if let Some(generics) = generics {
        for p in &generics.params.0 {
            match &p.value {
                GenericParam::Lifetime { .. } => {
                    // ignore for now
                }
                GenericParam::Const { .. } => {
                    // ignore for now
                }
                GenericParam::Type { name, .. } => {
                    type_params_s.push(format!(
                        "::facet::TypeParam {{ name: {:?}, shape: || <{name} as ::facet::Facet>::SHAPE }}",
                        // debug fmt because we want it to be quoted & escaped, but to_string because we don't want the `Ident { .. }`
                        name.to_string()
                    ));
                }
            }
        }
    }

    if type_params_s.is_empty() {
        "".to_string()
    } else {
        format!(".type_params(&[{}])", type_params_s.join(", "))
    }
}

fn get_discriminant_value(lit: &Literal) -> i64 {
    let s = lit.to_string();
    get_discriminant_value_from_str(&s)
}

fn strip_underscores(s: &str) -> Cow<str> {
    if s.contains('_') {
        Cow::Owned(s.chars().filter(|&c| c != '_').collect())
    } else {
        Cow::Borrowed(s)
    }
}

fn get_discriminant_value_from_str(s: &str) -> i64 {
    let s = s.trim();

    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let hex = strip_underscores(hex);
        i64::from_str_radix(&hex, 16).expect("Invalid hex literal for discriminant")
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        let bin = strip_underscores(bin);
        i64::from_str_radix(&bin, 2).expect("Invalid binary literal for discriminant")
    } else if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        let oct = strip_underscores(oct);
        i64::from_str_radix(&oct, 8).expect("Invalid octal literal for discriminant")
    } else {
        // Plain decimal. Support optional _ separators (Rust literals)
        let parsed = strip_underscores(s);
        parsed
            .parse::<i64>()
            .expect("Invalid decimal literal for discriminant")
    }
}

#[cfg(test)]
mod tests {
    use super::get_discriminant_value_from_str;

    #[test]
    fn test_decimal_discriminants() {
        assert_eq!(get_discriminant_value_from_str("7"), 7);
        assert_eq!(get_discriminant_value_from_str("10"), 10);
        assert_eq!(get_discriminant_value_from_str("123_456"), 123456);
        assert_eq!(get_discriminant_value_from_str(" 42 "), 42);
    }

    #[test]
    fn test_hex_discriminants() {
        assert_eq!(get_discriminant_value_from_str("0x01"), 1);
        assert_eq!(get_discriminant_value_from_str("0x7F"), 127);
        assert_eq!(get_discriminant_value_from_str("0x80"), 128);
        assert_eq!(get_discriminant_value_from_str("0x10"), 16);
        assert_eq!(get_discriminant_value_from_str("0xfeed"), 0xfeed);
        assert_eq!(get_discriminant_value_from_str("0xBEEF"), 0xBEEF);
        assert_eq!(get_discriminant_value_from_str("0xBE_EF"), 0xBEEF);
        assert_eq!(get_discriminant_value_from_str("0X1A"), 26);
    }

    #[test]
    fn test_binary_discriminants() {
        assert_eq!(get_discriminant_value_from_str("0b0000_0000"), 0);
        assert_eq!(get_discriminant_value_from_str("0b0000_0001"), 1);
        assert_eq!(get_discriminant_value_from_str("0b0000_0010"), 2);
        assert_eq!(get_discriminant_value_from_str("0b0000_0100"), 4);
        assert_eq!(get_discriminant_value_from_str("0b0000_0111"), 7);
        assert_eq!(get_discriminant_value_from_str("0B1011"), 11);
    }

    #[test]
    fn test_octal_discriminants() {
        assert_eq!(get_discriminant_value_from_str("0o77"), 63);
        assert_eq!(get_discriminant_value_from_str("0o077"), 63);
        assert_eq!(get_discriminant_value_from_str("0o123"), 83);
        assert_eq!(get_discriminant_value_from_str("0o1_234"), 668);
        assert_eq!(get_discriminant_value_from_str("0O345"), 229);
    }

    #[test]
    fn test_mixed_notations() {
        assert_eq!(get_discriminant_value_from_str("1"), 1);
        assert_eq!(get_discriminant_value_from_str("0xA"), 10);
        assert_eq!(get_discriminant_value_from_str("0b1111"), 15);
        assert_eq!(get_discriminant_value_from_str("0o77"), 63);
    }
}
