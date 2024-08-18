// https://leetcode.com/problems/maximum-depth-of-binary-tree

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |node| {
            let mut max_depth = 0;

            let mut nodes_to_visit = vec![(1, Some(node))]; // (depth_if_node_exist, node_ptr)
            while let Some((depth, node_ptr)) = nodes_to_visit.pop() {
                if let Some(node) = node_ptr {
                    // here node exists and we can update max_depth
                    max_depth = max_depth.max(depth);

                    let mut node = node.borrow_mut();
                    // mark left and right children for visit
                    nodes_to_visit.push((depth + 1, node.right.take()));
                    nodes_to_visit.push((depth + 1, node.left.take()));
                }
            }

            max_depth
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(Solution::max_depth(tree), 3);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));

        assert_eq!(Solution::max_depth(tree), 2);
    }

    #[test]
    fn example3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));

        assert_eq!(Solution::max_depth(tree), 1);
    }

    #[test]
    fn example4() {
        let tree = None;

        assert_eq!(Solution::max_depth(tree), 0);
    }
}
