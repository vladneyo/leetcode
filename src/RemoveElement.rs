// Problem: https://leetcode.com/problems/remove-element/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Remove Element ==");

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    printv!(nums);

    trap!({
        let result = remove_element(&mut nums, 2);
        printv!(nums);
        printv!(result);
    });

    println!();
}

// -- Remove Element --
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain_mut(|x| *x != val);
    nums.len() as i32
}