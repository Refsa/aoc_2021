use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    ops::Sub,
};

use crate::runner::Runner;

#[derive(Default)]
pub struct AOC19 {
    scanners: Vec<Scanner>,
}

fn parse_line(input: &str) -> Point {
    let mut xyz = input.split(',').map(|v| v.parse::<isize>().unwrap());
    Point::new(
        xyz.next().unwrap(),
        xyz.next().unwrap(),
        xyz.next().unwrap(),
    )
}

impl Runner for AOC19 {
    fn parse(&mut self, input: &Vec<String>) {
        let mut scanners = Vec::new();
        let mut beacons = Vec::new();

        for i in 0..input.len() {
            if input[i].starts_with("--- scanner") {
                continue;
            }

            if input[i].is_empty() {
                scanners.push(Scanner {
                    beacons: beacons.clone(),
                });
                beacons.clear();
                continue;
            }

            beacons.push(parse_line(&input[i]));
        }
        if beacons.len() > 0 {
            scanners.push(Scanner { beacons: beacons });
        }

        self.scanners = scanners;
    }

    fn run_p1(&self) -> usize {
        println!();
        /*
        --- scanner 0 ---
        0,2 -> -5, 0 = 5,2
        4,1 -> -1,-1 = 5,2
        3,3 -> -2,-1 = 5,2

        --- scanner 1 ---
        -1,-1 -> 4,1
        -5, 0 -> 0,2
        -2, 1 -> 3,3
        */
        let mut scanners = self.scanners.clone();

        let mut already_found: HashSet<usize> = HashSet::new();
        let mut matches = HashMap::new();

        for i in 0..scanners.len() {
            let a = &scanners[i];
            for j in 0..scanners.len() {
                if i == j || already_found.contains(&j) {
                    continue;
                }
                let b = &scanners[j];

                let mapped = map_scanners(a, b);
                let found = mapped.into_iter().find(|(_k, v)| v.len() >= 12);

                if let Some(found) = found {
                    let flip_idx = found.1[0].flip;
                    println!("{}, {} | {}", i, j, flip_idx);
                    
                    for p in scanners[j].beacons.iter_mut() {
                        p.flip_self(flip_idx);
                    }

                    matches.insert((i, j), found);
                    already_found.insert(i);
                    break;
                }
            }
        }

        let beacons: HashSet<Point> = scanners.into_iter().flat_map(|e| e.beacons).collect();

        beacons.len()
    }

    fn run_p2(&self) -> usize {
        todo!()
    }
}

fn map_scanners(a: &Scanner, b: &Scanner) -> HashMap<Point, Vec<Match>> {
    let mut counters: HashMap<Point, Vec<Match>> = HashMap::new();
    for idx1 in 0..a.beacons.len() {
        let b1 = a.beacons[idx1];

        for idx2 in 0..b.beacons.len() {
            let b2 = b.beacons[idx2];
            for (i, &b2) in b2.get_flips().iter().enumerate() {
                let p = b1 - b2;
                let m = Match::new(idx1, idx2, i);
                let counter = counters
                    .entry(p)
                    .and_modify(|e| e.push(m))
                    .or_insert(vec![m]);

                if counter.len() == 12 {
                    return counters;
                }
            }
        }
    }
    counters
}

#[derive(Debug, Clone, Copy)]
struct Match {
    a: usize,
    b: usize,
    flip: usize,
}

impl Match {
    fn new(a: usize, b: usize, flip: usize) -> Match {
        Match {
            a: a,
            b: b,
            flip: flip,
        }
    }
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{x: {:2}, y: {:2}, z: {:2} }}",
            self.x, self.y, self.z
        ))
    }
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x: x, y: y, z: z }
    }

    fn get_flips(&self) -> [Point; 24] {
        [
            self.flip(0),
            self.flip(1),
            self.flip(2),
            self.flip(3),
            self.flip(4),
            self.flip(5),
            self.flip(6),
            self.flip(7),
            self.flip(8),
            self.flip(9),
            self.flip(10),
            self.flip(11),
            self.flip(12),
            self.flip(13),
            self.flip(14),
            self.flip(15),
            self.flip(16),
            self.flip(17),
            self.flip(18),
            self.flip(19),
            self.flip(20),
            self.flip(21),
            self.flip(22),
            self.flip(23),
        ]
    }

    fn flip_self(&mut self, flip_idx: usize) {
        let flipped = self.flip(flip_idx);
        self.x = flipped.x;
        self.y = flipped.y;
        self.z = flipped.z;
    }

    fn flip(&self, flip_idx: usize) -> Point {
        match flip_idx {
            0 => Point::new(self.x, self.y, self.z),
            1 => Point::new(-self.x, self.y, self.z),
            2 => Point::new(self.x, -self.y, self.z),
            3 => Point::new(self.x, self.y, -self.z),
            4 => Point::new(-self.x, -self.y, self.z),
            5 => Point::new(-self.x, self.y, -self.z),
            6 => Point::new(self.x, -self.y, -self.z),
            7 => Point::new(-self.x, -self.y, -self.z),
            //
            8 => Point::new(self.y, self.x, self.z),
            9 => Point::new(-self.y, self.x, self.z),
            10 => Point::new(self.y, -self.x, self.z),
            11 => Point::new(self.y, self.x, -self.z),
            12 => Point::new(-self.y, -self.x, self.z),
            13 => Point::new(-self.y, self.x, -self.z),
            14 => Point::new(self.y, -self.x, -self.z),
            15 => Point::new(-self.y, -self.x, -self.z),
            //
            16 => Point::new(self.y, self.z, self.x),
            17 => Point::new(-self.y, self.z, self.x),
            18 => Point::new(self.y, -self.z, self.x),
            19 => Point::new(self.y, self.z, -self.x),
            20 => Point::new(-self.y, -self.z, self.x),
            21 => Point::new(-self.y, self.z, -self.x),
            22 => Point::new(self.y, -self.z, -self.x),
            23 => Point::new(-self.y, -self.z, -self.x),
            _ => unreachable!(),
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}
