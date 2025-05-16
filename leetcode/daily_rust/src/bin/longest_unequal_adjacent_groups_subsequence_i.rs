// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/description/?envType=daily-question&envId=2025-05-15
struct Solution {}
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        if groups.len() == 1 {
            return words;
        }
        let mut i = 1;
        let mut prev = groups[0];
        let mut ans = Vec::from([words[0].clone()]);
        while i < groups.len() {
            if prev != groups[i] {
                prev = groups[i];
                ans.push(words[i].clone());
            }
            i += 1;
        }
        ans
    }
}

fn main() {
    Solution::get_longest_subsequence(
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
        vec![1, 0, 1, 1],
    );
    Solution::get_longest_subsequence(vec!["lr".to_string(), "h".to_string()], vec![0, 0]);
}
