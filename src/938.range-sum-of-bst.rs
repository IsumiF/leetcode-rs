/*
 * @lc app=leetcode id=938 lang=rust
 *
 * [938] Range Sum of BST
 */

struct Solution {}

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

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut solver = Solver::new(low, high);
        solver.solve(root);
        solver.sum
    }
}

struct Solver {
    low: i32,
    high: i32,
    sum: i32,
}

impl Solver {
    fn new(low: i32, high: i32) -> Self {
        Self { low, high, sum: 0 }
    }

    fn solve(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            if node_ref.val >= self.low && node_ref.val <= self.high {
                self.sum += node_ref.val;
            }
            self.solve(node_ref.left.clone());
            self.solve(node_ref.right.clone());
        }
    }
}
// @lc code=end