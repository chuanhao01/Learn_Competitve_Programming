use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d12.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let counts = [5, 7, 7, 7, 6, 2];
    for l in input.split("\n") {
        if l.is_empty() {
            break;
        }
        let l = l.split(": ").collect::<Vec<&str>>();
        let grid = l[0]
            .split("x")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        let presents = l[1]
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();
        println!("{:?}", grid);
        println!("{:?}", presents);
        println!("{:?}", grid[0] * grid[1]);
        println!(
            "{}",
            presents
                .iter()
                .enumerate()
                .fold(0, |acc, (i, n)| acc + counts[i] * *n)
        );
        println!("");
        if grid[0] * grid[1]
            >= presents
                .iter()
                .enumerate()
                .fold(0, |acc, (i, n)| acc + 9 * *n)
        {
            sum += 1;
        }
    }

    println!("{:?}", sum);
    Ok(())
}
