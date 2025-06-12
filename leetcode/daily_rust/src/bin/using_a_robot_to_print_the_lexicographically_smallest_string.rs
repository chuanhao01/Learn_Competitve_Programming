// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/description/?envType=daily-question&envId=2025-06-06
// Ref: https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/solutions/2678792/python3-stack-counter-o-n-clean-concise/?envType=daily-question&envId=2025-06-06

struct Solution {}
use std::collections::VecDeque;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut s = s.chars().collect::<VecDeque<char>>();
        // We want to know from the back, what is the smallest char so far up to this point
        let mut min_prefix_count = vec![s.back().unwrap().to_owned() as u32; s.len()];
        for i in (0..s.len() - 1).rev() {
            let c = s[i];
            min_prefix_count[i] = min_prefix_count[i + 1].min(c as u32);
        }
        let mut t: VecDeque<char> = VecDeque::new();
        let mut ans = Vec::new();
        for i in 0..s.len() {
            t.push_back(s[i]);
            // Given the next char, if the back of t is smaller, we wanna pop it first before the next char
            while i + 1 < s.len()
                && !t.is_empty()
                && min_prefix_count[i + 1] >= (t.back().unwrap().to_owned() as u32)
            {
                ans.push(t.pop_back().unwrap());
            }
        }
        while !t.is_empty() {
            ans.push(t.pop_back().unwrap());
        }
        ans.iter().collect()
    }
}

fn main() {
    Solution::robot_with_string(String::from("bdda"));
}
