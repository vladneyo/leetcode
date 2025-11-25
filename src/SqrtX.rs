// Problem: https://leetcode.com/problems/sqrtx/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Sqrt(X) ==");

    let mut input = 0;

    input = 2147483647;
    printv!(input);

    trap!({
        let result = my_sqrt(2147483647);

        printv!(result);
    });

    input = 7919;
    printv!(input);

    trap!({
        let result = my_sqrt(7919);

        printv!(result);
    });

    input = 9;
    printv!(input);

    trap!({
        let result = my_sqrt(9);

        printv!(result);
    });

    input = 4;
    printv!(input);

    trap!({
        let result = my_sqrt(4);

        printv!(result);
    });

    println!();
}

// -- Sqrt(x) --
fn my_sqrt(x: i32) -> i32 {
    let mut guess = i64::from(1);

    while true {
        guess = (guess + x as i64 / guess) / 2;

        if guess * guess <= x as i64 && (guess + 1) * (guess + 1) > x as i64 {
            return guess as i32;
        }
    }

    guess as i32
}