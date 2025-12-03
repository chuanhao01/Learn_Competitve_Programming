use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d3.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    // let mut instructions: Vec<(i64, i64)> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            break;
        }
        let line = l
            .chars()
            .map(|c| c.to_string().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut left_so_far: i64 = -1;
        let mut nums: Vec<u64> = Vec::new();
        for offset_from_back in (1..13).rev(){
        // for offset_from_back in (1..3).rev(){
            let largest_possible = line.len() - offset_from_back;
            let first_seen = first_seen_arr(&line, (left_so_far + 1) as usize);
            // println!("{:?}, {}", first_seen, left_so_far);
            let num = (0..10)
                .rev()
                .map(|i| first_seen[i])
                .filter(|i| *i != -1)
                .find(|i| (*i > left_so_far) && (*i <= largest_possible as i64))
                .unwrap();
            left_so_far = num;
            nums.push(line[num as usize]);
        }
        let n: u64 = nums.iter().map(|n| n.to_string()).join("").parse().unwrap();
        println!("{}", n);
        sum += n;
    }

    println!("sum: {}", sum);
    Ok(())
}

fn first_seen_arr(line: &Vec<u64>, starting_index: usize) -> Vec<i64> {
    let mut first_seen: Vec<i64> = vec![-1; 10];
    for i in starting_index..line.len() {
        let n = line[i] as usize;
        if first_seen[n] == -1 {
            first_seen[n] = i as i64;
        }
    }
    return first_seen;
}
