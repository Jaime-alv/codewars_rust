pub fn parse_boolean(b: bool) -> String {
    String::from(if b {"true"} else {"false"})
}

#[test]
fn test_boolean() {
    assert_eq!(parse_boolean(true), "true");
    assert_eq!(parse_boolean(false), "false");
}