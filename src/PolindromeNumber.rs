// Problem: https://leetcode.com/problems/palindrome-number/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Polindrome Number ==");

    let input = 121421;
    printv!(input);

    trap!({
        let result = is_palindrome(input);

        printv!(result);
    });
    println!();
}

// -- Palindrome Number --
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let str = x.to_string();
    let lnum = str.len();
    let center = if (lnum / 2) % 2 == 0 {
        lnum / 2
    } else {
        lnum / 2 + 1
    };
    return str.chars().take(center).eq(str.chars().rev().take(center));
}