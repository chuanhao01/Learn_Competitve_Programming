use core::str;
use std::{
    fs::File,
    io::{Read, Result},
};

use itertools::Itertools;

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d2.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let mut sum: u64 = 0;

    let l = input.split("\n").next().unwrap();
    for r in l.split(",") {
        let mut r = r.split("-");
        let left = r.next().unwrap();
        let right = r.next().unwrap().parse::<u64>().unwrap();
        // println!("{}, {}", left, right);
        let left_half: Vec<char> = left.chars().collect();
        let mut left_half: u64 = if left_half.len() == 1 {
            1
        } else {
            left_half[..left_half.len() / 2]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap()
        };

        let left = left.parse::<u64>().unwrap();

        // Make sure we start checking from smallest pallindrome larger than left
        while construct_palindrome(left_half) < left {
            left_half += 1;
        }

        // println!("{}", left_half)
        let mut v: Vec<u64> = Vec::new();
        while construct_palindrome(left_half) <= right {
            let left = construct_palindrome(left_half);
            v.push(left);
            sum += left;
            left_half += 1;
        }
        println!("{}, {:?}", left, v);
    }

    println!("sum: {}", sum);
    Ok(())
}

fn construct_palindrome(left_half: u64) -> u64 {
    let mut full_palindrome = left_half.to_string().chars().collect::<Vec<char>>();
    full_palindrome.extend(full_palindrome.clone());
    full_palindrome.iter().collect::<String>().parse().unwrap()
}
