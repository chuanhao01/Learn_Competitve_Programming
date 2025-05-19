//https://leetcode.com/problems/painting-a-grid-with-three-different-colors/solutions/6754794/dp-in-depth-with-images-example-walkthro-oapb/
struct Solution {}
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        // m < 5
        // n < 1000
        let MOD = 10i64.pow(9) + 7;
        let total = 3i32.pow(m as u32);
        let mut pattern: Vec<Vec<i64>> = vec![Vec::new(); total as usize];
        let mut valids: Vec<i64> = Vec::new(); // Stores the "index"/permutation number of a good seq
        // Finding all valid columns
        for i in 0..total {
            let mut val = i as i64;
            let mut valid = true;
            // For each permutation, create an array representing the column
            // Per permutation, say 1 -> 1 0 0, 2 -> 2 0 0 and so on with a column of 3
            // 1 2
            // 0 0
            // 0 0
            for _ in 0..m {
                pattern[i as usize].push(val % 3);
                val = val / 3;
            }
            // Sliding window to check adjacent indexes in the column and remove all that are bad
            for j in 1..m as usize {
                if pattern[i as usize][j - 1] == pattern[i as usize][j] {
                    valid = false;
                }
            }
            if valid {
                valids.push(i as i64);
            }
        }
        // y represent which permutation of column in the first row, x represent which permutation no of column in the 2nd row
        // All set to false since we are only checking valid columns
        // Mem is only as large as 3^5 * 3^5 or 243 * 243
        let mut valid_rows = vec![vec![true; valids.len()]; valids.len()];
        // Finding all valid permutations of columns next to each other
        for i in 0..valids.len() {
            for j in 0..valids.len() {
                for k in 0..m as usize {
                    // Checking adj rows element by element
                    if pattern[valids[i] as usize][k] == pattern[valids[j] as usize][k] {
                        valid_rows[i][j] = false;
                        break;
                    }
                }
            }
        }
        // DP from bottom up col dp[col][X] col being no. of columns with last column being X
        // X being index in valids, actual perm no have to check valids Vec
        let mut dp = vec![vec![0; valids.len()]; (n + 1) as usize];
        for i in 0..valids.len() {
            dp[1][i] = 1;
        }

        for col in 2..(n + 1) {
            for j in 0..valids.len() {
                let mut total_ways = 0;
                // We swap i and j since no diff
                for i in 0..valids.len() {
                    // If the rows can be beside each other
                    if valid_rows[i][j] {
                        total_ways += dp[(col - 1) as usize][i];
                        total_ways %= MOD;
                    }
                }
                dp[col as usize][j] = total_ways;
            }
        }

        // println!("{:?}", pattern);
        // println!("{:?}", valid_rows);
        // println!("{:?}", dp);
        let ans = (dp[n as usize].iter().sum::<i64>() % MOD) as i32;
        // println!("{}", ans);
        ans
    }
}

fn main() {
    Solution::color_the_grid(3, 3); // 246
}
