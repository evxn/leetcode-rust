// https://leetcode.com/problems/validate-binary-search-tree

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
    // Time: O(n), Memory O(n)
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::inorder_traversal(root)
            .windows(2)
            .map(|window| (window[0], window[1]))
            .all(|(prev, next)| prev < next)
    }

    // fn impl copied from ../94-binary-tree-inorder-traversal/src/lib.rs
    // Time: O(n), Memory O(n)
    #[inline]
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(root) = root {
            let mut stack = vec![root];

            while let Some(current) = stack.pop() {
                let left = current.borrow_mut().left.take();
                let right = current.borrow_mut().right.take();

                if left.is_none() {
                    result.push(current.borrow().val);
                }

                let traverse_later = &mut match (left, right) {
                    (Some(left), Some(right)) => vec![right, current, left],
                    (Some(left), None) => vec![current, left],
                    (None, Some(right)) => vec![right],
                    (None, None) => vec![],
                };

                stack.append(traverse_later);
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
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert!(Solution::is_valid_bst(tree));
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        assert!(!Solution::is_valid_bst(tree));
    }

    #[test]
    fn example3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert!(Solution::is_valid_bst(tree));
    }

    #[test]
    fn example4() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        assert!(!Solution::is_valid_bst(tree));
    }

    #[test]
    fn example5() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert!(!Solution::is_valid_bst(tree));
    }
}
