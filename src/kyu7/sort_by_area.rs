use std::f64::consts::PI;

use either::Either;

/*
In this kata you will be given a sequence of the dimensions of rectangles ( sequence with width and length ) and circles ( radius - just a number ).
Your task is to return a new sequence of dimensions, sorted ascending by area.

For example,

seq = &[ Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212)) ] // ( rectangle, circle, circle, rectangle )

sort_by_area(seq) => &[ Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444) ];
 */

fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut result = seq.to_owned();
    result.sort_by(|a, b| calculate_area(a).partial_cmp(&calculate_area(b)).unwrap());
    return result;
}

fn calculate_area(object: &Either<(f64, f64), f64>) -> f64 {
    match object {
        Either::Left(rectangle) => rectangle.0 * rectangle.1,
        Either::Right(radius) => radius.powi(2) * PI,
    }
}

fn dotest(seq: &[Either<(f64, f64), f64>], expected: &[Either<(f64, f64), f64>]) {
    let actual = sort_by_area(seq);
    assert!(
        actual == expected,
        "With seq = {seq:?}\nExpected {expected:?} but got {actual:?}"
    )
}

#[test]
fn fixed_tests() {
    dotest(
        &[
            Either::Left((4.23, 6.43)),
            Either::Right(1.23),
            Either::Right(3.444),
            Either::Left((1.342, 3.212)),
        ],
        &[
            Either::Left((1.342, 3.212)),
            Either::Right(1.23),
            Either::Left((4.23, 6.43)),
            Either::Right(3.444),
        ],
    );
    dotest(
        &[Either::Left((2.0, 5.0)), Either::Right(6.0)],
        &[Either::Left((2.0, 5.0)), Either::Right(6.0)],
    );
    dotest(&[], &[]);
}
