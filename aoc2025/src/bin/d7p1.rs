use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d7.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;

    let mut map: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    if map.last().unwrap().is_empty() {
        map.pop();
    }

    let mut beams: Vec<usize> = vec![0; map[0].len()];
    // starting
    beams[map[0].iter().enumerate().find(|c| *c.1 == 'S').unwrap().0] = 1;
    // let mut beams: Vec<usize> = vec![];
    println!("{:?}", beams);

    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if beams[x] == 1 && map[y][x] == '^' {
                sum += 1;
                beams[x] = 0;
                if x > 0 {
                    beams[x - 1] = 1;
                }
                if x + 1 < map[0].len() {
                    beams[x + 1] = 1;
                }
            }
        }
    }

    // sum = beams.iter().filter(|x| **x == 1).count();

    println!("sum: {}", sum);
    Ok(())
}
