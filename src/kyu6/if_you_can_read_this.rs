/* 
Task
You'll have to translate a string to Pilot's alphabet (NATO phonetic alphabet).

Input:

If, you can read?

Output:

India Foxtrot , Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta ?

Note:

There are preloaded dictionary you can use, named NATO
The set of used punctuation is ,.!?.
Punctuation should be kept in your return string, but spaces should not.
Xray should not have a dash within.
Every word and punctuation mark should be separated by a space ' '.
There should be no trailing whitespace
 */

 use std::collections::HashMap;
 use once_cell::sync::Lazy;
 
 #[rustfmt::skip]
 pub static NATO: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
     [
         ('A', "Alfa"), ('B', "Bravo"), ('C', "Charlie"), ('D', "Delta"),
         ('E', "Echo"), ('F', "Foxtrot"), ('G', "Golf"), ('H', "Hotel"),
         ('I', "India"), ('J', "Juliett"), ('K', "Kilo"), ('L', "Lima"),
         ('M', "Mike"), ('N', "November"), ('O', "Oscar"), ('P', "Papa"),
         ('Q', "Quebec"), ('R', "Romeo"), ('S', "Sierra"), ('T', "Tango"),
         ('U', "Uniform"), ('V', "Victor"), ('W', "Whiskey"), ('X', "Xray"),
         ('Y', "Yankee"), ('Z', "Zulu"),
     ]
     .iter()
     .copied()
     .collect()       
 });

pub fn show_letters(character: char) {
    println!("{}", NATO[&character])
}

fn to_nato(words: &str) -> String {
    words.replace(" ", "").chars().into_iter().map(|c| match c.is_ascii_alphabetic() {
        true => format!("{} ",NATO[&c.to_ascii_uppercase()]),
        false => format!("{c} "),
    }).collect::<String>().trim().to_string()
}

#[test]
fn test_nato_dictionary() {
    assert_eq!(to_nato("If, you can read?"), "India Foxtrot , Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta ?")
}

#[test]
fn examples() {
    assert_eq!(
        to_nato("If you can read"),
        "India Foxtrot Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta"
    );
    
    assert_eq!(
        to_nato("Did not see that coming",),
        "Delta India Delta November Oscar Tango Sierra Echo Echo Tango Hotel Alfa Tango Charlie Oscar Mike India November Golf"
    );
    
    assert_eq!(
        to_nato("go for it!"),
        "Golf Oscar Foxtrot Oscar Romeo India Tango !"
    );
}