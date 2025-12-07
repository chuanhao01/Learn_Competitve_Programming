use std::{
    fs::File,
    io::{Read, Result},
};

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
        // let mut new_beams: Vec<usize> = beams.clone();
        for x in 0..map[0].len() {
            if beams[x] > 0 && map[y][x] == '^' {
                if x > 0 {
                    beams[x - 1] += beams[x];
                }
                if x + 1 < map[0].len() {
                    beams[x + 1] += beams[x];
                }
                beams[x] = 0;
            }
        }
        println!("{:?}", beams);
        // sum += new_beams.iter().filter(|c| **c > 0).count();
        // beams = new_beams;
    }

    sum += beams.into_iter().reduce(|acc, n| acc + n).unwrap();

    println!("sum: {}", sum);
    Ok(())
}
