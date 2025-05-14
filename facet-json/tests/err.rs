use facet::Facet;
use facet_json::from_str;
use facet_testhelpers::test;

#[derive(Facet, Debug)]
struct Foo {
    foo: u32,
}

#[derive(Facet, Debug)]
struct FooBar {
    foo: u64,
    bar: String,
}

#[cfg(not(miri))]
#[test]
fn bad_json_1() {
    let json = "}";
    let err = from_str::<Foo>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn bad_json_2() {
    let json = " }";
    let err = from_str::<Foo>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn bad_json_3() {
    let json = "\n}";
    let err = from_str::<Foo>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn bad_json_4() {
    let json = "\n  }";
    let err = from_str::<Foo>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn bad_json_5() {
    let json = "\n  }\n// and then some";
    let err = from_str::<Foo>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn bad_json_6_string_as_number_subpath() {
    let json = r#"{"foo": 42, "bar": 42}"#;
    let err = from_str::<FooBar>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

#[cfg(not(miri))]
#[test]
fn unknown_field_with_rename() {
    #[derive(Facet, Debug)]
    #[facet(deny_unknown_fields)]
    struct RenamedFields {
        #[facet(rename = "new_name")]
        original_name: String,
    }

    // This should fail because "wrong_name" doesn't match either the original field name
    // or the renamed field name
    let json = r#"{"wrong_name": "value"}"#;
    let err = from_str::<RenamedFields>(json).unwrap_err();
    insta::assert_snapshot!(err);
}

/// Expect a 1-tuple but it's a 2-tuple
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_wrong_size_one_too_many() {
    let result: Result<(i32,), _> = from_str("[10,20]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// --- EMPTY TUPLE (UNIT TYPE) TESTS
//
// Top level `[]` is `()` the unit type. Any nested deeper are called "parent", "grandparent" etc.
//
// --- RECEIVING UNIT TYPE AT THE WRONG NESTING

// (u:uP) Not unit --> unit parent
/// Expect a 0-tuple but it's a 1-tuple[0-tuple]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_from_empty_nested() {
    let result: Result<(), _> = from_str("[[]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uP:uGP) Not unit parent --> unit grandparent
/// Expect a 1-tuple[0-tuple] but it's a 1-tuple[1-tuple[0-tuple]]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_from_empty_nested_2x() {
    let result: Result<((),), _> = from_str("[[[]]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uP:uGGP) Not unit parent --> unit great grandparent
/// Expect a 1-tuple[0-tuple] but it's a 1-tuple[1-tuple[1-tuple[0-tuple]]]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_from_empty_nested_3x() {
    let result: Result<((),), _> = from_str("[[[[]]]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// --- RECEIVING NON-EMPTY INSTEAD OF UNIT TYPE

// (u:i) Not unit --> int
/// Expect a 0-tuple but it's a 1-tuple
#[cfg(not(miri))]
#[test]
fn test_deserialize_0tup_from_1tup() {
    let result: Result<(), _> = from_str("[10]");
    let err = result.unwrap_err();
    // insta::assert_debug_snapshot!(err);
    insta::assert_snapshot!(err);
}

// (uP:i) Not unit parent --> int
/// Expect a 1-tuple[0-tuple] but it's a 1-tuple
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_from_1tup() {
    let result: Result<((),), _> = from_str("[10]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uP:iP) Not unit parent --> int parent
/// Expect a 1-tuple[0-tuple] but it's a 1-tuple[1-tuple]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_from_1tup_nested() {
    let result: Result<((),), _> = from_str("[[10]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uGP:i) Not unit grandparent --> int
/// Expect a 1-tuple[1-tuple[0-tuple]] but it's a 1-tuple
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_2x_from_1tup() {
    let result: Result<(((),),), _> = from_str("[10]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uGP:iP) Not unit grandparent --> int parent
/// Expect a 1-tuple[1-tuple[0-tuple]] but it's a 1-tuple[1-tuple]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_2x_from_1tup_nested() {
    let result: Result<(((),),), _> = from_str("[[10]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}

// (uGP:iGP) Not unit grandparent --> int grandparent
/// Expect a 1-tuple[1-tuple[0-tuple]] but it's a 1-tuple[1-tuple[1-tuple]]
#[cfg(not(miri))]
#[test]
fn test_deserialize_tuple_empty_nested_2x_from_1tup_nested_2x() {
    let result: Result<(((),),), _> = from_str("[[[10]]]");
    let err = result.unwrap_err();
    insta::assert_snapshot!(err);
}
