use std::mem::MaybeUninit;

use super::*;

use shapely::{Partial, Shapely};

#[test]
fn test_from_json() {
    #[derive(Shapely)]
    struct TestStruct {
        name: String,
        age: u64,
    }
    let json = r#"{"name": "Alice", "age": 30}"#;

    let mut test_struct = MaybeUninit::<TestStruct>::uninit();
    from_json(Partial::borrow(&mut test_struct), json).unwrap();
    let built_struct = unsafe { test_struct.assume_init() };
    assert_eq!(built_struct.name, "Alice");
    assert_eq!(built_struct.age, 30);
}

#[test]
fn test_to_json() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    impl Shapely for TestStruct {
        fn shape() -> shapely::Shape {
            shapely::Shape {
                name: |f| write!(f, "TestStruct"),
                layout: std::alloc::Layout::new::<Self>(),
                innards: shapely::Innards::Struct {
                    fields: shapely::struct_fields!(TestStruct, (name, age)),
                },
                set_to_default: None,
                drop_in_place: Some(|ptr| unsafe { std::ptr::drop_in_place(ptr as *mut Self) }),
                typeid: std::any::TypeId::of::<Self>(),
            }
        }
    }

    let test_struct = TestStruct {
        name: "Alice".to_string(),
        age: 30,
    };

    let expected_json = r#"{"name":"Alice","age":30}"#;
    let expected_json_indented = r#"{
  "name": "Alice",
  "age": 30
}"#;

    let mut buffer = Vec::new();
    to_json(
        &test_struct as *const _ as *mut u8,
        TestStruct::shape_desc(),
        &mut buffer,
        false,
    )
    .unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json);

    let mut buffer = Vec::new();
    to_json(
        &test_struct as *const _ as *mut u8,
        TestStruct::shape_desc(),
        &mut buffer,
        true,
    )
    .unwrap();
    let json_indented = String::from_utf8(buffer).unwrap();
    assert_eq!(json_indented, expected_json_indented.trim());
}
