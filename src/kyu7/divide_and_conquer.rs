use either::Either;
/*
Given a mixed array of number and string representations of integers,
add up the non-string integers and subtract the total of the string integers.

Return as a number.
 */

enum Mixed<'a> {
    Number(i32),
    Text(&'a str),
}

fn divide_and_conquer(list: &[Mixed]) -> i32 {
    let mut result: i32 = 0;
    list.to_owned().iter().for_each(|i| match i {
        Mixed::Number(x) => result += x,
        Mixed::Text(y) => result -= y.parse::<i32>().unwrap(),
    });
    result
}

fn div_con(arr: &[Either<i32, String>]) -> i32 {
    arr.iter().fold(0, |acc, x| match x {
        Either::Left(number) => acc + number,
        Either::Right(text) => acc - text.parse::<i32>().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local() {
        let local1: &[Mixed] = &[
            Mixed::Number(0),
            Mixed::Text("1"),
            Mixed::Number(2),
            Mixed::Text("2"),
        ];
        let local2: &[Mixed] = &[
            Mixed::Number(4),
            Mixed::Text("1"),
            Mixed::Number(2),
            Mixed::Text("2"),
        ];
        assert_eq!(divide_and_conquer(local1), -1);
        assert_eq!(divide_and_conquer(local2), 3);
    }

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = div_con(arr);
        assert!(
            actual == expected,
            "With arr = {arr:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                either::Left(9),
                either::Left(3),
                either::Right("7".to_string()),
                either::Right("3".to_string()),
            ],
            2,
        );
        dotest(
            &[
                Either::Right("5".to_string()),
                Either::Right("0".to_string().to_string()),
                Either::Left(9),
                Either::Left(3),
                Either::Left(2),
                Either::Left(1),
                Either::Right("9".to_string()),
                Either::Left(6),
                Either::Left(7),
            ],
            14,
        );
        dotest(
            &[
                Either::Right("3".to_string()),
                Either::Left(6),
                Either::Left(6),
                Either::Left(0),
                Either::Right("5".to_string()),
                Either::Left(8),
                Either::Left(5),
                Either::Right("6".to_string()),
                Either::Left(2),
                Either::Right("0".to_string()),
            ],
            13,
        );
        dotest(
            &[
                Either::Right("1".to_string()),
                Either::Right("5".to_string()),
                Either::Right("8".to_string()),
                Either::Left(8),
                Either::Left(9),
                Either::Left(9),
                Either::Left(2),
                Either::Right("3".to_string()),
            ],
            11,
        );
        dotest(
            &[
                Either::Left(8),
                Either::Left(0),
                Either::Left(0),
                Either::Left(8),
                Either::Left(5),
                Either::Left(7),
                Either::Left(2),
                Either::Left(3),
                Either::Left(7),
                Either::Left(8),
                Either::Left(6),
                Either::Left(7),
            ],
            61,
        );
    }
}
