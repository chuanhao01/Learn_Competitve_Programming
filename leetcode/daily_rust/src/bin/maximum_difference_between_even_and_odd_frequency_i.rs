// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/?envType=daily-question&envId=2025-06-10
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut counts = HashMap::new();
        for c in s.chars() {
            counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        // println!("{:?}", counts);
        let mut max_odd = -1;
        let mut min_even = -1;
        for v in counts.values() {
            let v = *v;
            if v % 2 == 1 {
                if max_odd == -1 {
                    max_odd = v;
                } else {
                    max_odd = max_odd.max(v);
                }
            }

            if v % 2 == 0 {
                if min_even == -1 {
                    min_even = v;
                } else {
                    min_even = min_even.min(v);
                }
            }
        }
        // println!("{}", max_odd - min_even);
        max_odd - min_even
    }
}

fn main() {
    Solution::max_difference("abcabcab".to_string());
    Solution::max_difference("tzt".to_string());
}
