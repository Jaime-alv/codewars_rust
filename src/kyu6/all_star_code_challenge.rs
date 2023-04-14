/*
Your family runs a shop and have just brought a Scrolling Text Machine
(http://3.imimg.com/data3/RP/IP/MY-2369478/l-e-d-multicolour-text-board-250x250.jpg)
to help get some more business.
The scroller works by replacing the current text string with a similar text string,
but with the first letter shifted to the end; this simulates movement.

You're father is far too busy with the business to worry about such details,
so, naturally, he's making you come up with the text strings.

Create a function named rotate() that accepts a string argument and returns
an array of strings with each letter from the input string being rotated to the end.

rotate("Hello") // => ["elloH", "lloHe", "loHel", "oHell", "Hello"]

! Note:
The original string should be included in the output array. The order matters.
Each element of the output array should be the rotated version of the previous element.
The output array SHOULD be the same length as the input string.
The function should return an empty array with a 0 length string, '', as input
 */

pub fn rotate(s: &str) -> Vec<String> {
    helper(s, Vec::new(), s.len())
}

fn helper(word: &str, mut tmp: Vec<String>, i: usize) -> Vec<String> {
    if i != 0 {
        let new: String = last_char(word);
        tmp.push(new.to_string());
        helper(&new, tmp, i - 1)
    } else {
        tmp
    }
}

fn last_char(word: &str) -> String {
    let first: char = word.chars().next().unwrap();
    let mut result = word.to_string()[1..word.len()].to_string();
    result.push(first);
    result
}

fn rotate_one_liner(s: &str) -> Vec<String> {
    (1..s.len() + 1)
        .map(|i| s[i..].to_string() + &s[..i])
        .collect()
}

#[cfg(test)]
mod test_all_star_code {
    use super::rotate;

    #[test]
    fn basic_test() {
        assert_eq!(rotate("abc").len(), 3);
        assert_eq!(rotate("").len(), 0);
    }

    #[test]
    fn test_order() {
        assert_eq!(rotate("abc"), vec!["bca", "cab", "abc"]);
    }

    #[test]
    fn test_kata_example() {
        assert_eq!(rotate("Hello").len(), "Hello".len());
        assert_eq!(
            rotate("Hello"),
            vec!["elloH", "lloHe", "loHel", "oHell", "Hello"]
        )
    }
}
