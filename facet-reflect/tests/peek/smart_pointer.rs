use std::sync::Arc;
use facet::Facet;
use facet_reflect::PeekValue;

#[test]
fn test_peek_arc() {
    let source = Arc::new(42);
    let peek_value = PeekValue::new(&source);
    
    // First test we can convert to a smart pointer
    let peek_smart_pointer = peek_value.into_smart_pointer().unwrap();
    
    // Get the definition
    let def = peek_smart_pointer.def();
    
    // Verify the inner type is correct
    assert_eq!(def.t, i32::SHAPE);
}

#[test]
fn test_peek_arc_with_string() {
    let source = Arc::new("Hello, world!".to_string());
    let peek_value = PeekValue::new(&source);
    
    // Convert to a smart pointer
    let peek_smart_pointer = peek_value.into_smart_pointer().unwrap();
    
    // Get the definition
    let def = peek_smart_pointer.def();
    
    // Verify the inner type is correct
    assert_eq!(def.t, String::SHAPE);
}

#[test]
fn test_peek_arc_in_struct() {
    #[derive(Facet)]
    struct TestStruct {
        data: Arc<String>,
    }

    let source = TestStruct {
        data: Arc::new("Hello, world!".to_string()),
    };

    // First get the struct field
    let peek_value = PeekValue::new(&source);
    let peek_struct = peek_value.into_struct().unwrap();
    let peek_data = peek_struct.field_by_name("data").unwrap();
    
    // Then convert to a smart pointer
    let peek_smart_pointer = peek_data.into_smart_pointer().unwrap();
    
    // Verify the definition has the right flags
    let def = peek_smart_pointer.def();
    assert!(def.flags.contains(facet_core::SmartPointerFlags::ATOMIC));
    
    // Verify inner type is String
    assert_eq!(def.t, String::SHAPE);
}

#[test]
fn test_peek_arc_in_vec() {
    let source = vec![Arc::new(1), Arc::new(2), Arc::new(3)];
    let peek_value = PeekValue::new(&source);
    let peek_list = peek_value.into_list().unwrap();

    assert_eq!(peek_list.len(), 3);

    for item in peek_list.iter() {
        let peek_smart_pointer = item.into_smart_pointer().unwrap();
        
        // Test definition
        let def = peek_smart_pointer.def();
        assert_eq!(def.t, i32::SHAPE);
        assert!(def.flags.contains(facet_core::SmartPointerFlags::ATOMIC));
    }
}

#[test]
fn test_smart_pointer_flags() {
    let source = Arc::new(42);
    let peek_value = PeekValue::new(&source);
    let peek_smart_pointer = peek_value.into_smart_pointer().unwrap();
    
    // Test the definition has the expected flags
    let def = peek_smart_pointer.def();
    assert!(def.flags.contains(facet_core::SmartPointerFlags::ATOMIC));
    assert!(!def.flags.contains(facet_core::SmartPointerFlags::WEAK));
    assert!(!def.flags.contains(facet_core::SmartPointerFlags::LOCK));
    
    // Test known smart pointer type if available
    if let Some(known_type) = def.known {
        assert_eq!(known_type, facet_core::KnownSmartPointer::Arc);
    }
}