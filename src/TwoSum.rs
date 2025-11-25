// Problem: https://leetcode.com/problems/two-sum/description/

use timetrap::*;
use crate::{printv};

pub fn main(){
    println!("== Two Sum ==");

    let input1 = vec![2,7,11,15];
    let input2 = 9;
    printv!(input1);
    printv!(input2);

    trap!({
        let result = two_sum(input1, input2);

        printv!(result);
    });
    println!();
}

// -- Two Sum --
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return Vec::from([]);
    }

    let _first = nums.first().unwrap();

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return Vec::from([i as i32, j as i32]);
            }
        }
    }

    return Vec::from([]);
}