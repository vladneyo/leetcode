// Problem: https://leetcode.com/problems/longest-common-prefix/description/

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Longest Common Prefix ==");

    let input = ["flower", "floor", "flink", "flow"];
    printv!(input);

    trap!({
        let result = longest_common_prefix(Vec::from(
            input.map(|x| x.to_owned()),
        ));

        printv!(result);
    });

    println!();
}

// -- Longest Common Prefix --
fn longest_common_prefix(strs: Vec<String>) -> String {
    // allocate
    let mut ordered_str = strs.clone();
    // modify
    ordered_str.sort_by_key(|s| s.len());
    // printv!(ordered_str);

    // chunk to check is 1st word, then reduced
    let mut chunk = ordered_str.first().unwrap().to_string();
    while !ordered_str.iter().all(|x| x.starts_with(&chunk)) {
        chunk = chunk.chars().take(chunk.len() - 1).collect::<String>();
    }
    chunk
}