use rust_template;

#[test]
fn test_add_integration() {
    // This tests the library as a user would see it
    assert_eq!(rust_template::add(1, 1), 2);
}