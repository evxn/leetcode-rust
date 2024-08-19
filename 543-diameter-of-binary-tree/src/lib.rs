// https://leetcode.com/problems/diameter-of-binary-tree

use std::cell::RefCell;
use std::collections::HashMap;
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
    // Time: O(n)
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        // Memory: O(n)
        let mut stack = Vec::new();
        // Memory: O(n)
        let mut node_to_height: HashMap<_, i32> = HashMap::new();

        if let Some(root_node) = root {
            stack.push((root_node, false));
        }

        while let Some((node, visited)) = stack.pop() {
            if !visited {
                // push the node back with the visited flag set to true
                stack.push((node.clone(), true));

                // push the right and left children onto the stack
                if let Some(right_node) = node.borrow().right.clone() {
                    stack.push((right_node, false));
                }
                if let Some(left_node) = node.borrow().left.clone() {
                    stack.push((left_node, false));
                }
            } else {
                // childred were processed by now so we can check for pre-computed heights
                let left_height = node.borrow().left.as_ref().map_or(0, |left_node| {
                    node_to_height
                        .get(&Rc::as_ptr(left_node))
                        .copied()
                        .unwrap_or(0)
                });
                let right_height = node.borrow().right.as_ref().map_or(0, |right_node| {
                    node_to_height
                        .get(&Rc::as_ptr(right_node))
                        .copied()
                        .unwrap_or(0)
                });

                let node_height = 1 + left_height.max(right_height);
                node_to_height.insert(Rc::as_ptr(&node), node_height);

                // update the max diameter
                max_diameter = max_diameter.max(left_height + right_height);
            }
        }

        max_diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
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

        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));

        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }

    #[test]
    fn example3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));

        assert_eq!(Solution::diameter_of_binary_tree(tree), 0);
    }

    #[test]
    fn example4() {
        let tree = None;

        assert_eq!(Solution::diameter_of_binary_tree(tree), 0);
    }
}
