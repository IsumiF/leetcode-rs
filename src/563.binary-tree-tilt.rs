/*
 * @lc app=leetcode id=563 lang=rust
 *
 * [563] Binary Tree Tilt
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
struct MyTreeNode<T> {
    pub val: T,
    pub left: Option<Box<MyTreeNode<T>>>,
    pub right: Option<Box<MyTreeNode<T>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let tilt_tree = Self::build_tilt_tree(root);
        Self::sum_tilt_tree(tilt_tree)
    }

    fn build_tilt_tree(src: Option<Rc<RefCell<TreeNode>>>) -> Option<Box<MyTreeNode<(i32, i32)>>> {
        match src {
            None => None,
            Some(node_ref) => {
                let node = node_ref.borrow();
                let left_tree = Self::build_tilt_tree(node.left.clone());
                let right_tree = Self::build_tilt_tree(node.right.clone());
                let (left_sum, _) = left_tree.as_ref().map(|node| node.val).unwrap_or((0, 0));
                let (right_sum, _) = right_tree.as_ref().map(|node| node.val).unwrap_or((0, 0));
                let new_result = (left_sum + right_sum + node.val, i32::abs(left_sum - right_sum));
                Some(Box::new(MyTreeNode {
                    val: new_result,
                    left: left_tree,
                    right: right_tree,
                }))
            }
        }
    }

    fn sum_tilt_tree(node: Option<Box<MyTreeNode<(i32, i32)>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let left = Self::sum_tilt_tree(node.left);
                let right = Self::sum_tilt_tree(node.right);
                node.val.1 + left + right
            },
        }
    }
}
// @lc code=end
