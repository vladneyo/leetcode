// Problem: https://arc.net/l/quote/gjkfevqe

use timetrap::*;
use crate::printv;

pub fn main(){
    println!("== Merge Two Sorted Lists ==");

    let (list1, list2) = create_linked_list();
    printv!(list1);
    printv!(list2);

    trap!({
        let result = merge_two_lists(list1, list2);

        printv!(result);
    });

    println!();
}

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