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
    for (i, l) in input.split("\n").enumerate() {
        println!("{}", i);
        if l.is_empty() {
            break;
        }
        let mut l = l.split("] ");
        let indicator_light = l.next().unwrap();
        let mut others = l.next().unwrap().split("{");
        let toggles = others.next().unwrap().split(") ").collect::<Vec<_>>();
        let toggles = toggles.clone()[..toggles.len() - 1]
            .iter()
            .map(|s| s[1..].split(",").map(|c| c.parse().unwrap()).collect())
            .collect::<Vec<Vec<usize>>>();
        let mut indicator_light = others.next().unwrap().to_string();
        indicator_light.pop();
        let indicator_light = indicator_light
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<usize>>();

        // // prev_state, toggle, no
        let mut q: VecDeque<(Vec<usize>, usize, usize)> = (0..toggles.len())
            .map(|t| (vec![0; indicator_light.len()], t, 1))
            .collect();
        loop {
            let c = q.pop_front().unwrap();
            let mut state = c.0;
            // println!("{:?}, {}", state, c.2);
            let toggle = c.1;
            for t in &toggles[toggle] {
                state[*t] += 1;
            }
            // check state, first is smallest
            if (0..state.len()).all(|i| state[i] == indicator_light[i]) {
                sum += c.2;
                break;
            } else if (0..state.len()).all(|i| state[i] <= indicator_light[i]) {
                // Only bfs if we are below
                for t in 0..toggles.len() {
                    q.push_back((state.clone(), t, c.2 + 1));
                }
            }
            // If not bfs
        }
        // println!("{:?}", indicator_light);
    }

    println!("sum: {}", sum);
    Ok(())
}
