use crate::runner::Runner;
use std::ops::Add;

#[derive(Default)]
pub struct AOC11 {
    parsed: Map,
}

const DIRS: [Point; 8] = [
    Point(1, 0),
    Point(0, 1),
    Point(-1, 0),
    Point(0, -1),
    Point(1, 1),
    Point(1, -1),
    Point(-1, -1),
    Point(-1, 1),
];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Point;

    fn add(self, r: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point(self.0 + r.0, self.1 + r.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Debug, Default, Copy, Clone)]
struct Cell {
    value: u8,
    flashed: bool,
}

#[derive(Default, Clone)]
struct Map {
    data: Vec<Cell>,
    w: usize,
    h: usize,
}

impl Map {
    fn _get_cell_mut<'a>(&'a mut self, index: Index) -> &'a mut Cell {
        &mut self.data[index.1 * self.w + index.0]
    }
    fn value_sum(&self) -> usize {
        self.data.iter().map(|e| e.value as usize).sum()
    }
    fn neighbours_pos<'a>(&self, point: Point) -> Box<dyn Iterator<Item = Point> + 'a> {
        let (w, h) = (self.w, self.h);
        Box::new(
            DIRS.iter()
                .map(move |&e| point + e)
                .filter(move |e| in_bounds(e, w, h)),
        )
    }
}

impl Runner for AOC11 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        let cells: Vec<Cell> = input
            .iter()
            .flat_map(|e| {
                e.chars()
                    .map(|c| Cell {
                        value: (c as u8 - 48),
                        ..Default::default()
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect();

        self.parsed = Map {
            h: input.len(),
            w: input[0].len(),
            data: cells,
        };
    }
    fn run_p1(&self) -> usize {
        let mut map = self.parsed.clone();
        let mut sum = 0;

        for _i in 0..100 {
            let val = step(&mut map);

            sum += val;

            /* for l in &map.data {
                println!("{:2?}", l.iter().map(|e| e.value).collect::<Vec<u8>>());
            }
            println!(); */
        }

        sum
    }
    fn run_p2(&self) -> usize {
        let mut map = self.parsed.clone();
        let mut s = 0;

        for i in 0..10_000 {
            s = i + 1;
            let _val = step(&mut map);
            if map.value_sum() == 0 {
                break;
            }
        }

        s
    }
}

fn in_bounds(point: &Point, w: usize, h: usize) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < w as isize && point.1 < h as isize
}

fn step(map: &mut Map) -> usize {
    let mut flashed = Vec::new();
    let mut flashes = 0;

    for i in 0..(map.w * map.h) {
        let cell = &mut map.data[i];

        cell.value += 1;

        cell.flashed = if cell.value > 9 {
            cell.value = 0;
            flashed.push(i as isize);
            true
        } else {
            false
        };
    }

    let w = map.w as isize;
    while flashed.len() != 0 {
        flashes += 1;
        let f = flashed.pop().unwrap();

        let idx = Point(f % w, f / w);
        for n in map.neighbours_pos(idx) {
            let idx = n.1 * w + n.0;
            let cell = &mut map.data[idx as usize];

            if !cell.flashed {
                cell.value += 1;
                cell.flashed = if cell.value > 9 {
                    cell.value = 0;
                    flashed.push(idx);
                    true
                } else {
                    false
                };
            }
        }
    }

    flashes
}
