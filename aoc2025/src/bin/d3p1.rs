use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d3.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    // let mut instructions: Vec<(i64, i64)> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty(){
            break;
        }
        let line = l.chars().map(|c| c.to_string().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let mut first_seen: Vec<i64> = vec![-1; 10];
        for i in 0..line.len(){
            let n = line[i] as usize;
            if first_seen[n] == -1{
                first_seen[n] = i as i64;
            }
        }
        // Take first not from the back
        let first = (0..10).rev().map(|i| first_seen[i]).filter(|i| *i != -1).find(|i| *i != (line.len() as i64 - 1)).unwrap();
        // println!("{:?}, {}", first_seen, first);
        let mut first_seen: Vec<i64> = vec![-1; 10];
        for i in (first + 1) as usize..line.len(){
            let n = line[i] as usize;
            if first_seen[n] == -1{
                first_seen[n] = i as i64;
            }
        }

        let second = (0..10).rev().map(|i| first_seen[i]).filter(|i| *i != -1).find(|i| *i > first).unwrap();
        let n: u64 = format!("{}{}", line[first as usize], line[second as usize]).parse().unwrap();
        sum += n;
    }

    println!("sum: {}", sum);
    Ok(())
}
