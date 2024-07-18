// https://leetcode.com/problems/longest-consecutive-sequence

pub struct Solution;

impl Solution {
    // time: O(n^2), max space: O(n)
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let uniq_nums: HashSet<_> = nums.into_iter().collect();

        let mut max_seq_len = 0;

        // time O(n)
        for &n in &uniq_nums {
            if !uniq_nums.contains(&(n - 1)) {
                // n is a start of a new seq
                let mut seq_len = 1;

                // time O(n)
                while uniq_nums.contains(&(n + seq_len)) {
                    seq_len += 1;
                }

                max_seq_len = max_seq_len.max(seq_len);
            }
        }

        max_seq_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        let result = Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(result, 9);
    }
}
