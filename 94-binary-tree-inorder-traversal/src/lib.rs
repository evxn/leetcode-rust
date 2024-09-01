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
        struct InorderTreeIter {
            stack: Vec<(Option<Rc<RefCell<TreeNode>>>, bool)>,
        }

        impl InorderTreeIter {
            fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
                let stack = vec![(root, false)];
                Self { stack }
            }
        }

        impl Iterator for InorderTreeIter {
            type Item = Rc<RefCell<TreeNode>>;

            fn next(&mut self) -> Option<Self::Item> {
                while let Some((current_ptr, is_visited)) = self.stack.pop() {
                    if let Some(current) = current_ptr {
                        if !is_visited {
                            let left = current.borrow().left.clone();
                            let right = current.borrow().right.clone();

                            self.stack.extend_from_slice(&[
                                (right, false),
                                (Some(current), true),
                                (left, false),
                            ]);
                        } else {
                            return Some(current);
                        }
                    }
                }

                None
            }
        }

        InorderTreeIter::new(root)
            .map(|node| node.borrow().val)
            .collect()
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
