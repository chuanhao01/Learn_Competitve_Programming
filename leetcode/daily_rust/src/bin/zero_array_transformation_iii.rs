// https://leetcode.com/problems/zero-array-transformation-iii/description/?envType=daily-question&envId=2025-05-22
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        // Sorted by left index, then largest right index
        let mut q = queries.clone();
        q.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        for i in 0..nums.len(){
            let diff_arr = vec![0; nums.len() + 1];
            let count = 0;
            while count < nums[i]{
                let 
            }
        }
        2
    }
}

fn main() {
    Solution::max_removal(
        vec![1, 1, 1, 1],
        vec![vec![1, 2], vec![1, 3], vec![0, 2], vec![1, 3]],
    );
}
