// https://leetcode.com/problems/add-two-numbers/

use std::iter::{FromIterator, Fuse};

// ------- Prelude --------

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// -------- Solution ---------

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        enum CarriedDigit {
            AddedDigits(i32),
            FinalCarry,
        }

        let list = zip_longest(l1.unwrap().into_iter(), l2.unwrap().into_iter())
            .map(|either_or_both| match either_or_both {
                EitherOrBoth::Both(a, b) => a + b,
                EitherOrBoth::Left(a) => a,
                EitherOrBoth::Right(b) => b,
            })
            .map(CarriedDigit::AddedDigits)
            // add one element to the end of the sequence for a possible final carry
            .chain(Some(CarriedDigit::FinalCarry))
            // perform carry over to the next element
            .scan(0, |carry, carried_digits| match carried_digits {
                CarriedDigit::AddedDigits(added_digits) => {
                    let digit = (added_digits + *carry) % 10;
                    *carry = (added_digits + *carry) / 10;

                    Some(digit)
                }
                CarriedDigit::FinalCarry => {
                    if *carry > 0 {
                        Some(*carry)
                    } else {
                        None
                    }
                }
            })
            .collect::<Box<ListNode>>();

        Some(list)
    }
}
// -------- implement IntoIter for Box<ListNode> --------

pub struct ListNodeIterator(Box<ListNode>);

impl Iterator for ListNodeIterator {
    type Item = i32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next.take();

        #[allow(clippy::question_mark)]
        if next.is_none() {
            return None;
        }

        self.0 = next.unwrap();

        Some(self.0.val)
    }
}

impl IntoIterator for Box<ListNode> {
    type Item = i32;
    type IntoIter = ListNodeIterator;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ListNodeIterator(Box::new(ListNode {
            val: -1, // dummy value, iteration will be started from the next element
            next: Some(self),
        }))
    }
}

impl FromIterator<i32> for Box<ListNode> {
    /// Panics when iter.len() < 1
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let first_val = iter.next().unwrap();

        let mut wrapper_list = Box::new(ListNode {
            val: first_val,
            next: None,
        });

        let mut next_node = &mut wrapper_list.next;

        for x in iter {
            let list = Box::new(ListNode::new(x));
            next_node = &mut next_node.insert(list).next;
        }

        wrapper_list
    }
}

// -------- implement zip_longest iterator ---------

pub enum EitherOrBoth<T, U> {
    Left(T),
    Right(U),
    Both(T, U),
}

#[derive(Clone, Debug)]
pub struct ZipLongest<T: Iterator, U: Iterator> {
    a: Fuse<T>,
    b: Fuse<U>,
}

impl<T, U> Iterator for ZipLongest<T, U>
where
    T: Iterator,
    U: Iterator,
{
    type Item = EitherOrBoth<T::Item, U::Item>;

    /// Acts like a zip function but continues to produce single values
    /// instead of pairs after one of the underlying iterators is exhausted
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (Some(a), None) => Some(EitherOrBoth::Left(a)),
            (None, Some(b)) => Some(EitherOrBoth::Right(b)),
            (Some(a), Some(b)) => Some(EitherOrBoth::Both(a, b)),
        }
    }
}

pub fn zip_longest<T, U>(a: T, b: U) -> ZipLongest<T, U>
where
    T: Iterator,
    U: Iterator,
{
    ZipLongest {
        a: Iterator::fuse(a),
        b: Iterator::fuse(b),
    }
}

// -------- Tests --------

#[cfg(test)]
mod tests {
    use super::*;

    impl ListNode {
        pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
            v.iter().fold(None, |acc, &val| {
                Some(Box::new(ListNode { val, next: acc }))
            })
        }
    }

    #[test]
    fn example1() {
        let l1 = ListNode::from_vec(vec![3, 4, 2]);
        let l2 = ListNode::from_vec(vec![4, 6, 5]);
        let res = ListNode::from_vec(vec![8, 0, 7]);

        assert_eq!(Solution::add_two_numbers(l1, l2), res);
    }

    #[test]
    fn example2() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let res = Some(Box::new(ListNode::new(0)));

        assert_eq!(Solution::add_two_numbers(l1, l2), res);
    }

    #[test]
    fn example3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let res = ListNode::from_vec(vec![1, 0, 0, 0, 9, 9, 9, 8]);

        assert_eq!(Solution::add_two_numbers(l1, l2), res);
    }

    #[test]
    fn example4() {
        let l1 = ListNode::from_vec(vec![9, 4, 2]);
        let l2 = ListNode::from_vec(vec![9, 4, 6, 5]);
        let res = ListNode::from_vec(vec![1, 0, 4, 0, 7]);

        assert_eq!(Solution::add_two_numbers(l1, l2), res);
    }
}
