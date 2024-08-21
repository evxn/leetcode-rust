// https://leetcode.com/problems/binary-tree-level-order-traversal

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
    // Time: O(n), Memory: O(n)
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root);

        while !queue.is_empty() {
            let mut level = Vec::new();

            for _ in 0..queue.len() {
                if let Some(Some(current)) = queue.pop_front() {
                    queue.push_back(current.borrow_mut().left.take());
                    queue.push_back(current.borrow_mut().right.take());

                    level.push(current.borrow().val);
                }
            }

            if !level.is_empty() {
                levels.push(level);
            }
        }

        levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(
            Solution::level_order(tree),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(Solution::level_order(tree), vec![vec![1]]);
    }

    #[test]
    fn example3() {
        let tree = None;

        assert_eq!(Solution::level_order(tree), Vec::<Vec<_>>::new());
    }
}
