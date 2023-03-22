/* 
Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) and mode 
(the value that occurs most often; a hash map will be helpful here) of the list. */

use std::collections::HashMap;

fn median_function(list: &[i32]) -> i32 {
    let mut sorted = list.to_owned();
    sorted.sort();
    list[(list.len()/2)]
}

pub fn mode_value(list: &[i32]) -> i32 {
    let mut counter = HashMap::<i32, u32>::new();
    list.to_owned().iter().for_each(|x| *counter.entry(x.to_owned()).or_insert(0) += 1);
    counter.into_iter().max_by_key(|x| x.1).unwrap().0
}

#[test]
fn check_median() {
    assert_eq!(median_function(&[1, 2, 3, 4, 5]), 3);
    assert_eq!(median_function(&[1, 2, 3, 4, 5, 6]), 4);
}

#[test]
fn check_mode_value() {
    assert_eq!(mode_value(&[0, 0, 0, 1, 2]), 0);
    assert_eq!(mode_value(&[0, 2, 0, 1, 2]), 2);
    assert_eq!(mode_value(&[0, 0, 0, 1, 2]), 0);
}