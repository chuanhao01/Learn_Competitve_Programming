// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/description/?envType=daily-question&envId=2025-05-15
struct Solution {}
impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        if groups.len() == 1 {
            return words;
        }
        let mut l = 0;
        let mut r = 1;
        let mut longest = (0, 0);
        let mut prev = groups[0];
        while r < groups.len() {
            if groups[r as usize] + prev == 1 {
                if (r - l) > (longest.1 - longest.0) {
                    longest = (l, r);
                }
            } else {
                l = r;
            }
            prev = groups[r as usize];
            r += 1;
        }
        // println!("{:?}", longest);
        // println!("{:?}", words[longest.0..longest.1 + 1].to_vec());
        words[longest.0..longest.1 + 1].to_vec()
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
