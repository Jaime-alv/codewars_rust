

/* 
The two oldest ages function/method needs to be completed.
It should take an array of numbers as its argument and return the two
highest numbers within the array. The returned value should be an array
in the format [second oldest age,  oldest age].

The order of the numbers passed in could be any order.
The array will always include at least 2 items.
If there are two or more oldest age, then return both of them in array format.

For example (Input --> Output):

[1, 2, 10, 8] --> [8, 10]
[1, 5, 87, 45, 8, 8] --> [45, 87]
[1, 3, 10, 0]) --> [3, 10]
 */
fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let mut result: [u8; 2] = [0,0];
    let mut ages_sort = ages.to_owned();
    ages_sort.sort();
    ages_sort.reverse();
    result[0] = ages_sort[1];
    result[1] = ages_sort[0];
    result
    
}

#[test]
fn test_two_oldest_ages() {
    assert_eq!(two_oldest_ages(&[1, 5, 87, 45, 8, 8]), [45, 87]);
    assert_eq!(two_oldest_ages(&[6, 5, 83, 5, 3, 18]), [18, 83]);
    assert_eq!(two_oldest_ages(&[10, 1]), [1, 10]);
    assert_eq!(two_oldest_ages(&[1, 3, 10, 0]), [3, 10]);
}