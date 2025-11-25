// Problem: https://leetcode.com/problems/roman-to-integer/description/

use std::collections::HashMap;
use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Roman To Integer ==");

    let input = "MCMXCIV".to_string();
    printv!(input);

    trap!({
        let result = roman_to_int(input);

        printv!(result);
    });

    println!();
}

// -- Roman to Integer --
fn roman_to_int(s: String) -> i32 {
    if s.len() < 1 || s.len() > 15 {
        return 0;
    }

    let dict = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    //I can be placed before V (5) and X (10) to make 4 and 9.
    //X can be placed before L (50) and C (100) to make 40 and 90.
    //C can be placed before D (500) and M (1000) to make 400 and 900.
    let subtract_map = HashMap::from([
        ('V', 'I'),
        ('X', 'I'),
        ('L', 'X'),
        ('C', 'X'),
        ('D', 'C'),
        ('M', 'C'),
    ]);

    let mut acc = 0;
    let cvec = s.to_uppercase().chars().collect::<Vec<char>>();

    for i in 0..cvec.len() {
        let curr_int = dict.get(&cvec[i]).unwrap();

        if i <= 0 {
            acc += curr_int;
            continue;
        }

        let prev = dict.get(&cvec[i - 1]).unwrap();
        if curr_int > prev && &cvec[i - 1] == subtract_map.get(&cvec[i]).unwrap() {
            acc += curr_int - prev - prev
        } else {
            acc += curr_int;
        }

        // println!("{} {} {}", i, curr_int, acc);
    }

    return acc;
}