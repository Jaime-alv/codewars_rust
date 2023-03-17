/*
What if we need the length of the words separated by a space to be added at the end of that same word and have it returned as an array?

Example(Input --> Output)

"apple ban" --> ["apple 5", "ban 3"]
"you will win" -->["you 3", "will 4", "win 3"]
Your task is to write a function that takes a String and returns an Array/list with the length of each word added to each element .

Note: String will have at least one element; words will always be separated by a space.
*/

pub fn add_length(s: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    s.split(" ")
        .map(|x| format!("{} {}", x, x.len()))
        .for_each(|f| { result.push(f) });
    result
}

fn add_length_solution(s: &str) -> Vec<String> {
    s.split_whitespace().map(|x| format!("{} {}", x, x.len())).collect()
 }

#[test]
fn test_add_length() {
    assert_eq!(add_length("apple ban"), vec!["apple 5", "ban 3"]);
    assert_eq!(add_length("you will win"), vec!["you 3", "will 4", "win 3"])
}
