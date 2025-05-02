// https://leetcode.com/problems/push-dominoes/description/?envType=daily-question&envId=2025-05-02
struct Solution {}
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.chars().collect::<Vec<char>>();
        let mut tic_check: Vec<(usize, char)> = dominoes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .collect();
        while !tic_check.is_empty(){
        }
        println!("{:?}", tic_check);
        return String::new();
    }
}

fn main() {
    Solution::push_dominoes(String::from("RR.L"));
}
