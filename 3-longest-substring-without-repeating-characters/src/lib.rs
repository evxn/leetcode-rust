// https://leetcode.com/problems/longest-substring-without-repeating-characters/

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut current_substring: Vec<u8> = Vec::with_capacity(s.len());
        let mut max_length = 0;

        for byte in s.bytes() {
            if let Some(pos) = current_substring.iter().position(|el| el == &byte) {
                current_substring.drain(0..=pos);
            }
            current_substring.push(byte);
            max_length = std::cmp::max(current_substring.len(), max_length);
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }
}
