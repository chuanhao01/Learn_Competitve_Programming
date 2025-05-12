// https://leetcode.com/problems/finding-3-digit-even-numbers/?envType=daily-question&envId=2025-05-12
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let counts = digits.iter().fold(HashMap::new(), |mut hm, d| {
            hm.entry(*d).and_modify(|cc| *cc += 1).or_insert(1);
            hm
        });
        // println!("{:?}", counts);
        let mut ans: Vec<i32> = Vec::new();
        for i in 100..1000 {
            if i % 2 != 0 {
                continue;
            }
            let i_counts = i
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .fold(HashMap::new(), |mut hm, d| {
                    hm.entry(d).and_modify(|cc| *cc += 1).or_insert(1);
                    hm
                });

            // println!("{:?}", i);
            // println!("{:?}", i_counts);
            if i_counts.iter().all(|(k, v)| match counts.get(k) {
                None => false,
                Some(vv) => vv >= v,
            }) {
                ans.push(i);
            }
        }
        // println!("{:?}", ans);
        ans
    }
}

fn main() {
    Solution::find_even_numbers(vec![2, 2, 8, 8, 2]);
}
