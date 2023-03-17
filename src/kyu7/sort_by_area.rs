use either::Either;

/* 
In this kata you will be given a sequence of the dimensions of rectangles ( sequence with width and length ) and circles ( radius - just a number ).
Your task is to return a new sequence of dimensions, sorted ascending by area.

For example,

seq = &[ Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212)) ] // ( rectangle, circle, circle, rectangle )

sort_by_area(seq) => &[ Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444) ];
 */

// fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>>{
//     seq.iter()
// }