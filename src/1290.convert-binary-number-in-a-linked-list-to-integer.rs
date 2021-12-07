/*
 * @lc app=leetcode id=1290 lang=rust
 *
 * [1290] Convert Binary Number in a Linked List to Integer
 */

struct Solution {}

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

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut acc = 0;
        loop {
            match head {
                Some(node) => {
                    acc *= 2;
                    acc += node.val;
                    head = node.next;
                },
                None => {
                    return acc;
                }
            }
        }
    }
}
// @lc code=end
