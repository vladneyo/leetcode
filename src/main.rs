use std::collections::HashMap;
use std::time::Instant;

macro_rules! measure {
    ($e:expr) => {
        let start = Instant::now();
        $e;
        let duration = start.elapsed();
        println!("{:?}", duration);
    };
}

fn main() {
    let result;
    measure!({
        // result = two_sum(vec![2,7,11,15], 9);
        // result = is_palindrome(121421);
        // result = roman_to_int("MCMXCIV".to_string());
        // result = longest_common_prefix(Vec::from(
        //     ["flower", "floor", "flink", "flow"].map(|x| x.to_owned()),
        // ));
        // result = is_valid("({}[({})])".to_string());

        // let (list1, list2) = create_linked_list();
        // result = merge_two_lists(list1, list2);

        // result = remove_duplicates(&mut vec![0, 0, 1, 2, 2, 4, 5, 5, 5, 5, 6]);

        // let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        // result = remove_element(&mut nums, 2);
        // println!("{:?}", nums);

        //result = str_str("sadbutsad".to_string(), "sad".to_string());

        // result = search_insert(vec![1, 3, 5, 6], 5);
        // result = search_insert(vec![1, 3, 5, 6], 2);

        // result = length_of_last_word("   fly me   to   the moon  ".to_string());

        // result = plus_one(vec![4, 3, 2, 1]);
        // result = plus_one(vec![8, 9]);

        // result = add_binary("1010".to_string(), "1011".to_string());
        // result = add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1011".to_string());
        // result = add_binary("0".to_string(), "0".to_string());

        // result = my_sqrt(2147483647);
        // result = my_sqrt(7919);
        // result = my_sqrt(9);
        result = my_sqrt(4);
    });
    println!("{:?}", result);
}

// -- Sqrt(x) --
fn my_sqrt(x: i32) -> i32 {
    let mut guess = i64::from(1);
    let mut i = 0;

    while true {
        i += 1;

        println!("guess {:?}, sqr {:?}", guess, guess * guess);

        guess = (guess + x as i64 / guess + 1) / 2;

        if guess * guess <= x as i64 && (guess + 1) * (guess + 1) > x as i64 {
            println!("i {:?}", i);
            return guess as i32;
        }
    }

    println!("i {:?}", i);
    guess as i32
}
// --------------

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
        (x, _) => {}
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
// --------------

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
// --------------

// -- Length of Last Word --
fn length_of_last_word(s: String) -> i32 {
    match s.split_whitespace().last() {
        Some(word) => word.len() as i32,
        None => 0,
    }
}
// --------------

// -- Search Insert Position --
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let i = nums.iter().position(|&x| x == target);
    if i.is_some() {
        i.unwrap() as i32;
    }

    match nums.iter().position(|&x| x >= target) {
        Some(i) => i as i32,
        None => nums.len() as i32,
    }
}
// --------------

// -- Find the Index of the First Occurrence in a String --
pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(needle.as_str()) {
        None => -1,
        Some(idx) => idx as i32,
    }
}
// --------------

// -- Remove Element --
fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain_mut(|x| *x != val);
    nums.len() as i32
}
// --------------

// -- Remove Duplicates from Sorted Array --
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}
// --------------

// -- Merge Two Sorted Lists --
fn create_linked_list() -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    (list1, list2)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // create an empty head for the merged list
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    // while both lists have nodes, compare and attach the smaller node
    while list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            // store the next of less-valued list node
            let next = list1.as_mut().unwrap().next.take();
            // link the first remaining value of list1 to result list
            tail.next = list1;
            // move tail pointer to the last element in the list
            tail = tail.next.as_mut().unwrap();
            // assign the remainings of list1 back
            list1 = next;
        } else {
            // store the next of less-valued list node
            let next = list2.as_mut().unwrap().next.take();
            // link the first remaining value of list2 to result list
            tail.next = list2;
            // move tail pointer to the last element in the list
            tail = tail.next.as_mut().unwrap();
            // assign the remainings of list2 back
            list2 = next;
        }
    }

    // append any remaining nodes
    tail.next = if list1.is_some() { list1 } else { list2 };

    // return the merged list, skipping the head
    head.next
}
// --------------

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
// --------------

// -- Longest Common Prefix --
fn longest_common_prefix(strs: Vec<String>) -> String {
    // allocate
    let mut ordered_str = strs.clone();
    // modify
    ordered_str.sort_by_key(|s| s.len());
    println!("{:?}", ordered_str);

    // chunk to check is 1st word, then reduced
    let mut chunk = ordered_str.first().unwrap().to_string();
    while (!ordered_str.iter().all(|x| x.starts_with(&chunk))) {
        chunk = chunk.chars().take(chunk.len() - 1).collect::<String>();
    }
    return chunk;
}
// --------------

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

        println!("{} {} {}", i, curr_int, acc);
    }

    return acc;
}
// --------------

// -- Palindrome Number --
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let str = x.to_string();
    let lnum = str.len();
    let center = if (lnum / 2) % 2 == 0 {
        lnum / 2
    } else {
        lnum / 2 + 1
    };
    return str.chars().take(center).eq(str.chars().rev().take(center));
}
// --------------

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
// --------------
