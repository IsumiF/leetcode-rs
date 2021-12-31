/*
 * @lc app=leetcode id=1026 lang=rust
 *
 * [1026] Maximum Difference Between Node and Ancestor
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut solver = Solver::new();
        solver.solve(root.unwrap());
        solver.max_abs
    }
}

struct Solver {
    max_abs: i32,
}

impl Solver {
    fn new() -> Self {
        Self { max_abs: 0 }
    }

    fn solve(&mut self, root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
        let node = root.borrow();

        let left_result = node.left.clone().map(|left| self.solve(left));
        let right_result = node.right.clone().map(|right| self.solve(right));
        let sub_results = [left_result, right_result];
        let sub_results = sub_results.iter().flat_map(std::convert::identity);
        let max = sub_results
            .clone()
            .map(|(_, max)| *max)
            .chain(std::iter::once(node.val))
            .max()
            .unwrap();
        let min = sub_results
            .clone()
            .map(|(min, _)| *min)
            .chain(std::iter::once(node.val))
            .min()
            .unwrap();
        let max_abs = sub_results
            .clone()
            .map(|&(min, max)| std::cmp::max(i32::abs(node.val - min), i32::abs(node.val - max)))
            .max();
        max_abs.map(|max_abs| {
            if max_abs > self.max_abs {
                self.max_abs = max_abs;
            }
        });
        (min, max)
    }
}

// @lc code=end
