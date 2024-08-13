// https://leetcode.com/problems/group-anagrams/

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::{BTreeMap, HashMap};

        let mut res: HashMap<BTreeMap<char, u8>, Vec<String>> = HashMap::new();

        strs.into_iter().for_each(|word| {
            let mut key: BTreeMap<char, u8> = BTreeMap::new();

            for ch in word.chars() {
                key.entry(ch).and_modify(|count| *count += 1).or_insert(1);
            }

            res.entry(key).or_default().push(word);
        });

        res.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example1() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        let res = Solution::group_anagrams(strs);

        let res = res
            .into_iter()
            .map(|group| group.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let target = [
            ["bat"].to_vec(),
            ["nat", "tan"].to_vec(),
            ["ate", "eat", "tea"].to_vec(),
        ]
        .into_iter()
        .map(|group| group.into_iter().map(String::from).collect::<HashSet<_>>())
        .collect::<Vec<_>>();

        assert_eq!(res.len(), target.len());

        target.iter().for_each(|group| {
            assert!(res.contains(group));
        });
    }

    #[test]
    fn example2() {
        let strs = [""].into_iter().map(String::from).collect::<Vec<_>>();

        let res = Solution::group_anagrams(strs);

        let res = res
            .into_iter()
            .map(|group| group.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let target = [[""].to_vec()]
            .into_iter()
            .map(|group| group.into_iter().map(String::from).collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        assert_eq!(res.len(), target.len());

        target.iter().for_each(|group| {
            assert!(res.contains(group));
        });
    }

    #[test]
    fn example3() {
        let strs = ["a"].into_iter().map(String::from).collect::<Vec<_>>();

        let res = Solution::group_anagrams(strs);

        let res = res
            .into_iter()
            .map(|group| group.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let target = [["a"].to_vec()]
            .into_iter()
            .map(|group| group.into_iter().map(String::from).collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        assert_eq!(res.len(), target.len());

        target.iter().for_each(|group| {
            assert!(res.contains(group));
        });
    }
}
