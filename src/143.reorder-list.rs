/*
 * @lc app=leetcode id=143 lang=rust
 *
 * [143] Reorder List
 */

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

struct Solution {}

// @lc code=start
use std::iter::FromIterator;

struct ListIter {
    head: Option<Box<ListNode>>,
}

impl ListIter {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }
}

impl Iterator for ListIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut head: Option<Box<ListNode>> = None;
        std::mem::swap(&mut head, &mut self.head);
        head.map(|node| {
            self.head = node.next;
            node.val
        })
    }
}

struct List {
    head: Option<Box<ListNode>>,
}

impl FromIterator<i32> for List {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut head: Option<Box<ListNode>> = None;
        for item in iter {
            let new_head = Some(Box::new(ListNode {
                val: item,
                next: head,
            }));
            head = new_head;
        }
        List { head }
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut result: Option<Box<ListNode>> = None;
        std::mem::swap(head, &mut result);
        let list = ListIter::new(result).collect::<Vec<i32>>();
        let middle = if list.len() % 2 == 0 {
            list.len() / 2
        } else {
            list.len() / 2 + 1
        };
        let left = &list[..middle];
        let right = &list[middle..];
        let mut merged: Vec<i32> = Vec::new();
        merged.reserve_exact(left.len() + right.len());
        for i in 0..left.len() {
            merged.push(left[i]);
            if right.len() >= i + 1 {
                merged.push(right[right.len() - 1 - i]);
            }
        }
        result = merged.into_iter().rev().collect::<List>().head;
        std::mem::swap(&mut result, head);
    }
}
// @lc code=end
