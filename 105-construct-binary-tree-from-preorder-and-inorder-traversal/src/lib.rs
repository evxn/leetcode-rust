// https://leetcode.com/problems/105-construct-binary-tree-from-preorder-and-inorder-traversal

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // let tree = Some(Rc::new(RefCell::new(TreeNode {
        //     val: 1,
        //     left: None,
        //     right: Some(Rc::new(RefCell::new(TreeNode {
        //         val: 2,
        //         left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        //         right: None,
        //     }))),
        // })));

        // assert_eq!(Solution::inorder_traversal(tree), vec![1, 3, 2]);
    }

    #[test]
    fn example2() {}
}
