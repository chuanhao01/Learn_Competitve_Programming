// https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/description/?envType=daily-question&envId=2025-05-10
struct Solution {}
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let nums1 = nums1.iter().map(|n| *n as i64).collect::<Vec<i64>>();
        let nums2 = nums2.iter().map(|n| *n as i64).collect::<Vec<i64>>();
        let sum1 = nums1.iter().sum();
        let sum2 = nums2.iter().sum();
        let count1 = nums1.iter().filter(|n| **n == 0).count() as i64;
        let count2 = nums2.iter().filter(|n| **n == 0).count() as i64;
        // println!("{}, {}", count1, count2);
        if sum1 == sum2 {
            // Either we have atleast 1 count in each or none in either
            if count1 == 0 && count2 == 0 {
                return sum1;
            }
            if count1 > 0 && count2 > 0 {
                return sum1 + count1.max(count2);
            }
            return -1;
        }
        let a = if sum1 > sum2 {
            ((sum1, count1), (sum2, count2))
        } else {
            ((sum2, count2), (sum1, count1))
        };
        // println!("{:?}", a);
        let diff = a.0.0 - a.1.0;
        if a.0.1 == 0 {
            if a.1.1 == 0 {
                return -1;
            }
            if a.1.1 > diff {
                return -1;
            }
            return a.0.0;
        }
        if a.1.1 == 0 {
            return -1;
        }
        if a.1.1 > a.0.1 + diff {
            return a.1.0 + a.1.1;
        }
        a.0.0 + a.0.1
    }
}

fn main() {
    let a = Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]);
    println!("{}", a);
    let a = Solution::min_sum(vec![2, 0, 2, 0], vec![4, 1]);
    println!("{}", a);
    let a = Solution::min_sum(
        vec![
            20, 0, 18, 11, 0, 0, 0, 0, 0, 0, 17, 28, 0, 11, 10, 0, 0, 15, 29,
        ],
        vec![16, 9, 25, 16, 1, 9, 20, 28, 8, 0, 1, 0, 1, 27],
    );
    println!("{}", a);
}
