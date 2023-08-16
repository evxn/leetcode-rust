// https://leetcode.com/problems/3sum/

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::{cmp::Ordering, collections::HashSet};

        let len = nums.len();

        if len == 3 {
            if nums.iter().sum::<i32>() == 0 {
                return vec![nums];
            } else {
                return vec![];
            }
        }

        nums.sort_unstable();

        let mut res = HashSet::<Vec<i32>>::with_capacity(len / 3);

        for left in 0..len - 2 {
            let mut middle = left + 1;
            let mut right = len - 1;

            while middle < right {
                match (nums[middle] + nums[right]).cmp(&-nums[left]) {
                    Ordering::Less => middle += 1,
                    Ordering::Equal => {
                        res.insert(vec![nums[left], nums[middle], nums[right]]);

                        middle += 1;
                        right -= 1;
                    }
                    Ordering::Greater => right -= 1,
                }
            }
        }

        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_unordered(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {
        let a = a.iter().collect::<std::collections::HashSet<_>>();
        let b = b.iter().collect::<std::collections::HashSet<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn no_results() {
        let res = Solution::three_sum(vec![0, 1, 1]);
        assert_eq_unordered(res, Vec::new());
    }

    #[test]
    fn one_result() {
        let res = Solution::three_sum(vec![0, 0, 0]);
        assert_eq_unordered(res, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn two_results() {
        let res = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq_unordered(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn three_zeros() {
        let res = Solution::three_sum(vec![0, 0, 0]);
        assert_eq_unordered(res, vec![vec![0, 0, 0]]);
    }
}
