// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/?envType=daily-question&envId=2025-05-01
struct Solution {}
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|num| num.to_string().chars().count() % 2 == 0)
            .count() as i32
    }
}

fn main() {}
