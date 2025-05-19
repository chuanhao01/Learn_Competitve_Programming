// https://leetcode.com/problems/type-of-triangle/description/?envType=daily-question&envId=2025-05-19

/// triangle inequality theorem, sum of 2 sides > last side
/// Since we sort, smallest 2 must be greater than last side, as the rest will hold true
struct Solution {}
impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort();
        if nums[0] == nums[1] && nums[1] == nums[2] {
            String::from("equilateral")
        } else {
            if nums[0] + nums[1] > nums[2] {
                if nums[0] == nums[1] || nums[1] == nums[2] {
                    String::from("isosceles")
                } else {
                    String::from("scalene")
                }
            } else {
                String::from("none")
            }
        }
    }
}

fn main() {}
