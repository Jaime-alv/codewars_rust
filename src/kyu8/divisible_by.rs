pub fn is_divisible(n: u32, x: u32, y: u32) -> bool {
    n % x == 0 && n % y == 0
}

#[test]
fn test_divisible_by() {
    assert!(is_divisible(3, 1, 3));
    assert!(!is_divisible(100, 5, 3))
}

#[test]
fn basic_tests() {
    assert_eq!(is_divisible(3, 3, 4), false);
    assert_eq!(is_divisible(12, 3, 4), true);
    assert_eq!(is_divisible(8, 3, 4), false);
    assert_eq!(is_divisible(48, 3, 4), true);
    assert_eq!(is_divisible(100, 5, 10), true);
    assert_eq!(is_divisible(100, 5, 3), false);
    assert_eq!(is_divisible(4, 4, 2), true);
    assert_eq!(is_divisible(5, 2, 3), false);
    assert_eq!(is_divisible(17, 17, 17), true);
    assert_eq!(is_divisible(17, 1, 17), true);
}