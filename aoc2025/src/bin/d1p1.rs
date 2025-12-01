use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d1")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut dial = 50;
    let mut sum = 0;
    // let mut instructions: Vec<(i64, i64)> = Vec::new();
    for l in input.split('\n') {
        if l == "" {
            break;
        }
        let mut cs = l.chars();
        let mut dir = match cs.next().unwrap() {
            'L' => 0,
            'R' => 1,
            _ => panic!("nah"),
        };
        let num: i64 = cs.as_str().parse().unwrap();
        // sum += (dial + num) / 100;
        println!("{}", (dial + num) / 100);
        if dir == 0 {
            dial -= num % 100;
            dial = if dial < 0 { 100 + dial } else { dial };
            dial %= 100;
        } else {
            dial += num % 100;
            dial %= 100;
        }
        println!("{}", dial);
        if dial == 0 {
            sum += 1;
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
