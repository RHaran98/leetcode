use core::borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
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

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        match root.clone() {
            Some(val) => {
                stack.push(val);
            }
            None => {
                return root;
            }
        }

        loop {
            if let Some(node) = stack.pop() {
                let mut node = node.borrow_mut();

                if let Some(left) = node.left.clone() {
                    stack.push(left);
                };

                if let Some(right) = node.right.clone() {
                    stack.push(right);
                };
                let l = node.left.take();
                let r = node.right.take();
                node.right = l;
                node.left = r;
            } else {
                break;
            }
        }

        root
    }
}
