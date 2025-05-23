// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/?envType=daily-question&envId=2025-05-23
struct Solution {}
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut nums = nums;
        // Just try all once haha
        let mut change = false;
        while !change {
            change = true;
            for edge in &edges {
                let ui = edge[0] as usize;
                let vi = edge[1] as usize;
                let u = nums[ui];
                let v = nums[vi];
                let nu = u ^ k;
                let nv = v ^ k;
                if (nu + nv) > (u + v) {
                    nums[ui] = nu;
                    nums[vi] = nv;
                    change = false;
                }
            }
        }
        nums.iter().sum::<i32>() as i64
    }
}

fn main() {}
