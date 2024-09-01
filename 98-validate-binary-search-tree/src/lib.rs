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
    // Time: O(n), Memory: O(n)
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // inorder traversal iter is copied from ../94-binary-tree-inorder-traversal/src/lib.rs
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

        let mut values_inorder_iter = InorderTreeIter::new(root).map(|node| node.borrow().val);

        let first = values_inorder_iter.next();

        let mut values_inorder_pairwise_iter =
            values_inorder_iter.scan(first, |last_seen, curr| {
                let prev = last_seen.replace(curr)?;
                Some((prev, curr))
            });

        values_inorder_pairwise_iter.all(|(prev, curr)| prev < curr)
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
