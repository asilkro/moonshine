use std;
use tokio;

#[test] // Test format
pub fn test__returns_true() {
    // For...
    let value_a: u8 = 34;

    // When...
    let value_b: u8 = 34;

    // Then...
    assert_eq!(value_a, value_b);
}

#[test]
pub fn test__returns_false() {
    // For...
    let value_a: u8 = 34;

    // When...
    let value_b: u8 = 34;

    // Then...
    assert_ne!(value_a, value_b);
}