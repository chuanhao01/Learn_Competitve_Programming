// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/?envType=daily-question&envId=2025-05-31
struct Solution {}
use std::collections::VecDeque;
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn gen_dist(edges: &Vec<i32>, node: i32) -> Vec<i32> {
            let n = edges.len();
            let mut n1_dist = vec![-1; n];
            n1_dist[node as usize] = 0;
            let mut q = VecDeque::new();
            q.push_back(node);
            while let Some(e) = q.pop_front() {
                let ne = edges[e as usize];
                if ne == -1 {
                    continue;
                }
                if n1_dist[ne as usize] == -1 {
                    // Havent been here
                    n1_dist[ne as usize] = n1_dist[e as usize] + 1;
                    q.push_back(ne);
                } else if n1_dist[e as usize] + 1 < n1_dist[ne as usize] {
                    n1_dist[ne as usize] = n1_dist[e as usize] + 1;
                    q.push_back(ne);
                }
            }
            n1_dist
        }
        let n1_dist = gen_dist(&edges, node1);
        let n2_dist = gen_dist(&edges, node2);
        let mut smallest_so_far = -1;
        let mut idx_so_far = -1;
        for i in 0..edges.len() {
            let n1 = n1_dist[i];
            let n2 = n2_dist[i];
            if n1 == -1 || n2 == -1 {
                continue;
            }
            if smallest_so_far == -1 {
                smallest_so_far = n1.max(n2);
                idx_so_far = i as i32;
            } else if n1.max(n2) < smallest_so_far {
                smallest_so_far = n1.max(n2);
                idx_so_far = i as i32;
            }
        }
        idx_so_far
    }
}

fn main() {}
