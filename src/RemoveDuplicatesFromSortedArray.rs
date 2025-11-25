// Problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Remove Duplicates From Sorted Array ==");

    let mut input = vec![0, 0, 1, 2, 2, 4, 5, 5, 5, 5, 6];
    printv!(input);

    trap!({
        let result = remove_duplicates(&mut input);

        printv!(result);
    });

    println!();
}

// -- Remove Duplicates from Sorted Array --
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}