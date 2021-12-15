use colored::*;
use std::{
    collections::BinaryHeap,
    ops::{Add, Sub},
};

use crate::runner::Runner;

const DIRS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(pub isize, pub isize);

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
pub struct Map {
    data: Vec<isize>,
    pub w: usize,
    pub h: usize,
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

    fn to_point(&self, idx: usize) -> Point {
        Point((idx / self.w) as isize, (idx % self.w) as isize)
    }

    pub fn grow(&mut self) {
        let mut grow_lookup = vec![self.data.clone()];
        grow_lookup.extend((1..9).map(|i| {
            self.data
                .iter()
                .map(|e| (e + i - 1) % 9 + 1)
                .collect::<Vec<isize>>()
        }));

        let w = self.w * 5;
        let h = self.h * 5;
        let mut nmap = vec![-1isize; w * h];

        for y in 0..5usize {
            for x in 0..5usize {
                let risk_offset = ((x % 5) + (y % 5)) as isize;
                let cp = &grow_lookup[risk_offset as usize];

                for yy in 0..self.h {
                    let idy = y * self.w + yy;
                    for xx in 0..self.w {
                        let idx = x * self.w + xx;
                        let val = cp[yy * self.w + xx];

                        nmap[idy * w + idx] = val;
                    }
                }
            }
        }

        self.w = w;
        self.h = h;
        self.data = nmap;
    }

    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.w as isize && point.1 < self.h as isize
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
    pub map: Map,
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
        let flowfield = generate_flowfield(&self.map, end);
        let mut dirs = find_dirs(&flowfield);
        dirs[flowfield.to_idx(end)] = Point(0, 0);

        let (_path, tot_cost) = find_path(&flowfield, &self.map, &dirs, Point(0, 0), end);

        // draw_flowfield(&flowfield, &dirs, &path);
        // plot::_plot_flowfield("./assets/flowfield-p1.png", &flowfield, &dirs, &_path);

        tot_cost
    }

    fn run_p2(&self) -> usize {
        let mut map = self.map.clone();
        map.grow();

        let end = Point(map.w as isize - 1, map.h as isize - 1);
        let flowfield = generate_flowfield(&map, end);
        let mut dirs = find_dirs(&flowfield);
        dirs[flowfield.to_idx(end)] = Point(0, 0);

        let (_path, tot_cost) = find_path(&flowfield, &map, &dirs, Point(0, 0), end);

        // draw_flowfield(&flowfield, &dirs, &path);
        // plot::_plot_flowfield("./assets/flowfield-p2.png", &flowfield, &dirs, &_path);

        tot_cost
    }
}

pub fn find_path(
    flowfield: &Map,
    map: &Map,
    dirs: &Vec<Point>,
    start: Point,
    end: Point,
) -> (Vec<Point>, usize) {
    let mut path = Vec::new();

    if !flowfield.in_bounds(&start) || !flowfield.in_bounds(&end) {
        return (path, 0);
    }

    let mut curr = start;
    let mut tot_cost = -map.get_risk(&curr.into());
    while curr != end {
        path.push(curr);
        tot_cost += map.get_risk(&curr.into());
        curr = curr + dirs[flowfield.to_idx(curr)];
    }
    tot_cost += map.get_risk(&end.into());

    (path, tot_cost as usize)
}

pub fn find_dirs(flowfield: &Map) -> Vec<Point> {
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
    dirs
}

pub fn generate_flowfield(map: &Map, end: Point) -> Map {
    if !map.in_bounds(&end) {
        panic!("out of bounds, but probably shouldnt panic");
    }

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

    flowfield
}

fn _draw_flowfield(flowfield: &Map, dirs: &Vec<Point>, path: &Vec<Point>) {
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

mod plot {
    use super::*;
    use image::imageops::FilterType;
    use image::{GenericImageView, ImageFormat};
    use plotters::coord::Shift;
    use plotters::prelude::*;
    use std::fs::File;
    use std::io::BufReader;

    fn _load_image<'a>(path: &str) -> Vec<Vec<RGBColor>> {
        let image = image::load(
            BufReader::new(
                File::open(path).map_err(|e| {
                    eprintln!("Unable to open file plotters-doc-data.png, please make sure you have clone this repo with --recursive");
                    e
                }).unwrap()),
            ImageFormat::Png,
        ).unwrap()
        .resize_exact(8, 8, FilterType::Nearest);

        let mut colors = vec![vec![BLACK; 8]; 8];
        for x in 0..8 {
            for y in 0..8 {
                let pixel = image.get_pixel(x, y).0;
                colors[y as usize][x as usize] = RGBColor(pixel[0], pixel[1], pixel[2]);
            }
        }

        colors
    }

    fn _draw_image(image: &Vec<Vec<RGBColor>>, area: &mut DrawingArea<BitMapBackend, Shift>) {
        // let (w, h) = area.dim_in_pixel();
        let (w, h) = (8, 8);
        for x in 0..w as usize {
            for y in 0..h as usize {
                area.draw_pixel((x as i32, y as i32), &image[y][x]).unwrap();
            }
        }
    }

    pub fn _plot_flowfield(output: &str, flowfield: &Map, dirs: &Vec<Point>, path: &Vec<Point>) {
        let arrows: Vec<Vec<Vec<RGBColor>>> = [
            "./assets/arrow_right.png",
            "./assets/arrow_down.png",
            "./assets/arrow_left.png",
            "./assets/arrow_up.png",
            "./assets/path_right.png",
            "./assets/path_down.png",
            "./assets/path_left.png",
            "./assets/path_up.png",
        ]
        .into_iter()
        .map(|e| _load_image(e))
        .collect();

        let root_drawing_area =
            BitMapBackend::new(output, (flowfield.w as u32 * 8, flowfield.h as u32 * 8))
                .into_drawing_area();
        root_drawing_area.fill(&WHITE).unwrap();

        let mut child_drawing_areas = root_drawing_area.split_evenly((flowfield.w, flowfield.h));

        for y in 0..flowfield.h as isize {
            for x in 0..flowfield.w as isize {
                let idx = flowfield.to_idx(Point(x, y));
                let dir = dirs[idx];
                let in_path = path.contains(&Point(x, y));

                let arrow = match DIRS.iter().position(|&v| v == dir).unwrap_or(4) {
                    0 if in_path => Some(0 + 4),
                    1 if in_path => Some(1 + 4),
                    2 if in_path => Some(2 + 4),
                    3 if in_path => Some(3 + 4),
                    0 => Some(0),
                    1 => Some(1),
                    2 => Some(2),
                    3 => Some(3),
                    4 => None,
                    _ => unreachable!(),
                };

                if let Some(arrow) = arrow {
                    let mut area = child_drawing_areas.get_mut(idx).unwrap();
                    _draw_image(&arrows[arrow], &mut area);
                }
            }
        }

        root_drawing_area.present().expect("failed to create plot");
    }
}
