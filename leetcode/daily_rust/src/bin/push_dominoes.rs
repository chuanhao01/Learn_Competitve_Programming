// https://leetcode.com/problems/push-dominoes/description/?envType=daily-question&envId=2025-05-02
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.chars().collect::<Vec<char>>();
        let mut tic_check: Vec<(usize, char)> = dominoes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .collect();
        while !&tic_check.is_empty() {
            let mut new_tics: HashMap<usize, char> = HashMap::new();
            let mut ntc: Vec<(usize, char)> = Vec::new();
            for (i, c) in &tic_check {
                let i = if *c == 'L' {
                    if *i == 0 {
                        continue;
                    }
                    i - 1
                } else if *c == 'R' {
                    if *i == dominoes.len() - 1 {
                        continue;
                    }
                    i + 1
                } else {
                    panic!("should not happen")
                };
                new_tics
                    .entry(i)
                    .and_modify(|cc| {
                        if *cc != 'c' {
                            *cc = '.';
                        }
                    })
                    .or_insert(*c);
            }
            for (ni, nc) in &new_tics {
                if dominoes[*ni] == '.' {
                    dominoes[*ni] = *nc;
                    if *nc != '.' {
                        ntc.push((*ni, *nc));
                    }
                }
            }
            tic_check = ntc;
        }
        println!("{:?}", dominoes);
        return dominoes
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
    }
}

fn main() {
    Solution::push_dominoes(String::from("RR.L"));
    Solution::push_dominoes(String::from(".L.R...LR..L.."));
}
