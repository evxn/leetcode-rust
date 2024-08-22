// https://leetcode.com/problems/balanced-binary-tree

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
    // Time: O(n), Memory: O(n)
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_balanced = true;
        let mut node_to_height: HashMap<_, i32> = HashMap::new();
        let mut stack = Vec::new(); // stack frame: (node, is_visited)

        if let Some(root_node) = root {
            stack.push((root_node, false));
        }

        while is_balanced && !stack.is_empty() {
            if let Some((node, is_visited)) = stack.pop() {
                if !is_visited {
                    // push the node back with the visited flag set to true
                    stack.push((Rc::clone(&node), true));

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

                    // update is_balanced
                    is_balanced = (left_height - right_height).abs() <= 1;
                }
            }
        }

        is_balanced
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
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
        })));

        assert!(Solution::is_balanced(tree));
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),

                right: None,
            }))),
        })));

        assert!(!Solution::is_balanced(tree));
    }

    #[test]
    fn example3() {
        let tree = None;

        assert!(Solution::is_balanced(tree));
    }
}
