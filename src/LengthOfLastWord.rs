// Problem: https://leetcode.com/problems/length-of-last-word/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Length Of Last Word ==");

    let input = "   fly me   to   the moon  ".to_string();
    printv!(input);

    trap!({
        let result = length_of_last_word(input);

        printv!(result);
    });

    println!();
}

// -- Length of Last Word --
fn length_of_last_word(s: String) -> i32 {
    match s.split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0,
    }
}