/*
 * @lc app=leetcode id=701 lang=rust
 *
 * [701] Insert into a Binary Search Tree
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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) => {
                {
                    let mut node = node.borrow_mut();
                    if val < node.val {
                        node.left = Solution::insert_into_bst(node.left.clone(), val);
                    } else {
                        node.right = Solution::insert_into_bst(node.right.clone(), val);
                    }
                }
                Some(node)
            }
        }
    }
}
// @lc code=end
