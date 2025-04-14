use std::num::NonZero;

use facet::Facet;
use facet_reflect::Peek;

#[test]
fn test_to_json() {
    facet_testhelpers::setup();

    #[derive(Debug, PartialEq, Clone, Facet)]
    struct LinearFunction {
        variable: String,
        slope: f32,
        intercept: i32,
    }

    let test_struct = LinearFunction {
        variable: "x".to_string(),
        slope: -3.5,
        intercept: -5,
    };

    let expected_json = r#"{"variable":"x","slope":-3.5,"intercept":-5}"#;

    // Test without indentation (using to_string)
    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, expected_json);

    // Test with indentation (using to_writer directly with a custom writer)
    let mut buffer = Vec::new();
    facet_json::to_writer(&test_struct, &mut buffer).unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json);
}

#[test]
fn test_nonzero() {
    facet_testhelpers::setup();

    #[derive(Debug, PartialEq, Clone, Facet)]
    struct Foo {
        foo: NonZero<u8>,
    }

    let test_struct = Foo {
        foo: const { NonZero::new(1).unwrap() },
    };

    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, r#"{"foo":1}"#);
}

#[test]
fn test_hashmap_to_json() {
    facet_testhelpers::setup();

    let mut json_data = std::collections::HashMap::<&str, &str>::new();
    json_data.insert("foo", "bar");

    let expected_json = r#"{"foo":"bar"}"#;

    // Using peek_to_string
    let peek = Peek::new(&json_data);
    let json = facet_json::peek_to_string(&peek);
    assert_eq!(json, expected_json);

    // Using peek_to_writer
    let mut buffer = Vec::new();
    facet_json::peek_to_writer(&peek, &mut buffer).unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json);
}

#[test]
fn test_static_strings() {
    facet_testhelpers::setup();

    #[derive(Debug, PartialEq, Clone, Facet)]
    struct StaticFoo {
        foo: &'static str,
    }

    let test_struct = StaticFoo { foo: "foo" };

    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, r#"{"foo":"foo"}"#);

    #[derive(Debug, PartialEq, Clone, Facet)]
    struct OptStaticFoo {
        foo: Option<&'static str>,
    }

    let test_struct = OptStaticFoo { foo: None };

    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, r#"{"foo":null}"#);

    let test_struct = OptStaticFoo { foo: Some("foo") };

    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, r#"{"foo":"foo"}"#);

    #[derive(Debug, PartialEq, Clone, Facet)]
    struct CowFoo {
        foo: std::borrow::Cow<'static, str>,
    }

    let test_struct = CowFoo {
        foo: std::borrow::Cow::from("foo"),
    };

    let json = facet_json::to_string(&test_struct);
    assert_eq!(json, r#"{"foo":"foo"}"#);
}
