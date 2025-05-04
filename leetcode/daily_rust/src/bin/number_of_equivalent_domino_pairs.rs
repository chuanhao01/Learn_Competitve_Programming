// https://leetcode.com/problems/number-of-equivalent-domino-pairs/?envType=daily-question&envId=2025-05-04
struct Solution {}
// TLE
// impl Solution {
//     pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
//         let mut n = 0;
//         let mut l = 0;
//         while l < dominoes.len() {
//             let mut r = l + 1;
//             while r < dominoes.len() {
//                 let ll = &dominoes[l];
//                 let rr = &dominoes[r];
//                 if ll[0] == rr[0] && ll[1] == rr[1] {
//                     n += 1;
//                 } else if ll[0] == rr[1] && ll[1] == rr[0] {
//                     n += 1;
//                 }
//                 r += 1;
//             }
//             l += 1;
//         }
//         println!("{}", n);
//         n
//     }
// }

use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        fn ch(n: i32, k: i32) -> i32 {
            if k > n {
                return 0;
            }

            let mut result = 1;
            let mut k = k;

            // Take advantage of symmetry: C(n, k) == C(n, n-k)
            if k > n - k {
                k = n - k;
            }

            for i in 0..k {
                result = result * (n - i) / (i + 1);
            }

            result
        }

        let mut n = 0;
        let mut ss: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        for (i, pair) in dominoes.iter().enumerate() {
            let p = if pair[0] < pair[1] {
                (pair[0], pair[1])
            } else {
                (pair[1], pair[0])
            };
            ss.entry(p)
                .and_modify(|e| {
                    e.push(i);
                })
                .or_insert(vec![i]);
        }
        for (_, v) in ss {
            n += ch(v.len() as i32, 2);
        }
        println!("{}", n);
        n
    }
}

fn main() {
    Solution::num_equiv_domino_pairs(vec![
        vec![1, 2],
        vec![1, 2],
        vec![1, 1],
        vec![1, 2],
        vec![2, 2],
    ]);
    Solution::num_equiv_domino_pairs(vec![
        vec![1, 1],
        vec![2, 2],
        vec![1, 1],
        vec![1, 2],
        vec![1, 2],
        vec![1, 1],
    ]);
}
