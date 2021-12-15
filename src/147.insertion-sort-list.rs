/*
 * @lc app=leetcode id=147 lang=rust
 *
 * [147] Insertion Sort List
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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted_list: Option<Box<ListNode>> = None;
        loop {
            match head.map(|mut node| {
                let mut next: Option<Box<ListNode>> = None;
                std::mem::swap(&mut node.next, &mut next);
                (node, next)
            }) {
                None => break,
                Some((node, next)) => {
                    head = next;
                    sorted_list = insert(sorted_list, node);
                },
            }
        }
        sorted_list
    }
}

fn insert(head: Option<Box<ListNode>>, mut new_node: Box<ListNode>) -> Option<Box<ListNode>> {
    match head {
        None => Some(new_node),
        Some(mut node) => {
            if new_node.val > node.val {
                node.next = insert(node.next, new_node);
                Some(node)
            } else {
                new_node.next = Some(node);
                Some(new_node)
            }
        }
    }
}
// @lc code=end
