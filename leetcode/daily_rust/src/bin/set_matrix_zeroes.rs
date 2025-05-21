// https://leetcode.com/problems/set-matrix-zeroes/description/?envType=daily-question&envId=2025-05-21
struct Solution {}
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = false;
        let mut col = false;
        if matrix[0][0] == 0 {
            row = true;
            col = true;
        }
        // Check column and row first
        for j in 1..n {
            if matrix[0][j] == 0 {
                matrix[0][j] = 1;
                row = true;
            } else {
                matrix[0][j] += 1;
            }
        }
        for i in 1..m {
            if matrix[i][0] == 0 {
                matrix[i][0] = 1;
                col = true;
            } else {
                matrix[i][0] += 1;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 1;
                    matrix[0][j] = 1;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 1 {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            } else {
                matrix[0][j] -= 1;
            }
        }
        for i in 1..m {
            if matrix[i][0] == 1 {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            } else {
                matrix[i][0] -= 1;
            }
        }
        // println!("{} {}", col, row);
        if row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
        if col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

fn main() {
    Solution::set_zeroes(&mut vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5],
    ]);
}
