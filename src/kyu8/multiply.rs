pub fn multiply(a:i32, b:i32) -> i32 {
    a * b
  }
  
  #[test]
  fn test_multiply() {
      assert_eq!(multiply(3,5), 15)
  }