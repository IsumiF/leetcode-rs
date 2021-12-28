/*
 * @lc app=leetcode id=876 lang=rust
 *
 * [876] Middle of the Linked List
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

// 0 1 2 3
// 0 1 2

// @lc code=start
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = ListIter::new(&head).count();
        let mut current = head;
        for _ in 0..(len / 2) {
            current = current.take().and_then(|node| node.next);
        }
        current
    }
}

struct ListIter<'a> {
    current: Option<&'a ListNode>,
}

impl<'a> ListIter<'a> {
    fn new(head: &'a Option<Box<ListNode>>) -> Self {
        Self {
            current: head.as_ref().map(|node| node.as_ref()),
        }
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let val = &node.val;
            self.current = node.next.as_ref().map(|next| next.as_ref());
            val
        })
    }
}

// @lc code=end
