// https://leetcode.com/problems/longest-palindromic-substring/

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        use std::str;

        if s.is_empty() || s.len() == 1 {
            return s;
        }

        for size in (1..=s.len()).rev() {
            if let Some(palindrome) = s.as_bytes().windows(size).find(Self::is_palindrome) {
                // note: unwrap is save here cause `s` contains only english letters and digits
                let palindrome = str::from_utf8(palindrome).unwrap();
                return String::from(palindrome);
            }
        }

        String::from("")
    }

    /// Operates on slice of slice
    fn is_palindrome(&substr: &&[u8]) -> bool {
        let reversed_substr = substr.iter().rev();
        let mut both_ends = substr.iter().zip(reversed_substr);
        // b"121" -> (b'1', b'1'), (b'2', b'2'), (b'1', b'1') -> true
        // b"123" -> (b'1', b'3'), stop processing -> false
        both_ends.all(|(left, right)| left == right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(
            Solution::longest_palindrome("babad".to_string()) == "bab".to_string()
                || Solution::longest_palindrome("babad".to_string()) == "aba".to_string()
        );
    }

    #[test]
    fn example2() {
        assert!(Solution::longest_palindrome("cbbd".to_string()) == "bb".to_string());
    }

    #[test]
    fn example3() {
        assert!(Solution::longest_palindrome("".to_string()) == "".to_string());
    }

    #[test]
    fn example4() {
        assert!(Solution::longest_palindrome("f".to_string()) == "f".to_string());
    }

    #[test]
    fn example5() {
        assert!(Solution::longest_palindrome("omoromo".to_string()) == "omoromo".to_string());
    }

    #[test]
    fn example6() {
        assert!(Solution::longest_palindrome("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_string()) == "ranynar".to_string());
    }
}
