mod kyu8;
mod kyu7;
mod kyu6;
use kyu8::multiply;
use kyu7::median;

use kyu6::if_you_can_read_this;

fn main() {
    println!("Hello, world!");
    multiply::multiply(3, 4);
    
    println!("{}",median::mode_value(&[1, 2, 1, 1, 5, 2, 2, 8, 9, 1]));

    if_you_can_read_this::show_letters('A')
}