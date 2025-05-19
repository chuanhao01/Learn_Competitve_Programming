//https://leetcode.com/problems/sort-colors/description/?envType=daily-question&envId=2025-05-17
struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut l = 0;
        let mut m = 0;
        let mut r = (nums.len() - 1) as i32;
        if nums.len() == 1 {
            return;
        }
        while m as i32 <= r {
            // If 0, swap low and mid, move both up
            // We ensure that l is always 0
            if nums[m] == 0 {
                (nums[l], nums[m]) = (nums[m], nums[l]);
                l += 1;
                m += 1;
            } else if nums[m] == 1 {
                m += 1;
            } else {
                // since m is 2, we swap with right and move right down
                (nums[m], nums[r as usize]) = (nums[r as usize], nums[m]);
                r -= 1;
            }
        }
    }
}

fn main() {
    Solution::sort_colors(&mut vec![1, 2, 0]);
}
