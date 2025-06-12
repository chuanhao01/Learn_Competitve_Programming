// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/?envType=daily-question&envId=2025-06-03
struct Solution {}
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut left_over: HashSet<usize> = HashSet::new();
        let mut ans = 0;
        let mut status: HashSet<usize> = status
            .into_iter()
            .enumerate()
            .filter(|(_, s)| *s == 1)
            .map(|(i, _)| i)
            .collect();
        let mut q: VecDeque<usize> = initial_boxes.into_iter().map(|i| i as usize).collect();
        while let Some(box_id) = q.pop_front() {
            // Nothing to do, box closed
            if let None = status.get(&box_id) {
                left_over.insert(box_id);
                continue;
            }
            ans += candies[box_id];
            for new_box_status in &keys[box_id] {
                status.insert(*new_box_status as usize);
            }
            for new_box_id in &contained_boxes[box_id] {
                left_over.insert(*new_box_id as usize);
            }
            let left_over_so_far = left_over.clone();
            let could_open = left_over_so_far.intersection(&status);
            for could_open_box_id in could_open {
                q.push_back(*could_open_box_id);
                left_over.remove(could_open_box_id);
            }
        }
        println!("{:?}", left_over);
        println!("{:?}", status);
        ans
    }
}

fn main() {}
