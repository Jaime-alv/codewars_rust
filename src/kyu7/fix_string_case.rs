/*
In this Kata, you will be given a string that may have mixed uppercase and lowercase
letters and your task is to convert that string to either lowercase only or uppercase only based on:
> make as few changes as possible.
> if the string contains equal number of uppercase and lowercase letters, convert the string to lowercase.

For example:

solve("coDe") = "code". Lowercase characters > uppercase. Change only the "D" to lowercase.
solve("CODe") = "CODE". Uppercase characters > lowercase. Change only the "e" to uppercase.
solve("coDE") = "code". Upper == lowercase. Change all to lowercase.
 */

pub(crate) fn fix_string_case(s: &str) -> String {
    let lowercase = s
        .as_bytes()
        .iter()
        .filter(|x| x.is_ascii_lowercase())
        .count();
    if lowercase >= (s.len() - lowercase) {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}

fn solve(s: &str) -> String {
    let (lowers, uppers): (Vec<char>, _) = s.chars().partition(|c| c.is_lowercase());
    
    match (lowers.len(), uppers.len()) {
        (a, b) if a >= b => s.to_lowercase(),
        _ => s.to_uppercase()
    }
}

#[test]
fn test_string_case() {
    assert_eq!(fix_string_case("coDe"), "code");
    assert_eq!(fix_string_case("CODe"), "CODE");
    assert_eq!(fix_string_case("coDE"), "code");
}
