// https://leetcode.com/problems/subtree-of-another-tree

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
    // Time: O(n²), Memory: O(n)
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![root];

        while let Some(node_ptr) = stack.pop() {
            if Solution::is_same_tree(node_ptr.clone(), sub_root.clone()) {
                return true;
            }

            if let Some(node) = node_ptr {
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
            }
        }

        false
    }

    // fn impl copied from ../100-same-tree/src/lib.rs
    // Time: O(n), Memory: O(n)
    #[inline]
    fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_same = true;
        let mut stack = vec![(p, q)];

        while is_same && !stack.is_empty() {
            if let Some((p, q)) = stack.pop() {
                is_same = match (p, q) {
                    (None, None) => true,
                    (None, Some(_)) => false,
                    (Some(_), None) => false,
                    (Some(p), Some(q)) => {
                        let p = p.borrow();
                        let q = q.borrow();

                        stack.push((p.right.clone(), q.right.clone()));
                        stack.push((p.left.clone(), q.left.clone()));

                        p.val == q.val
                    }
                }
            }
        }

        is_same
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));

        assert!(Solution::is_subtree(tree1, tree2));
    }

    #[test]
    fn example2() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));

        assert!(!Solution::is_subtree(tree1, tree2));
    }

    #[test]
    fn example3() {
        let tree1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let tree2 = None;

        assert!(Solution::is_subtree(tree1, tree2));
    }

    #[test]
    fn example4() {
        let tree1 = None;
        let tree2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert!(!Solution::is_subtree(tree1, tree2));
    }

    #[test]
    fn example5() {
        let tree1 = None;
        let tree2 = None;

        assert!(Solution::is_subtree(tree1, tree2));
    }
}
