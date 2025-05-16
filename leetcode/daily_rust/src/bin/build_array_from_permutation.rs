// https://leetcode.com/problems/build-array-from-permutation/description/?envType=daily-question&envId=2025-05-06
struct Solution {}
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        for i in 0..nums.len() {
            ans[i] = nums[nums[i] as usize];
        }
        ans
    }
}

fn main() {
    let a = Solution::build_array(vec![5, 0, 1, 2, 3, 4]);
    println!("{:?}", a);
}
