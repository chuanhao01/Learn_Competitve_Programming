// https://leetcode.com/problems/three-consecutive-odds/?envType=daily-question&envId=2025-05-11
struct Solution {}
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut i = 0;
        let mut count = 0;
        while i < arr.len() {
            if arr[i] % 2 == 1 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
            i += 1;
        }
        false
    }
}

fn main() {
    let a = Solution::three_consecutive_odds(vec![2, 6, 4, 1]);
    println!("{}", a);
}
