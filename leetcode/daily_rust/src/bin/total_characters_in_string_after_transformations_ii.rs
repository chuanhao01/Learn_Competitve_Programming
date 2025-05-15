// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/?envType=daily-question&envId=2025-05-14
struct Solution {}
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let MOD = 10i64.pow(9) + 7;
        // println!("{}", MOD);
        let mut counts = s.chars().fold([0i64; 26], |mut acc, c| {
            acc[c as usize - 97] += 1;
            acc
        });
        for kk in 0..t {
            let mut new_counts = [0i64; 26];
            for i in 0..26 {
                for j in 1..(nums[i] + 1) {
                    let j = j as usize;
                    new_counts[(i + j) % 26] = (new_counts[(i + j) % 26] + counts[i]) % MOD;
                }
            }
            counts = new_counts;
        }
        // println!("{:?}", counts);
        let ans: i64 = counts.iter().sum();
        println!("{}", (ans % MOD) as i32);
        (ans % MOD) as i32
    }
}

fn main() {
    Solution::length_after_transformations(
        String::from("abcyy"),
        2,
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        ],
    );
    Solution::length_after_transformations(
        String::from("abc"),
        1,
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    );
}
