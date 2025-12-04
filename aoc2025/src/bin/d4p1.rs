use std::{
    fs::File,
    io::{Read, Result},
};

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
    let cc = vec![
        (1, 1),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let y = y as i64;
            let x = x as i64;
            // println!("{}, {}", y, x);
            if map[y as usize][x as usize] != '@' {
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
            // println!("{} {}, {}", y, x, s);
            if s < 4 {
                sum += 1;
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
