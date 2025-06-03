// https://leetcode.com/problems/candy/description/?envType=daily-question&envId=2025-06-03
struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy = vec![1; ratings.len()];
        for i in 0..ratings.len() - 1 {
            if ratings[i + 1] > ratings[i] {
                candy[i + 1] = candy[i] + 1;
            } else {
                candy[i + 1] = 1;
            }
        }
        for i in (1..ratings.len()).rev() {
            if ratings[i - 1] > ratings[i] {
                if candy[i - 1] <= candy[i] {
                    candy[i - 1] = candy[i] + 1;
                }
            }
        }
        candy.iter().sum()
    }
}

fn main() {}
