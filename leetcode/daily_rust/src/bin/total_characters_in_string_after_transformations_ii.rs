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
        // println!("{:?}", counts);

        let mut transformation_matrix = [[0i64; 26]; 26];
        for i in 0..26 {
            for j in 0..nums[i] {
                transformation_matrix[i][(1 + i + j as usize) % 26] += 1;
            }
        }
        // println!(
        //     "{}",
        //     transformation_matrix
        //         .map(|r| r.map(|e| e.to_string()).join(", "))
        //         .join("\n")
        // );

        fn mat_mul(a: [[i64; 26]; 26], b: [[i64; 26]; 26]) -> [[i64; 26]; 26] {
            let MOD = 10i64.pow(9) + 7;
            let mut ans = [[0i64; 26]; 26];
            for i in 0..26 {
                for j in 0..26 {
                    for k in 0..26 {
                        ans[i][j] += a[i][k] * b[k][j];
                        ans[i][j] %= MOD;
                    }
                }
            }
            ans
        }
        fn mat_pow(a: [[i64; 26]; 26], pow: i32) -> [[i64; 26]; 26] {
            if pow == 0 {
                let mut im = [[0i64; 26]; 26];
                for i in 0..26 {
                    for j in 0..26 {
                        let e = if i == j { 1 } else { 0 };
                        im[i][j] = e;
                    }
                }
                return im;
            } else if pow == 1 {
                return a;
            }
            if pow % 2 == 0 {
                let aa = mat_pow(a, pow / 2);
                mat_mul(aa, aa)
            } else {
                let aa = mat_pow(a, pow - 1);
                mat_mul(a, aa)
            }
        }
        let final_transformation_matrix = mat_pow(transformation_matrix, t);

        let mut ans = [0i64; 26];
        for i in 0..26 {
            for j in 0..26 {
                ans[i] += counts[j] * final_transformation_matrix[j][i];
                ans[i] %= MOD;
            }
        }

        let ans: i64 = ans.iter().sum();
        // println!("{}", (ans % MOD) as i32);
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
    Solution::length_after_transformations(
        String::from("u"),
        5,
        vec![
            1, 1, 2, 2, 3, 1, 2, 2, 1, 2, 3, 1, 2, 2, 2, 2, 3, 3, 3, 2, 3, 2, 3, 2, 2, 3,
        ],
    );
}
