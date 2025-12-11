use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{Read, Result},
    vec,
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d11.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.split("\n") {
        if l.is_empty() {
            break;
        }
        let mut l = l.split(": ");
        let k = l.next().unwrap().to_string();
        let v = l
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        hm.insert(k, v);
    }
    println!("{:?}", hm);
    let mut q: VecDeque<String> = VecDeque::new();
    for v in &hm["you"] {
        q.push_back(v.clone());
    }
    while !q.is_empty() {
        println!("{:?}", q.len());
        let k = q.pop_front().unwrap();
        if k == "out" {
            sum += 1;
            continue;
        }
        if !hm.contains_key(&k) {
            continue;
        }
        for v in &hm[&k] {
            q.push_back(v.clone());
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
