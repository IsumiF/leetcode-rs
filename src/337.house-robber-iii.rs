/*
 * @lc app=leetcode id=337 lang=rust
 *
 * [337] House Robber III
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::rob_rec(root).0
    }

    fn rob_rec(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => {
                let borrowed_node = node.borrow();
                let (left_max, left_max_without_root) = Self::rob_rec(borrowed_node.left.clone());
                let (right_max, right_max_without_root) = Self::rob_rec(borrowed_node.right.clone());
                let current_max_with_root = borrowed_node.val + left_max_without_root + right_max_without_root;
                let current_max_without_root = left_max + right_max;
                let current_max = std::cmp::max(current_max_without_root, current_max_with_root);
                (current_max, current_max_without_root)
            },
        }
    }
}
// @lc code=end
