/*
 * @lc app=leetcode id=1022 lang=rust
 *
 * [1022] Sum of Root To Leaf Binary Numbers
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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        solve(root, 0, 0)
    }
}

fn solve(root: Option<Rc<RefCell<TreeNode>>>, partial_value: i32, sum: i32) -> i32 {
    match root {
        None => sum,
        Some(node) => {
            let node = node.borrow();
            let new_partial_value = partial_value * 2 + node.val;
            if node.left.is_none() && node.right.is_none() { // is leaf
                return new_partial_value + sum;
            }
            let left_updated_sum = solve(node.left.clone(), new_partial_value, sum);
            let right_updated_sum = solve(node.right.clone(), new_partial_value, left_updated_sum);
            right_updated_sum
        }
    }
}
// @lc code=end
