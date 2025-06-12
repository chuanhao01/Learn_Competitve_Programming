// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/?envType=daily-question&envId=2025-06-10
struct Solution {}
impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut diff_so_far = -1;
        for i in 0..nums.len() {
            let num1 = nums[i];
            let num2 = nums[(i + 1) % nums.len()];
            diff_so_far = diff_so_far.max((num2 - num1).abs());
        }
        diff_so_far
    }
}

fn main() {}
