use ctor::ctor;

#[ctor]
fn init_backtrace() {
    color_backtrace::install();
}

use shapely_derive::Shapely;
use shapely_json::{from_json, to_json};
use shapely_poke::{Peek, Poke};
use shapely_trait::Shapely;

use shapely_trait as shapely;

#[test]
fn test_from_json() {
    #[derive(Shapely)]
    struct TestStruct {
        name: String,
        age: u64,
    }
    let json = r#"{"name": "Alice", "age": 30}"#;

    let (poke, _guard) = Poke::alloc::<TestStruct>();
    let opaque = from_json(poke, json).unwrap();
    let s = unsafe { opaque.read::<TestStruct>() };

    assert_eq!(s.name, "Alice");
    assert_eq!(s.age, 30);
}

#[test]
fn test_to_json() {
    #[derive(Debug, PartialEq, Clone, Shapely)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    let test_struct = TestStruct {
        name: "Alice".to_string(),
        age: 30,
    };

    let _expected_json = r#"{"name":"Alice","age":30}"#;
    let expected_json_indented = r#"{
  "name": "Alice",
  "age": 30
}"#;

    let mut buffer = Vec::new();
    let peek = Peek::new(&test_struct);
    to_json(peek, &mut buffer, true).unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json_indented);

    // let mut buffer = Vec::new();
    // let (poke, _guard) = Poke::alloc::<TestStruct>();
    // unsafe { std::ptr::write(poke.as_mut_ptr(), test_struct.clone()) };
    // to_json(&poke, &mut buffer, true).unwrap();
    // let json_indented = String::from_utf8(buffer).unwrap();
    // assert_eq!(json_indented, expected_json_indented.trim());

    // // Test roundtrip
    // let (poke, _guard) = Poke::alloc::<TestStruct>();
    // from_json(poke, expected_json).unwrap();
    // let deserialized_struct = unsafe { (*poke.as_mut_ptr()).clone() };
    // assert_eq!(deserialized_struct, test_struct);
}

// #[test]
// fn test_from_json_with_more_types() {
//     #[derive(Shapely)]
//     struct TestStructWithMoreTypes {
//         u8_val: u8,
//         u16_val: u16,
//         i8_val: i8,
//         i16_val: i16,
//         u32_val: u32,
//         i32_val: i32,
//         u64_val: u64,
//         i64_val: i64,
//         f32_val: f32,
//         f64_val: f64,
//     }

//     let json = r#"{
//         "u8_val": 255,
//         "u16_val": 65535,
//         "i8_val": -128,
//         "i16_val": -32768,
//         "u32_val": 4294967295,
//         "i32_val": -2147483648,
//         "u64_val": 18446744073709551615,
//         "i64_val": -9223372036854775808,
//         "f32_val": 3.141592653589793,
//         "f64_val": 3.141592653589793
//     }"#;

//     let mut test_struct = TestStructWithMoreTypes::partial();
//     from_json(&mut test_struct, json).unwrap();

//     let built_struct = test_struct.build::<TestStructWithMoreTypes>();
//     assert_eq!(built_struct.u8_val, 255);
//     assert_eq!(built_struct.u16_val, 65535);
//     assert_eq!(built_struct.i8_val, -128);
//     assert_eq!(built_struct.i16_val, -32768);
//     assert_eq!(built_struct.u32_val, 4294967295);
//     assert_eq!(built_struct.i32_val, -2147483648);
//     assert_eq!(built_struct.u64_val, 18446744073709551615);
//     assert_eq!(built_struct.i64_val, -9223372036854775808);
//     assert!((built_struct.f32_val - std::f32::consts::PI).abs() < f32::EPSILON);
//     assert!((built_struct.f64_val - std::f64::consts::PI).abs() < f64::EPSILON);
// }

// #[test]
// fn test_from_json_with_nested_structs() {
//     #[derive(Shapely)]
//     struct InnerStruct {
//         value: i32,
//     }

//     #[derive(Shapely)]
//     struct OuterStruct {
//         name: String,
//         inner: InnerStruct,
//     }

//     let json = r#"{
//         "name": "Outer",
//         "inner": {
//             "value": 42
//         }
//     }"#;

//     let mut test_struct = OuterStruct::partial();
//     from_json(&mut test_struct, json).unwrap();

//     let built_struct = test_struct.build::<OuterStruct>();
//     assert_eq!(built_struct.name, "Outer");
//     assert_eq!(built_struct.inner.value, 42);
// }

// #[test]
// fn test_from_json_with_tuples() {
//     #[derive(Shapely)]
//     struct TupleStruct(i32, String, (f64, bool));

//     let json = r#"[123, "Hello", [3.69, true]]"#;

//     let mut test_struct = TupleStruct::partial();
//     from_json(&mut test_struct, json).unwrap();

//     let built_struct = test_struct.build::<TupleStruct>();
//     assert_eq!(built_struct.0, 123);
//     assert_eq!(built_struct.1, "Hello");
//     assert!((built_struct.2.0 - 3.69).abs() < f64::EPSILON);
//     assert!(built_struct.2.1);
// }

// #[test]
// fn test_from_json_with_vec() {
//     #[derive(Shapely, Debug, PartialEq)]
//     struct VecStruct {
//         numbers: Vec<i32>,
//         names: Vec<String>,
//     }

//     let json = r#"{
//         "numbers": [1, 2, 3, 4, 5],
//         "names": ["Alice", "Bob", "Charlie"]
//     }"#;

//     // Deserialize
//     let mut test_struct = VecStruct::partial();
//     from_json(&mut test_struct, json).unwrap();
//     let built_struct = test_struct.build::<VecStruct>();

//     // Verify deserialization
//     assert_eq!(built_struct.numbers, vec![1, 2, 3, 4, 5]);
//     assert_eq!(built_struct.names, vec!["Alice", "Bob", "Charlie"]);

//     // Serialize
//     let mut buffer = Vec::new();
//     to_json(
//         &built_struct as *const _ as *mut u8,
//         VecStruct::SHAPE_FN,
//         &mut buffer,
//         true,
//     )
//     .unwrap();
//     let serialized_json = String::from_utf8(buffer).unwrap();

//     // Print the serialized JSON
//     eprintln!("Serialized JSON:\n{}", serialized_json);

//     // Round-trip: deserialize the serialized JSON
//     let mut round_trip_struct = VecStruct::partial();
//     from_json(&mut round_trip_struct, &serialized_json).unwrap();
//     let round_trip_built = round_trip_struct.build::<VecStruct>();

//     // Verify round-trip
//     assert_eq!(round_trip_built, built_struct);
// }

// #[test]
// fn test_from_json_with_hashmap() {
//     #[derive(Shapely, Debug, PartialEq)]
//     struct OtherStruct {
//         value: i32,
//         name: String,
//     }

//     #[derive(Shapely, Debug, PartialEq)]
//     struct HashmapStruct {
//         data: std::collections::HashMap<String, OtherStruct>,
//     }

//     let json = r#"{
//         "data": {
//             "first": {
//                 "value": 42,
//                 "name": "First Item"
//             },
//             "second": {
//                 "value": 84,
//                 "name": "Second Item"
//             },
//             "third": {
//                 "value": 126,
//                 "name": "Third Item"
//             }
//         }
//     }"#;

//     // Deserialize
//     let mut test_struct = HashmapStruct::partial();
//     from_json(&mut test_struct, json).unwrap();
//     let built_struct = test_struct.build::<HashmapStruct>();

//     // Verify deserialization
//     assert_eq!(built_struct.data.len(), 3);
//     assert_eq!(built_struct.data.get("first").unwrap().value, 42);
//     assert_eq!(built_struct.data.get("first").unwrap().name, "First Item");
//     assert_eq!(built_struct.data.get("second").unwrap().value, 84);
//     assert_eq!(built_struct.data.get("second").unwrap().name, "Second Item");
//     assert_eq!(built_struct.data.get("third").unwrap().value, 126);
//     assert_eq!(built_struct.data.get("third").unwrap().name, "Third Item");

//     // Serialize
//     let mut buffer = Vec::new();
//     to_json(
//         &built_struct as *const _ as *mut u8,
//         HashmapStruct::SHAPE_FN,
//         &mut buffer,
//         true,
//     )
//     .unwrap();
//     let serialized_json = String::from_utf8(buffer).unwrap();

//     // Print the serialized JSON
//     eprintln!("Serialized JSON:\n{}", serialized_json);

//     // Round-trip: deserialize the serialized JSON
//     let mut round_trip_struct = HashmapStruct::partial();
//     from_json(&mut round_trip_struct, &serialized_json).unwrap();
//     let round_trip_built = round_trip_struct.build::<HashmapStruct>();

//     // Verify round-trip
//     assert_eq!(round_trip_built, built_struct);
// }
