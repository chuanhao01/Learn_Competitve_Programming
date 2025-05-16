// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/description/?envType=daily-question&envId=2025-05-16
struct Solution {}
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn ham_dist_1(a: &String, b: &String) -> bool {
            if a.len() != b.len() {
                return false;
            }
            let a = a.chars().collect::<Vec<char>>();
            let b = b.chars().collect::<Vec<char>>();
            let mut ham_dist = 0;
            for i in 0..a.len() {
                ham_dist += if a[i] == b[i] { 0 } else { 1 };
            }
            ham_dist == 1
        }

        let mut dp: Vec<i32> = vec![0; groups.len()];
        let mut parent: Vec<Option<usize>> = vec![None; groups.len()];
        for i in 0..groups.len() {
            dp[i] = 1; // Longest is itself
            // Starting from j to i, if we can add i to the longest subseq so far
            // We try and update the longest so far at i
            for j in 0..i {
                if groups[j] != groups[i] && ham_dist_1(&words[j], &words[i]) && dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                    parent[i] = Some(j);
                }
            }
        }
        let maxdp = dp.iter().max().unwrap();
        let mut maxi = 0;
        for i in (0..dp.len()).rev() {
            if *maxdp == dp[i] {
                maxi = i;
                break;
            }
        }
        let mut ans: Vec<String> = Vec::new();
        ans.push(words[maxi].clone());
        while let Some(i) = parent[maxi] {
            ans.push(words[i].clone());
            maxi = i;
        }
        ans.reverse();
        // println!("{:?}", ans);
        ans
    }
}

fn main() {
    Solution::get_words_in_longest_subsequence(
        vec!["bab".to_string(), "dab".to_string(), "cab".to_string()],
        vec![1, 2, 2],
    );
}
