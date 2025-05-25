//https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/?envType=daily-question&envId=2025-05-02
struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut opposite: HashSet<String> = HashSet::new();
        for word in words {
            if opposite.contains(&word) {
                opposite.remove(&word);
                ans += 4;
            } else {
                opposite.insert(word.chars().rev().collect::<String>());
            }
        }
        for word in opposite {
            let mut w = word.chars();
            if w.next().unwrap() == w.next().unwrap() {
                ans += 2;
                break;
            }
        }
        ans
    }
}

fn main() {}
