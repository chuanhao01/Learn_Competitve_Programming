use std::{
    fs::File,
    io::{Read, Result},
};

#[derive(Default, Debug)]
struct Mapper {
    maps: Vec<Map>,
}
impl Mapper {
    fn convert(&self, input: i64) -> i64 {
        for map in &self.maps {
            if let Some(converted_input) = map.convert(input) {
                return converted_input;
            }
        }
        input
    }
}

#[derive(Debug)]
struct Map {
    dest: i64,
    source: i64,
    range: i64,
}
impl Map {
    fn convert(&self, input: i64) -> Option<i64> {
        if self.source <= input && input < self.source + self.range{
            return Some(self.dest + input - self.source )
        }
        None
    }
}

fn generate_mappers(mapper_inputs: &[&str]) -> Vec<Mapper> {
    mapper_inputs
        .iter()
        .map(|mapper_input| -> Mapper {
            let mapper_input = mapper_input.split('\n').collect::<Vec<_>>();
            let maps = mapper_input[1..mapper_input.len()]
                .iter()
                .map(|line_of_numbers| {
                    let numbers = line_of_numbers
                        .split(' ')
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    Map {
                        dest: numbers[0],
                        source: numbers[1],
                        range: numbers[2],
                    }
                })
                .collect::<Vec<_>>();
            Mapper { maps }
        })
        .collect::<Vec<_>>()
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d5")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    let input = input.split("\n\n").collect::<Vec<_>>();
    let seeds = input[0].split(": ").collect::<Vec<_>>()[1]
        .split(' ')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mappers = generate_mappers(&input[1..input.len()]);

    let mut sum = 0;
    sum = seeds
        .iter()
        .map(|seed| {
            mappers
                .iter()
                .fold(*seed, |acc, mapper| mapper.convert(acc))
        })
        .min()
        .unwrap();
    println!("sum: {}", sum);
    Ok(())
}
