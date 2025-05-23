// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/?envType=daily-question&envId=2025-05-23
struct Solution {}
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut nums = nums.iter().map(|n| *n as i64).collect::<Vec<i64>>();
        let k = k as i64;
        // The trick is that since all nodes have to be connected, any even no. of operations can be done on any node
        // For odd no. we need to do the least sum reducing op

        // Maximize all nums with xor and keep track
        let mut count = 0;
        for i in 0..nums.len() {
            let nn = nums[i] ^ k;
            if nn > nums[i] {
                count += 1;
                nums[i] = nn;
            }
        }
        let s = nums.iter().sum();
        if count % 2 == 0 {
            s
        } else {
            // In this case
            // If we got a bigger n^k, n - n^k would give us the reduction in the max sum turning it back to n
            // If we did not touch it, n - n^k would give us the reduction in max sum
            s - nums.iter().map(|n| (n - (n ^ k))).min().unwrap()
        }
    }
}

fn main() {}
