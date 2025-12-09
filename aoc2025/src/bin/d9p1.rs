use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{Read, Result},
    time::Instant,
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d9.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;

    let cords: Vec<(usize, usize)> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut l = l.split(",");
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    for i in 0..cords.len() {
        for j in (i + 1)..cords.len() {
            let l = cords[i];
            let r = cords[j];
            // println!(
            //     "{:?}, {:?}: {}",
            //     l,
            //     r,
            //     l.0.abs_diff(r.0).max(1) * l.1.abs_diff(r.1).max(1)
            // );
            sum = sum.max((l.0.abs_diff(r.0) + 1) * (l.1.abs_diff(r.1).max(1) + 1));
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
