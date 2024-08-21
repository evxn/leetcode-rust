// https://leetcode.com/problems/count-good-nodes-in-binary-tree

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut good_counter = 0;

        let root_val = root.clone().unwrap().borrow().val;
        let mut stack = vec![(root, root_val)];

        while let Some((node_ptr, max_seen)) = stack.pop() {
            if let Some(node) = node_ptr {
                if node.borrow().val >= max_seen {
                    good_counter += 1;
                }

                let max_seen = max_seen.max(node.borrow().val);

                stack.push((node.borrow_mut().right.take(), max_seen));
                stack.push((node.borrow_mut().left.take(), max_seen));
            }
        }

        good_counter
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
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        assert_eq!(Solution::good_nodes(tree), 4);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        })));
        assert_eq!(Solution::good_nodes(tree), 3);
    }

    #[test]
    fn example3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(Solution::good_nodes(tree), 1);
    }
}
