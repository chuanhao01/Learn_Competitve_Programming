use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{Read, Result},
    vec,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(nums: Vec<usize>) -> Self {
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct UFS {
    parents: Vec<usize>,
}
impl UFS {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
        }
    }
    fn find(&self, i: usize) -> usize {
        if self.parents[i] != i {
            self.find(self.parents[i])
        } else {
            i
        }
    }
    fn union(&mut self, i: usize, j: usize) {
        let l = self.find(i);
        let r = self.find(j);
        self.parents[l] = r;
    }
}

fn main() -> Result<()> {
    let mut file_input = File::open("inputs/d8.txt")?;
    // let mut file_input = File::open("inputs/input")?;
    let mut input = String::new();
    file_input.read_to_string(&mut input)?;

    // Get the ranges
    let mut sum = 0;

    let mut points: Vec<Point> = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            break;
        }
        let nums = l
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        points.push(Point::new(nums));
    }
    // println!("{:?}", points);

    // let mut points_graph: HashMap<Point, Vec<Point>> =
    //     points.iter().map(|p| (p.clone(), Vec::new())).collect();

    let mut dists: Vec<(usize, usize, usize)> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            dists.push((i, j, p1.dist(p2)));
        }
    }
    dists.sort_by(|a, b| a.2.cmp(&b.2));
    println!("{:?}", dists.len());

    // Removing dupes
    let mut new_dists: Vec<(usize, usize, usize)> = Vec::new();
    let mut i = 0;
    while i < dists.len() {
        if i == points.len() - 1 {
            new_dists.push(dists[i]);
            i += 1
        } else if dists[i].2 == dists[i + 1].2 {
            new_dists.push(dists[i]);
            i += 2;
        } else {
            new_dists.push(dists[i]);
            i += 1
        }
    }
    dists = new_dists;

    // println!("{:?}", dists);
    println!("{:?}", dists.len());
    // println!("{:?}", dists[0..10].iter().collect::<Vec<_>>());

    // Union find
    let mut ufs = UFS::new(points.len());
    for dist in dists[0..1000].iter() {
        ufs.union(dist.0, dist.1);
    }
    let mut size: Vec<usize> = vec![0; points.len()];
    for i in 0..points.len() {
        size[ufs.find(i)] += 1;
    }
    size.sort_by(|a, b| b.cmp(a));
    println!("{:?}", size);

    println!("sum: {}", size[0] * size[1] * size[2]);
    Ok(())
}
