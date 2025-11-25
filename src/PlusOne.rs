// Problem: https://leetcode.com/problems/plus-one/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Plus One ==");

    let input1 = vec![4, 3, 2, 1];
    printv!(input1);

    trap!({
        let result = plus_one(input1);

        printv!(result);
    });

    let input2 = vec![8, 9];
    printv!(input2);

    trap!({
        let result = plus_one(input2);

        printv!(result);
    });

    println!();
}

// -- Plus One --
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = digits.clone();

    for i in (0..res.len()).rev() {
        if res[i] == 9 {
            let mut new_digits = match plus_one(res[..digits.len() - 1].to_vec()) {
                x if x.len() == 0 => vec![1],
                x => x,
            };
            new_digits.extend([0]);
            return new_digits;
        } else {
            res[i] = res[i] + 1;
            break;
        }
    }
    res.to_owned()
}