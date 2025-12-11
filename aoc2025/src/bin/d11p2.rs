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
        // hm.insert(k, v);
        for vv in v {
            hm.entry(vv)
                .and_modify(|e| e.push(k.clone()))
                .or_insert(vec![k.clone()]);
        }
    }
    println!("{:?}", hm);
    // let mut q: VecDeque<(String, bool, bool, HashSet<String>)> = VecDeque::new();
    let mut q: VecDeque<(String, bool, bool)> = VecDeque::new();
    for v in &hm["out"] {
        // q.push_back((v.clone(), false, false, HashSet::from(["svr".to_string()])));
        q.push_back((v.clone(), false, false));
    }
    // println!("{:?}", q);
    while !q.is_empty() {
        println!("{:?}", q.len());
        let mut entry = q.pop_front().unwrap();
        println!("{:?}", entry);
        let k = entry.0;
        if k == "svr" {
            if entry.1 && entry.2 {
                sum += 1;
            }
            continue;
        }
        if !hm.contains_key(&k) {
            continue;
        }
        if k == "dac" {
            entry.1 = true;
        }
        if k == "fft" {
            entry.2 = true;
        }
        for v in &hm[&k] {
            // let mut new_seen = entry.3.clone();
            // new_seen.insert(k.clone());
            // if !entry.3.contains(v) {
            //     q.push_back((v.clone(), entry.1, entry.2, new_seen.clone()));
            // }

            q.push_back((v.clone(), entry.1, entry.2));
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
