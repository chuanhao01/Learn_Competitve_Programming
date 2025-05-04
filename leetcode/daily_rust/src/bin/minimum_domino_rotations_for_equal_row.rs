// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/?envType=daily-question&envId=2025-05-04
struct Solution {}
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut mm = -1;
        for i in 0..4 {
            let (t, c, mut n) = if i == 0 {
                (true, tops[0], 0)
            } else if i == 1 {
                (true, bottoms[0], 1)
            } else if i == 2 {
                (false, tops[0], 1)
            } else if i == 3 {
                (false, bottoms[0], 0)
            } else {
                panic!()
            };
            for ii in 1..tops.len() {
                if t {
                    if tops[ii] == c {
                        continue;
                    } else if bottoms[ii] == c {
                        n += 1;
                    } else {
                        n = -1;
                        break;
                    }
                } else {
                    if bottoms[ii] == c {
                        continue;
                    } else if tops[ii] == c {
                        n += 1;
                    } else {
                        n = -1;
                        break;
                    }
                }
            }
            if n == -1 {
                continue;
            }
            if mm == -1 {
                mm = n;
            } else {
                mm = mm.min(n);
            }
        }
        // println!("{}", mm);
        mm
    }
}

fn main() {
    // Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]);
    // Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]);
    Solution::min_domino_rotations(vec![1, 2, 1, 1, 1, 2, 2, 2], vec![2, 1, 2, 2, 2, 2, 2, 2]);
}
