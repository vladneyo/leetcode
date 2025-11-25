// Problem: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Find The Index Of The First Occurrence In A String ==");
    
    let input1 = "sadbutsad".to_string();
    printv!(input1);
    
    let input2 = "sad".to_string();
    printv!(input2);
    
    trap!({
        let result = str_str(input1, input2);

        printv!(result);
    });

    println!();
}

// -- Find the Index of the First Occurrence in a String --
pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(needle.as_str()) {
        None => -1,
        Some(idx) => idx as i32,
    }
}