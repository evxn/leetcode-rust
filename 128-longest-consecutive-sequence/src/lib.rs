// https://leetcode.com/problems/longest-consecutive-sequence

pub struct Solution;

impl Solution {
    // time: O(n), max space: O(n)
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut uniq_nums: HashSet<_> = nums.into_iter().collect();

        let mut max_seq_len = 0;

        // proper O(n) loop
        while let Some(&n) = uniq_nums.iter().next() {
            uniq_nums.remove(&n);

            // n is part of a new seq
            let mut seq_len = 1;

            let mut current = n;
            // remove and count elements in the ascending order
            while uniq_nums.remove(&(current + 1)) {
                current += 1;
                seq_len += 1;
            }

            // reset current to n
            current = n;
            // remove and count elements in the decending order
            while uniq_nums.remove(&(current - 1)) {
                current -= 1;
                seq_len += 1;
            }

            max_seq_len = max_seq_len.max(seq_len);
        }

        // NB: simpler O(n²) loop performs much faster on leetcode testsuite
        // for &n in &uniq_nums {
        //      if !uniq_nums.contains(&(n - 1)) {
        //          // n is a start of a new seq
        //          let mut seq_len = 1;
        //          while uniq_nums.contains(&(n + seq_len)) {
        //              seq_len += 1;
        //          }
        //
        //          max_seq_len = max_seq_len.max(seq_len);
        //      }
        //  }

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
