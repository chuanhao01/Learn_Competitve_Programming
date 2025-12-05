use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d5.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut sum = 0;
    let mut lines = input.split('\n');
    loop {
        let l = lines.next().unwrap();
        if l.is_empty() {
            break;
        }
        let range = l
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>();
        ranges.push((range[0], range[1]));
    }

    println!("{:?}", ranges);

    for l in lines {
        if l.is_empty() {
            break;
        }
        let num: u64 = l.parse().unwrap();
        for range in ranges.iter() {
            if range.0 <= num && num <= range.1 {
                sum += 1;
                break;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
