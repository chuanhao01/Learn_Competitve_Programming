// https://leetcode.com/problems/number-of-equivalent-domino-pairs/?envType=daily-question&envId=2025-05-04
struct Solution {}
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut n = 0;
        let mut l = 0;
        while l < dominoes.len() {
            let mut r = l + 1;
            while r < dominoes.len() {
                let ll = &dominoes[l];
                let rr = &dominoes[r];
                if ll[0] == rr[0] && ll[1] == rr[1] {
                    n += 1;
                } else if ll[0] == rr[1] && ll[1] == rr[0] {
                    n += 1;
                }
                r += 1;
            }
            l += 1;
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
}
