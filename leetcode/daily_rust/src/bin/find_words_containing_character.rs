//https://leetcode.com/problems/find-words-containing-character/description/?envType=daily-question&envId=2025-05-24
struct Solution {}
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..words.len() {
            let word = &words[i];
            if word.contains(x) {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() {}
