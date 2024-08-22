// https://leetcode.com/problems/binary-tree-inorder-traversal/

use std::cell::RefCell;
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
    // Time: O(n), Memory: O(n)
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = vec![(root, false)];

        while let Some((current_ptr, is_visited)) = stack.pop() {
            if let Some(current) = current_ptr {
                if !is_visited {
                    let left = current.borrow_mut().left.take();
                    let right = current.borrow_mut().right.take();

                    stack.extend_from_slice(&[
                        (right, false),
                        (Some(current), true),
                        (left, false),
                    ]);
                } else {
                    result.push(current.borrow().val);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));

        assert_eq!(Solution::inorder_traversal(tree), vec![1, 3, 2]);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::inorder_traversal(tree), vec![1]);
    }

    #[test]
    fn example3() {
        let tree = None;

        assert_eq!(Solution::inorder_traversal(tree), vec![]);
    }
}
