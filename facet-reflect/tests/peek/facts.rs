use std::{cmp::Ordering, collections::HashSet};

use facet::Facet;
use facet_ansi::{ColorStyle, Style, Stylize as _};
use facet_reflect::{ConstValue, Wip};

fn check_facts<T>(val1: T, val2: T, expected_facts: HashSet<Fact>)
where
    T: Facet + 'static,
{
    let mut facts: HashSet<Fact> = HashSet::new();
    let name = format!("{}", T::SHAPE);

    eprint!("{}", format!("== {name}: ").yellow());
    let value_vtable = T::SHAPE.vtable;
    let traits = [
        ("Debug", value_vtable.debug.is_some()),
        ("Display", value_vtable.display.is_some()),
        ("Default", value_vtable.default_in_place.is_some()),
        ("Eq", value_vtable.eq.is_some()),
        ("Ord", value_vtable.ord.is_some()),
        ("Clone", value_vtable.clone_into.is_some()),
    ];
    let trait_str = traits
        .iter()
        .filter_map(|(name, has_impl)| {
            if *has_impl {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" + ");
    eprintln!("{} {}", trait_str, "======".yellow());

    let l = ConstValue::new(&val1);
    let r = ConstValue::new(&val2);

    let remarkable = Style::new().fg_blue();

    // Format display representation
    if l.shape().vtable.display.is_some() {
        facts.insert(Fact::Display);
        let display_str = format!("{} vs {}", l.style(remarkable), r.style(remarkable));
        eprintln!("Display:   {}", display_str);
    }

    // Format debug representation
    if l.shape().vtable.debug.is_some() {
        facts.insert(Fact::Debug);
        let debug_str = format!("{:?} vs {:?}", l.style(remarkable), r.style(remarkable));
        eprintln!("Debug:     {}", debug_str);
    }

    // Test equality
    if let Some(eq_result) = l.eq(&r) {
        facts.insert(Fact::EqualAnd { l_eq_r: eq_result });
        let eq_str = format!(
            "{:?} {} {:?}",
            l.style(remarkable),
            if eq_result { "==" } else { "!=" }.yellow(),
            r.style(remarkable),
        );
        eprintln!("Equality:  {}", eq_str);
    }

    // Test ordering
    if let Some(cmp_result) = l.partial_cmp(&r) {
        facts.insert(Fact::OrdAnd {
            l_ord_r: cmp_result,
        });
        let cmp_symbol = match cmp_result {
            Ordering::Less => "<",
            Ordering::Equal => "==",
            Ordering::Greater => ">",
        };
        let cmp_str = format!(
            "{:?} {} {:?}",
            l.style(remarkable),
            cmp_symbol.yellow(),
            r.style(remarkable),
        );
        eprintln!("Ordering:  {}", cmp_str);
    }

    // Test default_in_place
    if let Ok(wip) = Wip::alloc::<T>().put_default() {
        let val = wip.build().unwrap();
        facts.insert(Fact::Default);
        eprintln!("Default:   {}", format!("{:?}", val).style(remarkable));
    }

    // Test clone
    if l.shape().vtable.clone_into.is_some() {
        facts.insert(Fact::Clone);
        eprintln!("Clone:     Implemented");
    }

    let expected_minus_actual: HashSet<_> = expected_facts.difference(&facts).collect();
    let actual_minus_expected: HashSet<_> = facts.difference(&expected_facts).collect();

    assert!(
        expected_facts == facts,
        "{} for {}: ({:?} vs {:?})\n{}\n{}",
        "Facts mismatch".red().bold(),
        name.style(remarkable),
        l.red(),
        r.blue(),
        expected_minus_actual
            .iter()
            .map(|f| format!("- {}", f))
            .collect::<Vec<_>>()
            .join("\n")
            .yellow(),
        actual_minus_expected
            .iter()
            .map(|f| format!("+ {}", f))
            .collect::<Vec<_>>()
            .join("\n")
            .yellow(),
    );
}

#[derive(Default)]
pub struct FactBuilder {
    has_debug: bool,
    has_display: bool,
    has_equal_and: Option<bool>,
    has_ord_and: Option<Ordering>,
    has_default: bool,
    has_clone: bool,
}

impl FactBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn debug(mut self) -> Self {
        self.has_debug = true;
        self
    }

    fn display(mut self) -> Self {
        self.has_display = true;
        self
    }

    fn equal_and(mut self, l_eq_r: bool) -> Self {
        self.has_equal_and = Some(l_eq_r);
        self
    }

    fn ord_and(mut self, l_ord_r: Ordering) -> Self {
        self.has_ord_and = Some(l_ord_r);
        self
    }

    fn default(mut self) -> Self {
        self.has_default = true;
        self
    }

    fn clone(mut self) -> Self {
        self.has_clone = true;
        self
    }

    fn build(self) -> HashSet<Fact> {
        let mut facts = HashSet::new();
        if self.has_debug {
            facts.insert(Fact::Debug);
        }
        if self.has_display {
            facts.insert(Fact::Display);
        }
        if let Some(l_eq_r) = self.has_equal_and {
            facts.insert(Fact::EqualAnd { l_eq_r });
        }
        if let Some(l_ord_r) = self.has_ord_and {
            facts.insert(Fact::OrdAnd { l_ord_r });
        }
        if self.has_default {
            facts.insert(Fact::Default);
        }
        if self.has_clone {
            facts.insert(Fact::Clone);
        }
        facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Fact {
    Debug,
    Display,
    EqualAnd { l_eq_r: bool },
    OrdAnd { l_ord_r: Ordering },
    Default,
    Clone,
}

use core::fmt::{Display, Formatter, Result};

impl Display for Fact {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Fact::Debug => write!(f, "impl Debug"),
            Fact::Display => write!(f, "impl Display"),
            Fact::EqualAnd { l_eq_r } => write!(
                f,
                "impl Equal and l {} r",
                if *l_eq_r { "==" } else { "!=" }
            ),
            Fact::OrdAnd { l_ord_r } => {
                let ord_str = match l_ord_r {
                    Ordering::Less => "<",
                    Ordering::Equal => "==",
                    Ordering::Greater => ">",
                };
                write!(f, "impl Ord and l {} r", ord_str)
            }
            Fact::Default => write!(f, "impl Default"),
            Fact::Clone => write!(f, "impl Clone"),
        }
    }
}

#[test]
fn test_integer_traits() {
    facet_testhelpers::setup();

    // i32 implements Debug, PartialEq, and Ord
    check_facts(
        42,
        24,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .clone()
            .build(),
    );

    // Test equal i32 values
    check_facts(
        42,
        42,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .default()
            .clone()
            .build(),
    );

    // Test i32::MIN and i32::MAX
    check_facts(
        i32::MIN,
        i32::MAX,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );

    // Test i32 with 0
    check_facts(
        0,
        42,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );

    // Test negative i32 values
    check_facts(
        -10,
        10,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );
}

#[test]
fn test_boolean_traits() {
    facet_testhelpers::setup();

    // bool implements Debug, PartialEq, Ord, and Display
    check_facts(
        true,
        false,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .clone()
            .build(),
    );

    check_facts(
        true,
        true,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .default()
            .clone()
            .build(),
    );

    check_facts(
        false,
        true,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );

    check_facts(
        false,
        false,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .default()
            .clone()
            .build(),
    );
}

#[test]
fn test_floating_traits() {
    facet_testhelpers::setup();

    // f64 implements Debug, PartialEq
    check_facts(
        3.18,
        2.71,
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .clone()
            .build(),
    );
}

#[test]
fn test_string_traits() {
    facet_testhelpers::setup();

    // String implements Debug, PartialEq, and Ord
    check_facts(
        String::from("hello"),
        String::from("world"),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );

    // &str implements Debug, PartialEq, and Ord
    check_facts(
        "hello",
        "world",
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .default()
            .build(),
    );

    // Cow<'a, str> implements Debug, PartialEq, and Ord
    use std::borrow::Cow;
    check_facts(
        Cow::Borrowed("hello"),
        Cow::Borrowed("world"),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .default()
            .build(),
    );
    check_facts(
        Cow::Owned("hello".to_string()),
        Cow::Owned("world".to_string()),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .default()
            .build(),
    );
    check_facts(
        Cow::Borrowed("same"),
        Cow::Owned("same".to_string()),
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .clone()
            .default()
            .build(),
    );
}

#[test]
fn test_slice_traits() {
    facet_testhelpers::setup();

    // &[i32] implements Debug, PartialEq, and Ord
    check_facts(
        &[1, 2, 3][..],
        &[4, 5, 6][..],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .default()
            .build(),
    );

    // &[&str] implements Debug, PartialEq, and Ord
    check_facts(
        &["hello", "world"][..],
        &["foo", "bar"][..],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .clone()
            .default()
            .build(),
    );
}

#[test]
fn test_array_traits() {
    facet_testhelpers::setup();

    // [i32; 0] implements Debug, PartialEq, Ord, Default, and Clone
    check_facts::<[i32; 0]>(
        [],
        [],
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .default()
            .clone()
            .build(),
    );
    // [i32; 1] implements Debug, PartialEq, Ord, Default, and Clone
    check_facts(
        [42],
        [24],
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .default()
            .clone()
            .build(),
    );
    // [i32; 2] implements Debug, PartialEq, Ord, Default, and Clone
    check_facts(
        [1, 2],
        [1, 3],
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );
    // [i32; 33] implements Debug, PartialEq, Ord and Clone but not yet `Default`
    check_facts(
        [0; 33],
        [0; 33],
        FactBuilder::new()
            .debug()
            .display()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .clone()
            .build(),
    );

    // [&str; 1] implements Debug, PartialEq, Ord, Default, and Clone
    check_facts(
        ["hello"],
        ["world"],
        FactBuilder::new()
            .display()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .default()
            .clone()
            .build(),
    );
}

#[test]
fn test_vecs() {
    facet_testhelpers::setup();

    // Vec<i32> implements Debug, PartialEq, but not Ord
    check_facts(
        vec![1, 2, 3],
        vec![4, 5, 6],
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .default()
            .clone()
            .build(),
    );

    // // Vec<String> implements Debug, PartialEq, but not Ord
    // check_facts(
    //     vec!["hello".to_string(), "world".to_string()],
    //     vec!["foo".to_string(), "bar".to_string()],
    //     FactBuilder::new()
    //         .debug()
    //         .equal_and(false)
    //         .default()
    //         .clone()
    //         .build(),
    // );

    // // Two pairs of equal Vecs
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![1, 2, 3];
    // check_facts(
    //     vec1.clone(),
    //     vec2.clone(),
    //     FactBuilder::new()
    //         .debug()
    //         .equal_and(true)
    //         .default()
    //         .clone()
    //         .build(),
    // );

    // let vec3 = vec!["hello".to_string(), "world".to_string()];
    // let vec4 = vec!["hello".to_string(), "world".to_string()];
    // check_facts(
    //     vec3.clone(),
    //     vec4.clone(),
    //     FactBuilder::new()
    //         .debug()
    //         .equal_and(true)
    //         .default()
    //         .clone()
    //         .build(),
    // );
}

#[test]
fn test_hashmaps() {
    facet_testhelpers::setup();

    use std::collections::HashMap;

    // HashMap<String, i32> implements Debug, PartialEq, but not Ord
    let mut map1 = HashMap::new();
    map1.insert("key1".to_string(), 42);
    map1.insert("key2".to_string(), 24);

    let mut map2 = HashMap::new();
    map2.insert("key3".to_string(), 100);
    map2.insert("key4".to_string(), 200);

    check_facts(
        map1.clone(),
        map2.clone(),
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .default()
            .clone()
            .build(),
    );

    // Two pairs of equal HashMaps
    let mut map3 = HashMap::new();
    map3.insert("key1".to_string(), 10);
    map3.insert("key2".to_string(), 20);

    let mut map4 = HashMap::new();
    map4.insert("key1".to_string(), 10);
    map4.insert("key2".to_string(), 20);

    check_facts(
        map3.clone(),
        map4.clone(),
        FactBuilder::new()
            .debug()
            .equal_and(true)
            .default()
            .clone()
            .build(),
    );
}

#[test]
fn test_custom_structs() {
    facet_testhelpers::setup();

    // Struct with no trait implementations
    #[derive(Facet)]
    struct StructNoTraits {
        value: i32,
    }
    check_facts(
        StructNoTraits { value: 42 },
        StructNoTraits { value: 24 },
        FactBuilder::new().build(),
    );

    // Struct with Debug only
    #[derive(Facet, Debug)]
    struct StructDebug {
        value: i32,
    }
    check_facts(
        StructDebug { value: 42 },
        StructDebug { value: 24 },
        FactBuilder::new().debug().build(),
    );

    // Struct with Debug and PartialEq
    #[derive(Facet, Debug, PartialEq)]
    struct StructDebugEq {
        value: i32,
    }
    check_facts(
        StructDebugEq { value: 42 },
        StructDebugEq { value: 24 },
        FactBuilder::new().debug().equal_and(false).build(),
    );

    // Struct with all traits
    #[derive(Facet, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct StructAll {
        value: i32,
    }
    check_facts(
        StructAll { value: 42 },
        StructAll { value: 24 },
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .clone()
            .build(),
    );
    check_facts(
        StructAll { value: 10 },
        StructAll { value: 90 },
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .build(),
    );
    check_facts(
        StructAll { value: 69 },
        StructAll { value: 69 },
        FactBuilder::new()
            .debug()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .clone()
            .build(),
    );
}

#[test]
fn test_tuple_structs() {
    facet_testhelpers::setup();

    // Tuple struct with no trait implementations
    #[derive(Facet)]
    #[allow(dead_code)]
    struct TupleNoTraits(i32, String);
    check_facts(
        TupleNoTraits(42, "Hello".to_string()),
        TupleNoTraits(24, "World".to_string()),
        FactBuilder::new().build(),
    );

    // Tuple struct with Debug only
    #[derive(Facet, Debug)]
    #[allow(dead_code)]
    struct TupleDebug(i32, String);
    check_facts(
        TupleDebug(42, "Hello".to_string()),
        TupleDebug(24, "World".to_string()),
        FactBuilder::new().debug().build(),
    );

    // Tuple struct with EQ only
    #[derive(Facet, PartialEq)]
    struct TupleEq(i32, String);
    check_facts(
        TupleEq(42, "Hello".to_string()),
        TupleEq(24, "World".to_string()),
        FactBuilder::new().equal_and(false).build(),
    );

    // Tuple struct with all traits
    #[derive(Facet, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct TupleAll(i32, String);
    check_facts(
        TupleAll(42, "Hello".to_string()),
        TupleAll(24, "World".to_string()),
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .clone()
            .build(),
    );
}

#[test]
fn test_enums() {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    #[repr(u8)]
    enum TestEnum {
        Variant1,
        Variant2(i32),
        Variant3 { field: String },
    }

    // Unit variant with equal values
    check_facts(
        TestEnum::Variant1,
        TestEnum::Variant1,
        FactBuilder::new()
            .debug()
            .equal_and(true)
            .ord_and(Ordering::Equal)
            .clone()
            .build(),
    );

    // Tuple variant with different values
    check_facts(
        TestEnum::Variant2(42),
        TestEnum::Variant2(24),
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Greater)
            .clone()
            .build(),
    );

    // Struct variant with different values
    check_facts(
        TestEnum::Variant3 {
            field: "Hello".to_string(),
        },
        TestEnum::Variant3 {
            field: "World".to_string(),
        },
        FactBuilder::new()
            .debug()
            .equal_and(false)
            .ord_and(Ordering::Less)
            .clone()
            .build(),
    );
}
