// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree

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
    // Time: O(n), Memory O(n)
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (root, p, q) = (root.unwrap(), p.unwrap(), q.unwrap());

        let mut known_parents = HashMap::new();
        let mut stack = vec![root];

        let (mut found_p, mut found_q) = (false, false);
        while !found_p || !found_q {
            if let Some(node) = stack.pop() {
                // push children to the stack to process
                let right = node.borrow().right.clone();
                let left = node.borrow().left.clone();

                [right, left].into_iter().flatten().for_each(|child| {
                    known_parents.insert(Rc::as_ptr(&child), Some(node.clone()));
                    stack.push(child);
                });
                // break the cycle
                if node == p {
                    found_p = true;
                }
                if node == q {
                    found_q = true;
                }
            }
        }

        // we have parents hashmap populated and we've found p and q
        // so we can reconstruct both paths to the root
        let p_path = Solution::path_from_node_to_root(&known_parents, p);
        let q_path = Solution::path_from_node_to_root(&known_parents, q);

        p_path
            .into_iter()
            .rev()
            .fuse()
            .zip(q_path.into_iter().rev().fuse())
            .take_while(|(p_ancestor, q_ancestor)| p_ancestor == q_ancestor)
            .map(|(node_ptr, _)| node_ptr)
            .last()
            .unwrap()
    }

    // Time: O(height), Memory: O(height)
    #[inline]
    fn path_from_node_to_root(
        parents: &HashMap<*const RefCell<TreeNode>, Option<Rc<RefCell<TreeNode>>>>,
        node: Rc<RefCell<TreeNode>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut path = vec![];
        // start with the node itself and go to the root
        let mut node = Some(node);
        while node.is_some() {
            path.push(node.clone());
            node = parents
                .get(&Rc::as_ptr(&node.unwrap()))
                .cloned()
                .and_then(|maybe_parent| maybe_parent);
        }

        path
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
