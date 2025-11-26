// Problem: https://leetcode.com/problems/climbing-stairs/description/?envType=problem-list-v2&envId=dynamic-programming

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Climbing Stairs ==");

    let mut input = 3;
    printv!(input);

    trap!({
        let result = climb_stairs(input);

        printv!(result);
    });
    /*
    1 + 1 + 1
    1 + 2
    2 + 1
     */

    input = 6;
    printv!(input);

    trap!({
        let result = climb_stairs(input);

        printv!(result);
    });
    /*
    1. 1 + 1 + 1 + 1 + 1 + 1

    2. 1 + 1 + 1 + 1 + 2
    3. 1 + 1 + 1 + 2 + 1
    4. 1 + 1 + 2 + 1 + 1
    5. 1 + 2 + 1 + 1 + 1
    6. 2 + 1 + 1 + 1 + 1

    7. 1 + 1 + 2 + 2
    8. 1 + 2 + 2 + 1
    9. 2 + 2 + 1 + 1
    10. 1 + 2 + 1 + 2
    11. 2 + 1 + 2 + 1
    12. 2 + 1 + 1 + 2

    13. 2 + 2 + 2
     */

    println!();
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut one : i32 = 1;
    let mut two : i32 = 1;

    for _i in 0..n-1 {
        one = one + two;
        two = one - two;
    }

    one
}