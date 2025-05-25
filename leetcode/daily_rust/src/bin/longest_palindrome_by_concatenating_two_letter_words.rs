//https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/?envType=daily-question&envId=2025-05-02
struct Solution {}
use std::collections::HashMap;
// impl Solution {
//     pub fn longest_palindrome(words: Vec<String>) -> i32 {
//         let mut ans = 0;
//         let mut opposite: HashMap<String, i64> = HashMap::new();
//         for word in words {
//             // Check if we can remove it as a pair
//             match opposite.get(&word) {
//                 Some(wc) => {
//                     if *wc == 1 {
//                         opposite.remove(&word);
//                     } else {
//                         opposite.entry(word).and_modify(|wc| *wc -= 1);
//                     }
//                     ans += 4;
//                 }
//                 None => {
//                     opposite
//                         .entry(word.chars().rev().collect::<String>())
//                         .and_modify(|wc| *wc += 1)
//                         .or_insert(1);
//                 }
//             }
//         }
//         for (word, _) in opposite {
//             let mut w = word.chars();
//             if w.next().unwrap() == w.next().unwrap() {
//                 ans += 2;
//                 break;
//             }
//         }
//         ans
//     }
// }
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut wm = vec![vec![0; 26]; 26];
        let mut ans = 0;
        let mut middle = 0;
        for word in words {
            let mut w = word.chars();
            let x = w.next().unwrap() as usize - 'a' as usize;
            let y = w.next().unwrap() as usize - 'a' as usize;
            if wm[y][x] > 0 {
                wm[y][x] -= 1;
                if x == y {
                    middle -= 1;
                }
                ans += 4;
            } else {
                wm[x][y] += 1;
                if x == y {
                    middle += 1;
                }
            }
        }
        if middle > 0 { ans + 2 } else { ans }
    }
}

fn main() {
    Solution::longest_palindrome(vec!["lc".to_string(), "cl".to_string(), "gg".to_string()]);
}
