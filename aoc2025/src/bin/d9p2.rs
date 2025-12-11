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

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Edge {
    pt1: Point,
    pt2: Point,
}

impl Edge {
    fn new(pt1: (usize, usize), pt2: (usize, usize)) -> Edge {
        Edge {
            pt1: Point { x: pt1.0, y: pt1.1 },
            pt2: Point { x: pt2.0, y: pt2.1 },
        }
    }
}

#[derive(Debug)]
struct Polygon {
    edges: Vec<Edge>,
}

// --- Boundary Check Function ---

/// Checks if point p lies exactly on the given axis-aligned edge.
fn is_on_edge(p: &Point, edge: &Edge) -> bool {
    let (p1, p2) = (&edge.pt1, &edge.pt2);

    // Find the min/max x and y coordinates of the edge
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // 1. Check if the point is within the bounding box of the edge
    let within_x = p.x >= min_x && p.x <= max_x;
    let within_y = p.y >= min_y && p.y <= max_y;

    if within_x && within_y {
        // 2. Check for collinearity (since the edge is axis-aligned, this is simple):

        // Vertical Edge: If the edge is vertical (min_x == max_x),
        // the point must have the same x-coordinate as the edge.
        if min_x == max_x {
            return p.x == min_x;
        }

        // Horizontal Edge: If the edge is horizontal (min_y == max_y),
        // the point must have the same y-coordinate as the edge.
        if min_y == max_y {
            return p.y == min_y;
        }
    }

    // Not within the bounding box or not collinear (e.g., a diagonal edge, which we assume is not the case)
    false
}

// --- Main Point-in-Polygon Logic ---

/// Checks if a point is inside or on the boundary of an Axis-Aligned Polygon.
fn pt_in_polygon(pt: &Point, poly: &Polygon) -> bool {
    // 1. **Immediate Boundary Check (Primary change)**
    // If the point is on any edge, it is considered inside.
    if poly.edges.iter().any(|edge| is_on_edge(pt, edge)) {
        return true;
    }

    // 2. **Ray Casting (For points strictly inside or strictly outside)**
    let count = poly
        .edges
        .iter()
        .filter(|edge| ray_intersect_seg(pt, edge))
        .count();

    // An odd number of intersections means the point is inside.
    count % 2 == 1
}

/// Simplified intersection check for Vertical Edges with a horizontal ray cast.
/// Note: This function now only handles points *not* on the boundary,
/// as boundary points are caught by the check above.
fn ray_intersect_seg(p: &Point, edge: &Edge) -> bool {
    // Determine the lower and upper points by y-coordinate.
    let (a, b) = if edge.pt1.y < edge.pt2.y {
        (&edge.pt1, &edge.pt2)
    } else {
        (&edge.pt2, &edge.pt1)
    };

    // 1. Check if the edge is vertical (x-coordinates are identical).
    if a.x == b.x {
        // This is a **Vertical Edge**.

        // Intersection conditions:
        // 2. The point's Y must be within the edge's y-bounds: [min, max)
        // This half-open interval is used to prevent double-counting vertices
        // when two edges meet at a point.
        let intersects_y_range = p.y >= a.y && p.y < b.y;

        // 3. The edge must be strictly to the right of the point (p.x < x_edge).
        // Since we checked for boundary points first, we don't worry about p.x == a.x here.
        let is_to_the_right_of_point = p.x < a.x;

        return intersects_y_range && is_to_the_right_of_point;
    } else {
        // Horizontal edges are ignored by a horizontal ray cast.
        return false;
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
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..cords.len() {
        let l = cords[i];
        let r = cords[(i + 1) % cords.len()];
        edges.push(Edge::new(l, r));
    }
    let polygon: Polygon = Polygon { edges };

    for i in 0..cords.len() {
        for j in (i + 1)..cords.len() {
            let l = cords[i];
            let r = cords[j];
            if [l, r, (l.0, r.1), (r.0, l.1)]
                .iter()
                .all(|p| pt_in_polygon(&Point { x: p.0, y: p.1 }, &polygon))
            {
                sum = sum.max((l.0.abs_diff(r.0) + 1) * (l.1.abs_diff(r.1).max(1) + 1));
            }
        }
    }

    println!("sum: {}", sum);
    Ok(())
}
