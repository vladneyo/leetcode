// Problem: https://leetcode.com/problems/search-insert-position/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Search Insert Position ==");

    let input = vec![1, 3, 5, 6];
    printv!(input);

    trap!({
        let result = search_insert(&input, 5);
        println!("{:?}",result);
    });

    trap!({
        let result = search_insert(&input, 2);
        println!("{:?}",result);
    });

    println!();
}

// -- Search Insert Position --
fn search_insert(nums: &Vec<i32>, target: i32) -> i32 {
    let i = nums.iter().position(|&x| x == target);
    if i.is_some() {
        i.unwrap() as i32;
    }

    match nums.iter().position(|&x| x >= target) {
        Some(i) => i as i32,
        None => nums.len() as i32,
    }
}