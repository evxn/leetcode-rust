// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree

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
    // Time: O(tree_height), Memory O(1)
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        loop {
            if let Some(node) = root {
                let val = node.borrow().val;

                if p < val && q < val {
                    // p, q are on the left subtree
                    root = node.borrow().left.clone();
                } else if val < p && val < q {
                    // p, q are on the right subtree
                    root = node.borrow().right.clone();
                } else {
                    // p, q are either on different subtrees or at the node itself
                    return Some(node);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));

        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: p.clone(),
            right: q.clone(),
        })));

        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), root);
    }

    #[test]
    fn example2() {
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));

        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: q.clone(),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: p.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        })));

        assert_eq!(Solution::lowest_common_ancestor(root, p.clone(), q), p);
    }

    #[test]
    fn example3() {
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));

        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: q.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        })));

        let root = p.clone();

        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), root);
    }
}
