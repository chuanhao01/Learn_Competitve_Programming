//https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/?envType=daily-question&envId=2025-05-02
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut opposite: HashMap<String, i64> = HashMap::new();
        for word in words {
            // Check if we can remove it as a pair
            match opposite.get(&word) {
                Some(wc) => {
                    if *wc == 1 {
                        opposite.remove(&word);
                    } else {
                        opposite.entry(word).and_modify(|wc| *wc -= 1);
                    }
                    ans += 4;
                }
                None => {
                    opposite
                        .entry(word.chars().rev().collect::<String>())
                        .and_modify(|wc| *wc += 1)
                        .or_insert(1);
                }
            }
        }
        for (word, _) in opposite {
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
