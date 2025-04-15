//! Tests for TOML values to maps.

use std::collections::HashMap;

use facet::Facet;

#[test]
fn test_scalar_map() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        values: HashMap<String, i32>,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("[values]").expect("Failed to parse TOML"),
        Root {
            values: HashMap::new()
        },
    );

    assert_eq!(
        facet_toml::from_str::<Root>(
            r#"
            [values]
            a = 0
            b = -1
            "#
        )
        .expect("Failed to parse TOML"),
        Root {
            values: [("a".to_string(), 0), ("b".to_string(), -1)].into()
        },
    );
}

#[test]
fn test_scalar_map_with_other_fields() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        values: HashMap<String, i32>,
        other: i32,
    }

    assert_eq!(
        facet_toml::from_str::<Root>(
            r#"
            other = 1
            [values]
            "#
        )
        .expect("Failed to parse TOML"),
        Root {
            values: HashMap::new(),
            other: 1,
        },
    );

    assert_eq!(
        facet_toml::from_str::<Root>(
            r#"
            other = 2
            [values]
            a = 0
            b = -1
            "#
        )
        .expect("Failed to parse TOML"),
        Root {
            values: [("a".to_string(), 0), ("b".to_string(), -1)].into(),
            other: 2,
        },
    );
}

#[test]
fn test_unit_struct_map() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        values: HashMap<String, Item>,
    }

    #[derive(Debug, Facet, PartialEq)]
    struct Item(bool);

    assert_eq!(
        facet_toml::from_str::<Root>("[values]").expect("Failed to parse TOML"),
        Root {
            values: HashMap::new()
        },
    );

    assert_eq!(
        facet_toml::from_str::<Root>(
            r#"
            values.a = true
            values.b = false
            "#
        )
        .expect("Failed to parse TOML"),
        Root {
            values: [
                ("a".to_string(), Item(true)),
                ("b".to_string(), Item(false))
            ]
            .into()
        },
    );
}

#[test]
fn test_struct_map() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        dependencies: HashMap<String, Dependency>,
    }

    #[derive(Debug, Facet, PartialEq)]
    struct Dependency {
        version: String,
        optional: bool,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("[dependencies]").expect("Failed to parse TOML"),
        Root {
            dependencies: HashMap::new()
        },
    );

    assert_eq!(
        facet_toml::from_str::<Root>(
            r#"
            [dependencies]
            syn = { version = "1", optional = false }
            paste = { version = "0.0.1", optional = true }
            "#
        )
        .expect("Failed to parse TOML"),
        Root {
            dependencies: [
                (
                    "syn".to_string(),
                    Dependency {
                        version: "1".to_string(),
                        optional: false,
                    }
                ),
                (
                    "paste".to_string(),
                    Dependency {
                        version: "0.0.1".to_string(),
                        optional: true,
                    }
                )
            ]
            .into()
        },
    );
}
