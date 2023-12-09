use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{Read, Result},
};

use itertools::izip;

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d8")?;
    // let mut file_input = File::open("inputs/temp")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;
    let mut sum = 0;

    let input = input.split("\n\n").collect::<Vec<_>>();
    let seq = input[0].chars().map(|c| c.to_string()).collect::<Vec<_>>();
    // println!("{:?}", seq);
    let raw_mappings = input[1].split('\n');
    let mut map: HashMap<String, Next> = HashMap::new();
    let mut current: Vec<String> =  Vec::new();
    for raw_mapping in raw_mappings {
        let mapping = raw_mapping.split(" = ").collect::<Vec<_>>();
        let from = mapping[0];
        if from.ends_with('A'){
            current.push(from.to_string());
        }
        let raw_locations = mapping[1].replace(['(', ')'], "");
        let locations = raw_locations.split(", ").collect::<Vec<_>>();
        // println!("from: {}, locations: {:?}", from, locations);
        map.insert(
            from.to_string(),
            Next {
                left: locations[0].to_string(),
                right: locations[1].to_string(),
            },
        );
    }
    // println!("{:?}, {}", current, current.iter().all(|location| location.ends_with('Z')));
    let mut index = 0;
    while !current.iter().all(|location| location.ends_with('Z')){
        current = current.iter().map(|location|{
            let direction = &seq[index];
            let mapping = &map[location];
            if direction == "L" {
                mapping.left.clone()
            } else {
                mapping.right.clone()
            }
        }).collect::<Vec<_>>();
        // println!("{:?}, index: {}", current, index);
        index = if index < seq.len() - 1 { index + 1 } else { 0 };
        sum += 1;
    }

    println!("sum: {}", sum);
    Ok(())
}

// Write test for get type for cards
// Check for each type test case
#[cfg(test)]
mod tests {
    use super::*;
}
