// https://leetcode.com/problems/kth-smallest-element-in-a-bst

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

/// For the followup question of the BST modified often (i.e.,
/// we can do insert and delete operations) and when you need to
/// find the kth smallest frequently, how would you optimize?
///
/// Store the first k of inorder traversed node values in a `first_k: Vecdeque<i32>`.
/// They will be sorted cause it's a BST. After the tree insert we compare the `new_val`
/// with our k-th stored element. We check if `first_k.back() < new_val`.
/// If it's not, `first_k` does not need to be updated.
/// Otherwise we need to evict the current k-th element and insert the `new_val` in
/// our sorted way:
/// ```
/// // pseudocode
/// first_k.pop_back()
/// let index = first_k.binary_search(new_val)
/// first_k.insert(index, new_val)
/// ```
/// with this Vecdeque populated we will have O(1) lookups on that cache.
///
/// On deletion we can just recompute the first k. For handling tree deletions
/// efficiently after evicting the `deleted_node_val` from the `first_k` we need
/// to be able to quickly push the next largest element in the tree to the back of
/// the `first_k`. For that we may need parent pointers hashmap O(n) memory
/// to figure out the next largest node to the deleted node
/// which may take O(n) time.
impl Solution {
    // Time: O(n), Memory O(n)
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Solution::inorder_traversal(root)
            .into_iter()
            .nth((k - 1) as _)
            .unwrap()
    }

    // fn impl copied from ../94-binary-tree-inorder-traversal/src/lib.rs
    // Time: O(n), Memory O(n)
    #[inline]
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = vec![(root, false)];

        while let Some((current_ptr, is_visited)) = stack.pop() {
            if let Some(current) = current_ptr {
                if !is_visited {
                    let left = current.borrow_mut().left.take();
                    let right = current.borrow_mut().right.take();

                    stack.extend_from_slice(&[
                        (right, false),
                        (Some(current), true),
                        (left, false),
                    ]);
                } else {
                    result.push(current.borrow().val);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::kth_smallest(tree, 3), 3);
    }

    #[test]
    fn example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        })));
        assert_eq!(Solution::kth_smallest(tree, 3), 3);
    }

    #[test]
    fn example3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(42))));
        assert_eq!(Solution::kth_smallest(tree, 1), 42);
    }
}
