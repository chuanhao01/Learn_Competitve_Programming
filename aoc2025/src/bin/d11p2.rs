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

    let mut i = 0;
    let mut idx: HashMap<String, usize> = HashMap::new();
    let mut f: Vec<String> = Vec::new();
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
        if !idx.contains_key(&k) {
            idx.insert(k.clone(), i);
            i += 1;
        }
        for vv in &v {
            if !idx.contains_key(vv) {
                idx.insert(vv.clone(), i);
                i += 1;
            }
            f.push(format!("{},{}", idx[&k], idx[vv]));
        }
        hm.insert(k, v);
    }
    // println!("{:?}", idx);
    // println!("{:?}", hm);
    // println!("{}", f.join("\n"));

    // let b = dfs("svr".to_string(), "out".to_string(), &hm);
    // let b = dfs("svr".to_string(), "fft".to_string(), &hm);
    // let c = dfs("fft".to_string(), "dac".to_string(), &hm);
    // let a = dfs("dac".to_string(), "out".to_string(), &hm);

    let b = dfs("you".to_string(), "out".to_string(), &hm);

    println!("{}", b);

    // println!("sum: {}", a * b * c);
    Ok(())
}

fn dfs(start: String, end: String, hm: &HashMap<String, Vec<String>>) -> usize {
    let mut dp: HashSet<(String, String)> = HashSet::new();
    let mut off: HashSet<(String, String)> = HashSet::new();
    let mut q: VecDeque<(String, Vec<(String, String)>)> = VecDeque::new();
    for v in &hm[&start] {
        q.push_back((v.clone(), Vec::new()));
    }
    let mut sum = 0;
    while !q.is_empty() {
        println!("{:?}", q.len());
        let entry = q.pop_back().unwrap();
        // println!("{:?}", entry);
        let k = entry.0;
        if k == end {
            for e in entry.1 {
                dp.insert(e.clone());
            }
            sum += 1;
            continue;
        }
        if !hm.contains_key(&k) {
            for e in entry.1 {
                off.insert(e.clone());
            }
            continue;
        }
        for v in &hm[&k] {
            if dp.contains(&(k.clone(), v.clone())) {
                for e in &entry.1 {
                    dp.insert(e.clone());
                }
                sum += 1;
                continue;
            }
            if off.contains(&(k.clone(), v.clone())) {
                for e in &entry.1 {
                    off.insert(e.clone());
                }
                continue;
            }

            let mut n = entry.1.clone();
            n.push((k.clone(), v.clone()));
            q.push_back((v.clone(), n));
        }
    }
    sum
}
