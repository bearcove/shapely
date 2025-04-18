use facet::Facet;
use facet_reflect::Wip;

#[derive(Facet, PartialEq, Eq, Debug)]
struct Outer {
    name: String,
    inner: Inner,
}

#[derive(Facet, PartialEq, Eq, Debug)]
struct Inner {
    x: i32,
    b: i32,
}

#[test]
fn wip_nested() -> eyre::Result<()> {
    facet_testhelpers::setup();

    let v = Wip::alloc::<Outer>()
        .field_named("name")?
        .put(String::from("Hello, world!"))?
        .pop()?
        .field_named("inner")?
        .field_named("x")?
        .put(42)?
        .pop()?
        .field_named("b")?
        .put(43)?
        .pop()?
        .pop()?
        .build()?
        .materialize::<Outer>()?;

    assert_eq!(
        v,
        Outer {
            name: String::from("Hello, world!"),
            inner: Inner { x: 42, b: 43 }
        }
    );

    Ok(())
}

#[test]
fn wip_nested_out_of_order() -> eyre::Result<()> {
    facet_testhelpers::setup();

    let v = Wip::alloc::<Outer>()
        .field_named("inner")?
        .field_named("x")?
        .put(42)?
        .pop()?
        .pop()?
        .field_named("name")?
        .put(String::from("Hello, world!"))?
        .pop()?
        .field_named("inner")?
        .field_named("b")?
        .put(43)?
        .pop()?
        .pop()?
        .build()?
        .materialize::<Outer>()?;

    assert_eq!(
        v,
        Outer {
            name: String::from("Hello, world!"),
            inner: Inner { x: 42, b: 43 }
        }
    );

    Ok(())
}

#[test]
fn readme_sample() -> eyre::Result<()> {
    facet_testhelpers::setup();

    use facet::Facet;

    #[derive(Debug, PartialEq, Eq, Facet)]
    struct FooBar {
        foo: u64,
        bar: String,
    }

    let foo_bar = Wip::alloc::<FooBar>()
        .field_named("foo")?
        .put(42u64)?
        .pop()?
        .field_named("bar")?
        .put(String::from("Hello, World!"))?
        .pop()?
        .build()?
        .materialize::<FooBar>()?;

    // Now we can use the constructed value
    println!("{}", foo_bar.bar);

    Ok(())
}

// Enum tests

#[derive(Facet, PartialEq, Eq, Debug)]
#[repr(u8)]
enum SimpleEnum {
    A,
    B,
    #[expect(dead_code)]
    C,
}

#[test]
fn wip_unit_enum() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test unit variant A
    let a = Wip::alloc::<SimpleEnum>()
        .variant_named("A")?
        .build()?
        .materialize::<SimpleEnum>()?;
    assert_eq!(a, SimpleEnum::A);

    // Test unit variant B
    let b = Wip::alloc::<SimpleEnum>()
        .variant(1)? // B is at index 1
        .build()?
        .materialize::<SimpleEnum>()?;
    assert_eq!(b, SimpleEnum::B);

    Ok(())
}

#[derive(Facet, PartialEq, Eq, Debug)]
#[repr(u8)]
enum EnumWithData {
    Empty,
    Single(i32),
    Tuple(i32, String),
    Struct { x: i32, y: String },
}

#[test]
fn wip_enum_with_data() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test empty variant
    let empty = Wip::alloc::<EnumWithData>()
        .variant_named("Empty")?
        .build()?
        .materialize::<EnumWithData>()?;
    assert_eq!(empty, EnumWithData::Empty);

    // Test single-field tuple variant
    let single = Wip::alloc::<EnumWithData>()
        .variant_named("Single")?
        .field(0)? // Access the first field
        .put(42)?
        .pop()?
        .build()?
        .materialize::<EnumWithData>()?;
    assert_eq!(single, EnumWithData::Single(42));

    // Test multi-field tuple variant
    let tuple = Wip::alloc::<EnumWithData>()
        .variant_named("Tuple")?
        .field(0)?
        .put(42)?
        .pop()?
        .field(1)?
        .put(String::from("Hello"))?
        .pop()?
        .build()?
        .materialize::<EnumWithData>()?;
    assert_eq!(tuple, EnumWithData::Tuple(42, String::from("Hello")));

    // Test struct variant
    let struct_variant = Wip::alloc::<EnumWithData>()
        .variant_named("Struct")?
        .field_named("x")?
        .put(42)?
        .pop()?
        .field_named("y")?
        .put(String::from("World"))?
        .pop()?
        .build()?
        .materialize::<EnumWithData>()?;
    assert_eq!(
        struct_variant,
        EnumWithData::Struct {
            x: 42,
            y: String::from("World")
        }
    );

    Ok(())
}

#[derive(Facet, PartialEq, Eq, Debug)]
#[repr(C)]
enum EnumWithDataReprC {
    Empty,
    Single(i32),
    Tuple(i32, String),
    Struct { x: i32, y: String },
}

#[test]
fn wip_enum_with_data_repr_c() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test empty variant
    let empty = Wip::alloc::<EnumWithDataReprC>()
        .variant_named("Empty")?
        .build()?
        .materialize::<EnumWithDataReprC>()?;
    assert_eq!(empty, EnumWithDataReprC::Empty);

    // Test single-field tuple variant
    let single = Wip::alloc::<EnumWithDataReprC>()
        .variant_named("Single")?
        .field(0)? // Access the first field
        .put(42)?
        .pop()?
        .build()?
        .materialize::<EnumWithDataReprC>()?;
    assert_eq!(single, EnumWithDataReprC::Single(42));

    // Test multi-field tuple variant
    let tuple = Wip::alloc::<EnumWithDataReprC>()
        .variant_named("Tuple")?
        .field(0)?
        .put(42)?
        .pop()?
        .field(1)?
        .put(String::from("Hello"))?
        .pop()?
        .build()?
        .materialize::<EnumWithDataReprC>()?;
    assert_eq!(tuple, EnumWithDataReprC::Tuple(42, String::from("Hello")));

    // Test struct variant
    let struct_variant = Wip::alloc::<EnumWithDataReprC>()
        .variant_named("Struct")?
        .field_named("x")?
        .put(42)?
        .pop()?
        .field_named("y")?
        .put(String::from("World"))?
        .pop()?
        .build()?
        .materialize::<EnumWithDataReprC>()?;
    assert_eq!(
        struct_variant,
        EnumWithDataReprC::Struct {
            x: 42,
            y: String::from("World")
        }
    );

    Ok(())
}

#[test]
fn wip_enum_error_cases() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test error: trying to access a field without selecting a variant
    let result = Wip::alloc::<EnumWithData>().field_named("x");
    assert!(result.is_err());

    // Test error: trying to select a non-existent variant
    let result = Wip::alloc::<EnumWithData>().variant_named("NonExistent");
    assert!(result.is_err());

    // Test error: trying to access a non-existent field in a variant
    let result = Wip::alloc::<EnumWithData>()
        .variant_named("Struct")?
        .field_named("non_existent");
    assert!(result.is_err());

    // Test error: trying to build without initializing all fields
    let result = Wip::alloc::<EnumWithData>()
        .variant_named("Struct")?
        .field_named("x")?
        .put(42)?
        .pop()?
        .build();
    assert!(result.is_err());

    Ok(())
}

// We've already tested enum functionality with SimpleEnum and EnumWithData,
// so we'll skip additional representation tests

#[test]
fn wip_switch_enum_variant() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test switching variants
    let result = Wip::alloc::<EnumWithData>()
        .variant_named("Single")?
        .field(0)?
        .put(42)?
        .pop()?
        .variant_named("Tuple")? // Switch to another variant
        .field(0)?
        .put(43)?
        .pop()?
        .field(1)?
        .put(String::from("Changed"))?
        .pop()?
        .build()?
        .materialize::<EnumWithData>()?;

    assert_eq!(result, EnumWithData::Tuple(43, String::from("Changed")));

    Ok(())
}

// List tests

#[test]
fn wip_empty_list() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Create an empty list with put_empty_list
    let empty_list = Wip::alloc::<Vec<i32>>()
        .put_empty_list()?
        .build()?
        .materialize::<Vec<i32>>()?;

    assert_eq!(empty_list, Vec::<i32>::new());
    assert_eq!(empty_list.len(), 0);

    Ok(())
}

#[test]
fn wip_list_push() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Build a vector by pushing elements one by one
    let list = Wip::alloc::<Vec<i32>>()
        .begin_pushback()?
        .push()?
        .put(10)?
        .pop()?
        .push()?
        .put(20)?
        .pop()?
        .push()?
        .put(30)?
        .pop()?
        .build()?
        .materialize::<Vec<i32>>()?;

    assert_eq!(list, vec![10, 20, 30]);
    assert_eq!(list.len(), 3);

    Ok(())
}

#[test]
fn wip_list_string() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Build a vector of strings
    let list = Wip::alloc::<Vec<String>>()
        .begin_pushback()?
        .push()?
        .put("hello".to_string())?
        .pop()?
        .push()?
        .put("world".to_string())?
        .pop()?
        .build()?
        .materialize::<Vec<String>>()?;

    assert_eq!(list, vec!["hello".to_string(), "world".to_string()]);

    Ok(())
}

#[derive(Facet, Debug, PartialEq)]
struct WithList {
    name: String,
    values: Vec<i32>,
}

#[test]
fn wip_struct_with_list() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Create a struct that contains a list
    let with_list = Wip::alloc::<WithList>()
        .field_named("name")?
        .put("test list".to_string())?
        .pop()?
        .field_named("values")?
        .begin_pushback()?
        .push()?
        .put(42)?
        .pop()?
        .push()?
        .put(43)?
        .pop()?
        .push()?
        .put(44)?
        .pop()?
        .pop()?
        .build()?
        .materialize::<WithList>()?;

    assert_eq!(
        with_list,
        WithList {
            name: "test list".to_string(),
            values: vec![42, 43, 44]
        }
    );

    Ok(())
}

#[test]
fn wip_list_error_cases() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test error: trying to push to a non-list type
    let result = Wip::alloc::<i32>().push();
    assert!(result.is_err());

    // Test error: trying to get element shape from non-list
    let result = Wip::alloc::<String>().element_shape();
    assert!(result.is_err());

    // Test error: trying to put_empty_list on non-list type
    let result = Wip::alloc::<i32>().put_empty_list();
    assert!(result.is_err());

    Ok(())
}

#[test]
fn wip_opaque_arc() -> eyre::Result<()> {
    facet_testhelpers::setup();

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct NotDerivingFacet(u64);

    #[derive(Facet)]
    pub struct Handle(#[facet(opaque)] std::sync::Arc<NotDerivingFacet>);

    #[derive(Facet)]
    pub struct Container {
        inner: Handle,
    }

    let result = Wip::alloc::<Container>()
        .field_named("inner")?
        .put(Handle(std::sync::Arc::new(NotDerivingFacet(35))))?
        .pop()?
        .build()?
        .materialize::<Container>()?;

    assert_eq!(*result.inner.0, NotDerivingFacet(35));

    Ok(())
}

#[test]
fn wip_put_option_explicit_some() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test switching variants
    let result = Wip::alloc::<Option<u64>>()
        .put::<Option<u64>>(Some(42))?
        .build()?
        .materialize::<Option<u64>>()?;

    assert_eq!(result, Some(42));

    Ok(())
}

#[test]
fn wip_put_option_explicit_none() -> eyre::Result<()> {
    facet_testhelpers::setup();

    let result = Wip::alloc::<Option<u64>>()
        .put::<Option<u64>>(None)?
        .build()?
        .materialize::<Option<u64>>()?;

    assert_eq!(result, None);

    Ok(())
}

#[test]
fn wip_put_option_implicit_some() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test switching variants
    let result = Wip::alloc::<Option<u64>>()
        .put::<u64>(42)?
        .build()?
        .materialize::<Option<u64>>()?;

    assert_eq!(result, Some(42));

    Ok(())
}

#[test]
fn wip_parse_option() -> eyre::Result<()> {
    facet_testhelpers::setup();

    // Test switching variants
    let result = Wip::alloc::<Option<f64>>()
        .parse("8.13")?
        .build()?
        .materialize::<Option<f64>>()?;

    assert_eq!(result, Some(8.13));

    Ok(())
}

#[test]
fn wip_option_explicit_some_through_push_some() -> eyre::Result<()> {
    #[derive(Facet, Debug, PartialEq, Eq)]
    struct Foo {
        foo: u32,
    }

    facet_testhelpers::setup();

    // Test switching variants
    let result = Wip::alloc::<Option<Foo>>()
        .push_some()?
        .field_named("foo")?
        .put::<u32>(42)?
        .pop()?
        .build()?
        .materialize::<Option<Foo>>()?;

    assert_eq!(result, Some(Foo { foo: 42 }));

    Ok(())
}
