use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{Read, Result},
    time::Instant,
};

#[derive(Debug)]
enum Line {
    Horizontal { x1: usize, x2: usize, y: usize },
    Vertical { x: usize, y1: usize, y2: usize },
}
impl Line {
    fn new(p1: (usize, usize), p2: (usize, usize)) -> Self {
        if p1.1 == p2.1 {
            Self::Horizontal {
                x1: p1.0.min(p2.0),
                x2: p1.0.max(p2.0),
                y: p2.1,
            }
        } else {
            Self::Vertical {
                x: p1.0,
                y1: p1.1.min(p2.1),
                y2: p1.1.max(p2.1),
            }
        }
    }
    fn check_intersection(&self, p: (usize, usize)) -> bool {
        match self {
            Self::Horizontal { x1, x2, y } => {
                // if *y != p.1 {
                //     false
                // } else {
                //     *x1 <= p.0 && p.0 <= *x2
                // }
                false
            }
            Self::Vertical { x, y1, y2 } => {
                if *y1 <= p.1 && p.1 <= *y2 {
                    p.0 < *x
                } else {
                    false
                }
            }
        }
    }
    fn lies_on_line(&self, p: (usize, usize)) -> bool {
        match self {
            Self::Horizontal { x1, x2, y } => {
                if *y != p.1 {
                    false
                } else {
                    *x1 <= p.0 && p.0 <= *x2
                }
            }
            Self::Vertical { x, y1, y2 } => {
                if *x != p.0 {
                    false
                } else {
                    *y1 <= p.1 && p.1 <= *y2
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d9.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;

    let cords: Vec<(usize, usize)> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut l = l.split(",");
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let points: HashSet<(usize, usize)> = cords.clone().into_iter().collect();
    let mut lines: Vec<Line> = Vec::new();
    for i in 0..cords.len() {
        let l = cords[i];
        let r = cords[(i + 1) % cords.len()];
        lines.push(Line::new(l, r));
    }
    println!("{:?}", lines);

    for i in 0..cords.len() {
        for j in (i + 1)..cords.len() {
            let l = cords[i];
            let r = cords[j];
            // println!(
            //     "{:?}, {:?}: {:?} {:?}",
            //     l,
            //     r,
            //     [(l.0, r.1), (r.0, l.1)]
            //         .iter()
            //         .map(|p| {
            //             lines
            //                 .iter()
            //                 .map(|l| l.check_intersection(*p))
            //                 .filter(|intersection| *intersection)
            //                 .count()
            //         })
            //         .collect::<Vec<_>>(),
            //     [(l.0, r.1), (r.0, l.1)]
            //         .iter()
            //         .map(|p| {
            //             if points.contains(p) {
            //                 true
            //             } else {
            //                 false
            //             }
            //         })
            //         .collect::<Vec<_>>()
            // );
            if [(l.0, r.1), (r.0, l.1)].iter().all(|p| {
                if points.contains(p) || lines.iter().any(|l| l.lies_on_line(*p)) {
                    true
                } else {
                    lines
                        .iter()
                        .map(|l| l.check_intersection(*p))
                        .filter(|intersection| *intersection)
                        .count()
                        % 2
                        != 0
                }
            }) {
                sum = sum.max((l.0.abs_diff(r.0) + 1) * (l.1.abs_diff(r.1).max(1) + 1));
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
