use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d4.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    for l in input.split('\n') {
        if l.is_empty() {
            break;
        }
        map.push(l.chars().collect());
    }
    let mut num_map: Vec<Vec<u64>> = Vec::new();

    let cc = [
        (1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut q: Vec<(i64, i64)> = Vec::new();
    let mut cs = 0;
    for y in 0..map.len() {
        num_map.push(Vec::new());
        for x in 0..map[0].len() {
            let y = y as i64;
            let x = x as i64;
            if map[y as usize][x as usize] != '@' {
                num_map[y as usize].push(0);
                continue;
            }
            let mut s = 0;
            for c in 0..8 {
                let change = cc[c];
                let cy = y + change.0;
                let cx = x + change.1;
                if cy < 0 || cx < 0 {
                    continue;
                }
                if cy >= map.len() as i64 || cx >= map[0].len() as i64 {
                    continue;
                }
                if map[cy as usize][cx as usize] == '@' {
                    s += 1;
                }
            }
            // println!("{}, {} {}", y, x, s);
            if s < 4 {
                q.push((y, x));
            }
            num_map[y as usize].push(s);
        }
    }
    println!("{:?}", q);

    println!(
        "{}",
        num_map
            .iter()
            .map(|v| v.iter().map(|n| n.to_string()).join(""))
            .join("\n")
    );

    while let Some(cur) = q.pop() {
        let y = cur.0;
        let x = cur.1;
        if num_map[y as usize][x as usize] == 0 {
            // Skip we already checked
            continue;
        }
        num_map[y as usize][x as usize] = 0;
        for c in 0..8 {
            let change = cc[c];
            let cy = y + change.0;
            let cx = x + change.1;
            if cy < 0 || cx < 0 {
                continue;
            }
            if cy >= map.len() as i64 || cx >= map[0].len() as i64 {
                continue;
            }
            let cy = cy as usize;
            let cx = cx as usize;
            if num_map[cy][cx] > 0 {
                num_map[cy][cx] -= 1;
                if num_map[cy][cx] < 4 {
                    q.push((cy as i64, cx as i64));
                }
            }
        }
    }

    println!();
    println!(
        "{}",
        num_map
            .iter()
            .map(|v| v.iter().map(|n| n.to_string()).join(""))
            .join("\n")
    );

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' && num_map[y][x] < 4 {
                sum += 1;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
