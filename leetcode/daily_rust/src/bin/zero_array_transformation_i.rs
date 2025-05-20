// https://leetcode.com/problems/zero-array-transformation-i/description/?envType=daily-question&envId=2025-05-20
struct Solution {}
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut nums = nums;
        let mut diff_arr = vec![0; nums.len() + 1];
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            diff_arr[l] += 1;
            diff_arr[r + 1] -= 1;
        }
        let mut prefix_sum = 0;
        for i in 0..nums.len() {
            prefix_sum += diff_arr[i];
            nums[i] = (nums[i] - prefix_sum).max(0);
        }
        nums.iter().all(|n| *n == 0)
    }
}

fn main() {
    Solution::is_zero_array(vec![1, 0, 1], vec![vec![0, 2]]);
}
