// https://leetcode.com/problems/top-k-frequent-elements

pub struct Solution;

impl Solution {
    // time: O(n), space: O(n)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        // worst space: O(n)
        let mut num_count = HashMap::new();
        // time: O(n)
        for &n in &nums {
            num_count
                .entry(n)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        // space: O(n)
        let mut group_by_count_asc = vec![Vec::new(); nums.len()];
        // time: O(n)
        for (n, count) in num_count {
            // no elem can have count 0, so index 0 is definitely vacant, hence - 1
            let idx = count - 1;
            group_by_count_asc[idx].push(n);
        }

        // time: O(n)
        group_by_count_asc
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_unordered(a: Vec<i32>, b: Vec<i32>) {
        let a = a.iter().collect::<std::collections::HashSet<_>>();
        let b = b.iter().collect::<std::collections::HashSet<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn example1() {
        assert_eq_unordered(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2],
        );
    }

    #[test]
    fn example2() {
        assert_eq_unordered(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn example3() {
        assert_eq_unordered(Solution::top_k_frequent(vec![1, 1, 1, 1, 1], 1), vec![1]);
    }
}
