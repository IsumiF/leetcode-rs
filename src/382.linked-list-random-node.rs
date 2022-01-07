/*
 * @lc app=leetcode id=382 lang=rust
 *
 * [382] Linked List Random Node
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

// @lc code=start
use rand::prelude::random;

struct Solution {
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut values = Vec::<i32>::new();
        while let Some(node) = head.take() {
            values.push(node.val);
            head = node.next;
        }
        Self { values }
    }

    fn get_random(&self) -> i32 {
        let idx = random::<usize>() % self.values.len();
        self.values[idx]
    }
}

/*
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
// @lc code=end
