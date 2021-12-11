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
    visited: bool,
}

#[derive(Default, Clone)]
struct Map {
    data: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
}

impl Map {
    fn get_cell_mut<'a>(&'a mut self, index: Index) -> &'a mut Cell {
        &mut self.data[index.1][index.0]
    }
    fn value_sum(&self) -> usize {
        self.data.iter().fold(0usize, |acc1, c| {
            acc1 + c.iter().fold(0usize, |acc2, r| acc2 + r.value as usize)
        })
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
        let cells: Vec<Vec<Cell>> = input
            .iter()
            .map(|e| {
                e.chars()
                    .map(|c| Cell {
                        value: (c as u8 - 48),
                        ..Default::default()
                    })
                    .collect()
            })
            .collect();

        self.parsed = Map {
            h: cells.len(),
            w: cells[0].len(),
            data: cells,
        };
    }
    fn run_p1(&self) -> usize {
        let mut map = self.parsed.clone();
        let mut sum = 0;

        for _i in 0..100 {
            let val = step(&mut map);

            sum += val;
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
    for y in 0..map.h {
        for x in 0..map.w {
            let mut cell = map.get_cell_mut(Index(x, y));

            cell.value += 1;
            cell.flashed = false;
            cell.visited = false;
        }
    }

    let mut flashes = 0;

    let mut open: Vec<Index> = Vec::new();

    open.push(Index(0, 0));
    while open.len() > 0 {
        let idx = open.pop().unwrap();
        let mut val = map.get_cell_mut(idx);
        if val.visited {
            continue;
        }

        val.visited = true;

        if val.value > 9 && !val.flashed {
            val.value = 0;
            val.flashed = true;
            flashes += 1;
        }

        if val.flashed {
            for n_pos in map.neighbours_pos(idx.into()) {
                let mut n = map.get_cell_mut(n_pos.into());

                if !n.flashed {
                    n.value += 1;
                }

                if n.value > 9 && !n.flashed {
                    n.visited = false;
                    open.push(n_pos.into());
                }
            }
        }

        for n_pos in map.neighbours_pos(idx.into()) {
            open.push(n_pos.into());
        }
    }

    flashes
}
