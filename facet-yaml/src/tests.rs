use super::*;
use facet::Facet;

#[derive(Debug, Facet, PartialEq)]
struct Person {
    name: String,
    age: u64,
}

#[test]
fn test_deserialize_person() {
    let yaml = r#"
            name: Alice
            age: 30
        "#;

    let person: Person = from_str(yaml).expect("Failed to parse YAML");
    assert_eq!(
        person,
        Person {
            name: "Alice".to_string(),
            age: 30
        }
    );
}
