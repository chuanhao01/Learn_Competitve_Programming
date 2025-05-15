// https://leetcode.com/problems/total-characters-in-string-after-transformations-i/description/?envType=daily-question&envId=2025-05-13
struct Solution {}
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let MOD = 10i64.pow(9) + 7;
        // println!("{}", MOD);
        let mut counts = s.chars().fold([0i64; 26], |mut acc, c| {
            acc[c as usize - 97] += 1;
            acc
        });
        for _ in 0..t {
            let mut new_counts = [0i64; 26];
            for i in 0..26 {
                if i == 25 {
                    // println!("{:?}", counts);
                    // println!("{:?}", new_counts);
                    // println!("");
                    new_counts[0] = (new_counts[0] + counts[25]) % MOD;
                    new_counts[1] = (new_counts[1] + counts[25]) % MOD;
                    // new_counts[0] %= MOD;
                    // new_counts[1] %= MOD;
                } else {
                    new_counts[i + 1] = counts[i];
                    // new_counts[i + 1] %= MOD;
                }
            }
            counts = new_counts;
        }
        let ans: i64 = counts.iter().sum();
        // println!("{:?}", counts);
        // println!("{}", ans % MOD);
        (ans % MOD) as i32
    }
}

fn main() {
    Solution::length_after_transformations(String::from("abcyy"), 2);
    Solution::length_after_transformations(String::from("abcyy"), 28);
    Solution::length_after_transformations(String::from("jqktcurgdvlibczdsvnsg"), 7517);
}
