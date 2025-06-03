// https://leetcode.com/problems/snakes-and-ladders/?envType=daily-question&envId=2025-05-31
struct Solution {}
use std::collections::VecDeque;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let board: Vec<Vec<i32>> = board.into_iter().rev().collect();
        let n = board.len();
        let mut dp = vec![-1; n * n + 1];
        let mut q = VecDeque::new();
        dp[1] = 0;
        q.push_back(1);
        while let Some(cur) = q.pop_front() {
            for next in (cur + 1)..(cur + 7).min(n * n + 1) {
                let y = (next - 1) / n;
                let x = if y % 2 == 0 {
                    (next - 1) % n
                } else {
                    n - 1 - (next - 1) % n
                };
                let next = if board[y][x] == -1 {
                    next
                } else {
                    board[y][x] as usize
                };
                if dp[next] == -1 {
                    dp[next] = dp[cur] + 1;
                    q.push_back(next);
                }
            }
        }
        println!("{:?}", dp);
        dp[n * n]
    }
}

fn main() {}
