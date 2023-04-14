use std::{ops::Index, collections::HashMap};

/*
* For a given string s find the character c (or C)
* with longest consecutive repetition and return:
* Some((c, l))
* where l (or L) is the length of the repetition. If there are two or more characters with the same l return the first in order of appearance.

* For empty string return:

* None
* Happy coding! :)
*/
pub fn longest_repetition(s: &str) -> Option<(char, u32)> {
    let mut counter: HashMap<char, u32> = HashMap::new();
    let piece= s.as_bytes();
    (1..piece.len()).for_each(|i| {
        if piece[i] == piece[i - 1] {
            counter.entry(piece[i] as char).and_modify(|counter| *counter += 1).or_insert(2);
        }
    });
    counter.into_iter().max_by_key(|x| x.1)
}

fn counter_letters(s: &str) {
    let mut letters = HashMap::new();
    s.chars().for_each(|ch| {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
});
}


#[test]
fn test_longest_repetition() {
    assert_eq!(Some(('A', 4)), longest_repetition("baaabbecAAAA"))
}