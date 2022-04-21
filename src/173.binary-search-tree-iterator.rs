/*
 * @lc app=leetcode id=173 lang=rust
 *
 * [173] Binary Search Tree Iterator
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

// @lc code=start
use std::{cell::RefCell, rc::Rc};

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        if let Some(r) = root {
            stack.push(r);
        }

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        loop {
            let node = self.stack.last().unwrap().clone();
            let left_opt = node.borrow().left.clone();
            if let Some(left) = left_opt {
                // println!("pushed {} to stack as left", left.borrow().val);
                self.stack.push(left);
            } else {
                break;
            }
        };
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;
        if let Some(parent) = self.stack.last_mut() {
            parent.borrow_mut().left = None;
        }
        // println!("current value: {}", val);
        if let Some(right) = node.borrow().right.clone() {
            // println!("pushed {} to stack as right", right.borrow().val);
            self.stack.push(right);
        }
        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

// @lc code=end
