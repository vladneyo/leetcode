// Problem: https://leetcode.com/problems/valid-parentheses/description/

use std::collections::HashMap;
use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Valid Parentheses ==");

    let input = "({}[({})])".to_string();
    printv!(input);

    trap!({
        let result = is_valid(input);

        printv!(result);
    });

    println!();
}

// -- Valid Parentheses --
fn is_valid(s: String) -> bool {
    if s.len() < 2 {
        return false;
    }

    let pairs = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => {
                if stack.pop() != Some(pairs[&c]) {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}