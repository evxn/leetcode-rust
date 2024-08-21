// https://leetcode.com/problems/binary-tree-right-side-view

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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    // Time: O(n), Memory: O(n)
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut right_side = Vec::new();
        let mut queue = VecDeque::from([root]);

        while !queue.is_empty() {
            let mut level_last_elem = None;

            for _ in 0..queue.len() {
                if let Some(Some(current)) = queue.pop_front() {
                    queue.push_back(current.borrow_mut().left.take());
                    queue.push_back(current.borrow_mut().right.take());

                    level_last_elem = Some(current.borrow().val);
                }
            }

            if let Some(level_last_elem) = level_last_elem {
                right_side.push(level_last_elem);
            }
        }

        right_side
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
                right: None,
            }))),
        })));
        assert_eq!(Solution::right_side_view(tree), vec![3, 20, 15]);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
        })));

        assert_eq!(Solution::right_side_view(tree), vec![3, 15]);
    }

    #[test]
    fn example3() {
        let tree = None;

        assert_eq!(Solution::right_side_view(tree), Vec::<_>::new());
    }
}
