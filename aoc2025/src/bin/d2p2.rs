use core::str;
use std::{
    collections::HashSet,
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
        let right = r.next().unwrap();
        let left_num: u64 = left.parse().unwrap();
        let right_num: u64 = right.parse().unwrap();
        // println!("{}, {}, {}, {}", left, right, left.len(), right.len());
        let mut valid_palindromes: HashSet<u64> = HashSet::new();
        for valid_left_multiple in 1..(right.len() as u64 / 2) + 1 {
            // start with 1, fill the rest with 0
            // 1, 10, 100, ...
            for possible_total_digits in left.len()..(right.len() + 1) {
                let possible_total_digits = possible_total_digits as u64;
                // only checking within range
                let mut starting_num: u64 = 10u64.pow(valid_left_multiple as u32 - 1);
                // println!("{}, {}", starting_num, 10u64.pow(valid_left_multiple as u32));
                while construct_palindrome(
                    starting_num,
                    possible_total_digits / valid_left_multiple,
                ) < left_num
                {
                    if starting_num >= 10u64.pow(valid_left_multiple as u32) {
                        break;
                    }
                    starting_num += 1;
                }

                // println!("{}, {}", starting_num, possible_total_digits);
                if starting_num >= 10u64.pow(valid_left_multiple as u32) {
                    continue;
                }
                while construct_palindrome(
                    starting_num,
                    possible_total_digits / valid_left_multiple,
                ) <= right_num
                {
                    if starting_num >= 10u64.pow(valid_left_multiple as u32) {
                        break;
                    }
                    let num = construct_palindrome(
                        starting_num,
                        possible_total_digits / valid_left_multiple,
                    );
                    // println!(
                    //     "{}, {}, {}",
                    //     starting_num, possible_total_digits, valid_left_multiple
                    // );
                    valid_palindromes.insert(num);
                    starting_num += 1;
                }
            }
        }
        // println!("{}, {}, {:?}", left, right, valid_palindromes);
        // sum += valid_palindromes.iter().sum::<u64>();
        sum += valid_palindromes
            .iter()
            .filter(|n| n.to_string().len() > 1)
            .sum::<u64>();
    }

    println!("sum: {}", sum);
    Ok(())
}

fn construct_palindrome(left_half: u64, no_of_times: u64) -> u64 {
    let left_half = left_half.to_string().chars().collect::<Vec<char>>();
    let mut full_palindrome = Vec::new();
    for _ in 0..no_of_times {
        full_palindrome.extend(left_half.clone());
    }
    full_palindrome.iter().collect::<String>().parse().unwrap()
}
