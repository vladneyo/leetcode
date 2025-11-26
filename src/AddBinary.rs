// Problem: https://leetcode.com/problems/add-binary/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== AddBinary ==");

    let mut input1 = "1010".to_string();
    printv!(input1);
    let mut input2 = "1011".to_string();
    printv!(input2);

    trap!({
        let result = add_binary(input1, input2);
        
        printv!(result);
    });

    input1 = "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string();
    printv!(input1);
    input2 = "1011".to_string();
    printv!(input2);

    trap!({
        let result = add_binary(input1, input2);
        
        printv!(result);
    });

    input1 = "0".to_string();
    printv!(input1);
    input2 = "0".to_string();
    printv!(input2);

    trap!({
        let result = add_binary(input1, input2);
        
        printv!(result);
    });

    println!();
}

// -- Add Binary --
fn add_binary(a: String, b: String) -> String {
    let mut ma = a.clone();
    let mut mb = b.clone();
    match (ma.len(), mb.len()) {
        (x, y) if x < y => {
            // zero-padding
            String::insert_str(&mut ma, 0, "0".repeat(y - x).as_str());
        }
        (x, y) if x > y => {
            // zero-padding
            String::insert_str(&mut mb, 0, "0".repeat(x - y).as_str());
        }
        (_x, _) => {}
    };

    let mut result = String::new();
    let mut carry = 0;
    for i in (0..ma.len()).rev() {
        let x = ma.chars().nth(i).unwrap().to_digit(2).unwrap();
        let y = mb.chars().nth(i).unwrap().to_digit(2).unwrap();
        match (x + y, carry) {
            // overflow bit, already have 1 to carry
            (2, 1) => {
                result.push('1');
                carry = 1;
            }
            // overflow bit, nothing left to carry
            (2, 0) => {
                result.push('0');
                carry = 1;
            }
            // fill bit, already have 1 to carry
            (1, 1) => {
                result.push('0');
                carry = 1;
            }
            // fill bit, nothing left to carry
            (1, 0) => {
                result.push('1');
                carry = 0;
            }
            // 0+0, already have 1 to carry
            (0, 1) => {
                // assign from carry, nothing left to carry
                result.push('1');
                carry = 0;
            }
            // 0+0. nothing left to carry
            (0, 0) => {
                result.push('0');
                carry = 0;
            }
            _ => unreachable!(),
        }
    }

    if carry == 1 {
        result.push_str(carry.to_string().as_str());
    }

    result.chars().rev().collect::<String>()
}