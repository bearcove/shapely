use std::hash::{DefaultHasher, Hash, Hasher};

use facet_reflect::Peek;

#[test]
fn test_peek_value_twoints() {
    facet_testhelpers::setup();

    let a = 42_i32;
    let b = 42_i32;

    let av = Peek::new(&a);
    let bv = Peek::new(&b);

    assert_eq!(av, bv);
    assert_eq!(av.to_string(), "42");

    let mut h = DefaultHasher::new();
    a.hash(&mut h);
    let h1 = h.finish();

    let mut h = DefaultHasher::new();
    av.hash(&mut h);
    let h2 = h.finish();

    assert_eq!(h1, h2);
}

#[test]
fn test_peek_value_twostrings() {
    facet_testhelpers::setup();

    let a = Some(42_i32);
    let av = Peek::new(&a);

    assert_eq!(av.to_string(), "⟨Option<i32>⟩");
    assert_eq!(format!("{a:?}"), format!("{av:?}"));
}
