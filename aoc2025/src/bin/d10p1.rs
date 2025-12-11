use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::File,
    io::{Read, Result},
    vec,
};

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d10.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;
    for l in input.split("\n") {
        if l.is_empty() {
            break;
        }
        let mut l = l.split("] ");
        let indicator_light = l.next().unwrap();
        let mut others = l.next().unwrap().split("{");
        let indicator_light = indicator_light.chars().collect::<Vec<char>>()[1..]
            .iter()
            .map(|c| if *c == '.' { 0 } else { 1 })
            .collect::<Vec<usize>>();
        let toggles = others.next().unwrap().split(") ").collect::<Vec<_>>();
        let toggles = toggles.clone()[..toggles.len() - 1]
            .iter()
            .map(|s| s[1..].split(",").map(|c| c.parse().unwrap()).collect())
            .collect::<Vec<Vec<usize>>>();
        // prev_state, toggle, no
        let mut q: VecDeque<(Vec<usize>, usize, usize)> = (0..toggles.len())
            .map(|t| (vec![0; indicator_light.len()], t, 1))
            .collect();
        loop {
            let c = q.pop_front().unwrap();
            let mut state = c.0;
            let toggle = c.1;
            for t in &toggles[toggle] {
                state[*t] = if state[*t] == 1 { 0 } else { 1 };
            }
            // check state, first is smallest
            if (0..state.len()).all(|i| state[i] == indicator_light[i]) {
                sum += c.2;
                break;
            }
            // If not bfs
            for t in 0..toggles.len() {
                q.push_back((state.clone(), t, c.2 + 1));
            }
        }
        println!("{:?}", toggles)
    }

    println!("sum: {}", sum);
    Ok(())
}
