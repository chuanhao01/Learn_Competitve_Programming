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
        let mut range = l
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>();
        ranges.sort_by_key(|x| x.0);
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        for r in ranges.iter() {
            if (range[0] <= r.0 && r.0 <= range[1])
                || (range[0] <= r.1 && r.1 <= range[1])
                || (r.0 <= range[0] && range[1] <= r.1)
            {
                range[0] = r.0.min(range[0]);
                range[1] = r.1.max(range[1]);
            } else {
                new_ranges.push(r.clone());
            }
        }
        new_ranges.push((range[0], range[1]));
        ranges = new_ranges;
        // ranges.push(());
        // ranges.push((range[0], range[1]));
    }
    // println!("{}", lines.next().unwrap());

    // for i in 0..(ranges.len() - 1) {
    //     let right = ranges[i];
    //     let left = ranges[i + 1];
    //     println!("{}", left.1 - right.0);
    // }
    // println!("{:?}", ranges);
    for range in ranges.iter() {
        // println!("{}", range.1 - range.0 + 1);
        sum += range.1 - range.0 + 1;
    }

    // println!("{:?}", ranges);

    println!("sum: {}", sum);
    Ok(())
}
