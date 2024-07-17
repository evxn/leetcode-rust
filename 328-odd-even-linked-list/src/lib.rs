// https://leetcode.com/problems/odd-even-linked-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // case list.len() <= 1
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // case list.len() > 1
        let (mut odd_head, mut even_head) = (None, None);
        let (mut next_odd, mut next_even) = (&mut odd_head, &mut even_head);

        let mut is_odd = true;

        let mut next_node = head;
        while let Some(mut node) = next_node {
            next_node = node.next.take();

            if is_odd {
                next_odd = &mut next_odd.insert(node).next;
            } else {
                next_even = &mut next_even.insert(node).next;
            }

            is_odd = !is_odd;
        }

        // note: unwrap is safe bacause of the guard on head.next
        let _ = next_odd.insert(even_head.unwrap());

        odd_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ListNode {
        pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
            v.iter().rev().fold(None, |acc, &val| {
                Some(Box::new(ListNode { val, next: acc }))
            })
        }
    }

    #[test]
    fn example1() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![1, 2, 3, 4, 5])),
            ListNode::from_vec(vec![1, 3, 5, 2, 4])
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![2, 1, 3, 5, 6, 4, 7])),
            ListNode::from_vec(vec![2, 3, 6, 7, 1, 5, 4])
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![1])),
            ListNode::from_vec(vec![1])
        );
    }
    #[test]
    fn example4() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![])),
            ListNode::from_vec(vec![])
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            Solution::odd_even_list(ListNode::from_vec(vec![1, 2])),
            ListNode::from_vec(vec![1, 2])
        );
    }
}
