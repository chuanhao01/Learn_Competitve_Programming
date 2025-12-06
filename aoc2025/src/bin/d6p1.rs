use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

fn main() -> Result<()> {
    // let mut file_input = File::open("inputs/d6.txt")?;
    let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = input.split("\n").map(|s| s.chars().collect()).collect();
    if grid.last().unwrap().is_empty() {
        grid.pop();
    }
    let mut cuts: Vec<usize> = vec![0];
    for x in 0..grid[0].len() {
        if grid[0][x] == ' ' {
            // Check if the whole column is spaces
            if (0..grid.len()).all(|y| grid[y][x] == ' ') {
                cuts.push(x);
            }
        }
    }
    cuts.push(grid[0].len());
    println!("{:?}", cuts);
    let mut equations: Vec<(Vec<usize>, char)> = Vec::new();
    for i in 0..cuts.len() - 1 {
        // println!("{:?}", (cuts[i]..cuts[i + 1]).collect_vec());
        let mut nums: Vec<usize> = Vec::new();
        for y in 0..grid.len() - 1 {
            let n: usize = grid[y][cuts[i]..cuts[i + 1]]
                .iter()
                .map(|s| s.to_string())
                .join("")
                .trim()
                .parse()
                .unwrap();
            nums.push(n);
        }
        equations.push((
            nums,
            grid[grid.len() - 1][cuts[i]..cuts[i + 1]]
                .iter()
                .find(|c| **c != ' ')
                .unwrap()
                .clone(),
        ));
    }
    println!("{:?}", equations);
    for equation in equations {
        let nums = equation.0;
        let op = equation.1;
        if op == '+' {
            sum += nums
                .clone()
                .into_iter()
                .reduce(|acc, next| acc + next)
                .unwrap();
        } else if op == '*' {
            sum += nums
                .clone()
                .into_iter()
                .reduce(|acc, next| acc * next)
                .unwrap();
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
