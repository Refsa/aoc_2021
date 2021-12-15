use colored::*;
use std::{
    collections::BinaryHeap,
    ops::{Add, Sub},
};

use crate::runner::Runner;

const DIRS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Point;

    fn add(self, r: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point(self.0 + r.0, self.1 + r.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Index(usize, usize);

impl Into<Index> for Point {
    fn into(self) -> Index {
        Index(self.0 as usize, self.1 as usize)
    }
}

impl Into<Point> for Index {
    fn into(self) -> Point {
        Point(self.0 as isize, self.1 as isize)
    }
}

#[derive(Default, Clone)]
struct Map {
    data: Vec<isize>,
    w: usize,
    h: usize,
}

impl Map {
    fn neighbours_pos<'a>(&self, point: Point) -> Box<dyn Iterator<Item = Point> + 'a> {
        let (w, h) = (self.w, self.h);
        Box::new(
            DIRS.iter()
                .map(move |&e| point + e)
                .filter(move |e| in_bounds(e, w, h)),
        )
    }

    fn get_risk(&self, index: &Index) -> isize {
        self.data[index.1 * self.w + index.0]
    }

    fn set(&mut self, index: &Index, val: isize) {
        self.data[index.1 * self.w + index.0] = val;
    }

    fn to_idx(&self, point: Point) -> usize {
        self.w * point.1 as usize + point.0 as usize
    }

    fn grow(&mut self) {
        let w = self.w * 5;
        let h = self.h * 5;
        let mut nmap = vec![vec![-1isize; w]; h];
        let mut ros = Vec::new();
        for y in 0..5usize {
            for x in 0..5usize {
                let risk_offset = ((x % 5) + (y % 5)) as isize;
                ros.push(risk_offset);
                let cp: Vec<Vec<isize>> = self
                    .data
                    .iter()
                    .map(|e| {
                        if e + risk_offset > 9 {
                            (e + risk_offset) % 9
                        } else {
                            e + risk_offset
                        }
                    })
                    .map(|e| e.max(1))
                    .collect::<Vec<isize>>()
                    .chunks(self.w)
                    .into_iter()
                    .map(|e| e.to_vec())
                    .collect();

                for yy in 0..self.h {
                    let idy = y * self.w + yy;
                    for xx in 0..self.w {
                        let idx = x * self.w + xx;
                        let val = cp[yy][xx];
                        nmap[idy][idx] = val;
                    }
                }
            }
        }

        self.w = w;
        self.h = h;
        self.data = nmap.iter().flatten().map(|&e| e).collect();
    }
}

fn in_bounds(point: &Point, w: usize, h: usize) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < w as isize && point.1 < h as isize
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    tot_risk: isize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.tot_risk.cmp(&self.tot_risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct AOC15 {
    map: Map,
}

impl Runner for AOC15 {
    fn parse(&mut self, input: &Vec<String>) {
        let data: Vec<isize> = input
            .iter()
            .map(|e| e.chars().map(|c| c as isize - 48).collect::<Vec<isize>>())
            .flatten()
            .collect();

        self.map = Map {
            w: input[0].len(),
            h: input.len(),
            data: data,
        };
    }

    fn run_p1(&self) -> usize {
        let end = Point(self.map.w as isize - 1, self.map.h as isize - 1);
        let (flowfield, dirs) = generate_flowfield(&self.map, end);

        let mut path = Vec::new();
        let mut curr = Point(0, 0);
        let mut tot_cost = -self.map.get_risk(&curr.into());
        while curr != end {
            path.push(curr);
            tot_cost += self.map.get_risk(&curr.into());
            curr = curr + dirs[flowfield.to_idx(curr)];
        }
        tot_cost += self.map.get_risk(&end.into());

        // draw_flowfield(&flowfield, &dirs, &path);

        tot_cost as usize
    }

    fn run_p2(&self) -> usize {
        let mut map = self.map.clone();
        map.grow();

        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let (flowfield, dirs) = generate_flowfield(&map, end);

        let mut path = Vec::new();
        let mut curr = Point(0, 0);
        let mut tot_cost = -map.get_risk(&curr.into());
        while curr != end {
            path.push(curr);
            tot_cost += map.get_risk(&curr.into());
            curr = curr + dirs[flowfield.to_idx(curr)];
        }
        tot_cost += map.get_risk(&end.into());

        // draw_flowfield(&flowfield, &dirs, &path);

        tot_cost as usize
    }
}

fn generate_flowfield(map: &Map, end: Point) -> (Map, Vec<Point>) {
    let end = Node {
        point: end,
        tot_risk: 0,
    };

    let mut flowfield = Map {
        w: map.w,
        h: map.h,
        data: vec![1 << 32; map.data.len()],
    };

    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    *flowfield.data.last_mut().unwrap() = 0;
    open.push(end);

    while let Some(curr) = open.pop() {
        for n in map.neighbours_pos(curr.point) {
            let n_risk = curr.tot_risk + map.get_risk(&n.into()); // + (curr.point.distance(&n) as isize);

            if n_risk < flowfield.get_risk(&n.into()) {
                open.push(Node {
                    point: n,
                    tot_risk: n_risk,
                });
                flowfield.set(&n.into(), n_risk);
            }
        }
    }

    let mut dirs = Vec::new();
    for y in 0..flowfield.h {
        for x in 0..flowfield.w {
            let point = Point(x as isize, y as isize);
            let dir = flowfield
                .neighbours_pos(point)
                .fold((point, 1 << 32), |acc, e| {
                    let risk = flowfield.get_risk(&e.into());

                    if risk < acc.1 {
                        (e, risk)
                    } else {
                        acc
                    }
                });

            dirs.push(dir.0 - point);
        }
    }

    (flowfield, dirs)
}

fn draw_flowfield(flowfield: &Map, dirs: &Vec<Point>, path: &Vec<Point>) {
    // E, S, W, N
    println!();
    for y in 0..flowfield.h as isize {
        for x in 0..flowfield.w as isize {
            let idx = flowfield.to_idx(Point(x, y));
            let dir = dirs[idx];
            let in_path = path.contains(&Point(x, y));

            let arrow = match DIRS.iter().position(|&v| v == dir).unwrap_or(4) {
                0 => {
                    if in_path {
                        "→".green().on_black()
                    } else {
                        "→".white().on_black()
                    }
                }
                1 => {
                    if in_path {
                        "↓".green().on_black()
                    } else {
                        "↓".white().on_black()
                    }
                }
                2 => {
                    if in_path {
                        "←".green().on_black()
                    } else {
                        "←".white().on_black()
                    }
                }
                3 => {
                    if in_path {
                        "↑".green().on_black()
                    } else {
                        "↑".white().on_black()
                    }
                }
                4 => "X".blue().on_black(),
                _ => unreachable!(),
            };

            print!("{:2}", arrow);
        }
        println!();
    }
    println!();
}
