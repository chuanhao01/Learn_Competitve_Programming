// https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/description/?envType=daily-question&envId=2025-06-04
struct Solution {}
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        fn compare_word(word1: &Vec<char>, word2: &Vec<char>) -> bool {
            if word1.len() == 0 {
                return false;
            }
            // True if first word is larger
            for i in 0..word1.len().min(word2.len()) {
                let a = word1[i];
                let b = word2[i];
                if a as u32 > b as u32 {
                    return true;
                } else if b as u32 > a as u32 {
                    return false;
                }
            }
            // They are all equal
            word1.len() > word2.len()
        }

        let word = word.chars().collect::<Vec<char>>();
        if num_friends == 1 {
            return word.iter().collect();
        }
        let mut longest_word_so_far: Vec<char> = Vec::new();
        let m = word.len() - num_friends as usize + 1;
        for i in 0..word.len() {
            let back = word.len().min(i + m);
            let new_word = word[i..back].to_vec();
            // println!("{}, {}, {:?}", i, back, new_word);
            if longest_word_so_far.len() == 0 {
                longest_word_so_far = new_word;
            } else {
                if compare_word(&new_word, &longest_word_so_far) {
                    longest_word_so_far = new_word;
                }
            }
        }
        longest_word_so_far.iter().collect()
    }
}

fn main() {
    Solution::answer_string(String::from("nbjnc"), 2);
}
