pub fn even_or_odd(n: i32) -> &'static str {
    match n % 2 {
        0 => "Even",
        _ => "Odd"
    }
}

fn if_else_even_or_odd(n: i32) -> &'static str {
    if n % 2 == 0 {"Even"} else {"Odd"}
}

#[test]
fn returns_expected() {
  assert_eq!(even_or_odd(0), "Even");
  assert_eq!(even_or_odd(2), "Even");
  assert_eq!(even_or_odd(1), "Odd");
  assert_eq!(even_or_odd(7), "Odd");
  assert_eq!(even_or_odd(-1), "Odd");
}