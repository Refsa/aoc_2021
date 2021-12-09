use crate::Runner;
use std::collections::HashSet;
use std::ops::Add;

const CARDINAL_DIRS: [Point; 4] = [Point(1, 0), Point(0, 1), Point(-1, 0), Point(0, -1)];

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Point;

    fn add(self, r: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point(self.0 + r.0, self.1 + r.1)
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

#[derive(Default)]
pub struct AOC9 {
    w: isize,
    h: isize,
    parsed: Vec<Vec<u8>>,
}

impl AOC9 {
    fn valid_pos(&self, pos: Point) -> Option<Index> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.w || pos.1 >= self.h {
            None
        } else {
            Some(pos.into())
        }
    }

    fn get_value(&self, pos: Index) -> u8 {
        self.parsed[pos.1][pos.0]
    }

    fn is_low_point(&self, pos: Point) -> bool {
        let point = self.get_value(pos.into());
        let lower_neighbours = CARDINAL_DIRS
            .iter()
            .filter(|&&dir| {
                let pos = self.valid_pos(pos + dir);
                if pos.is_none() {
                    return false;
                }
                let pos = pos.unwrap();

                let val = self.get_value(pos);
                if val > point {
                    false
                } else {
                    true
                }
            })
            .count();
        lower_neighbours == 0
    }

    fn find_low_points(&self) -> Vec<Index> {
        let mut low_points: Vec<Index> = Vec::new();

        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                let point = Point(x, y);
                if self.is_low_point(point) {
                    low_points.push(point.into());
                }
            }
        }

        low_points
    }

    fn flood_fill(&self, point: Point, visited: &mut HashSet<Point>) -> usize {
        if visited.contains(&point) {
            return 0;
        }

        let pos = self.valid_pos(point);
        if pos.is_none() {
            return 0;
        }
        let pos = pos.unwrap();

        let val = self.get_value(pos);
        if val == 9 {
            return 0;
        }
        visited.insert(point);
        let neighbours = CARDINAL_DIRS
            .iter()
            .map(|&e| self.flood_fill(point + e, visited));
        let neighbours: usize = neighbours.sum();
        1usize + neighbours
    }
}

impl Runner for AOC9 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        let h = input.len();
        let w = input[0].len();
        let mut map = vec![vec![255u8; w]; h];

        for y in 0..h {
            let l = &input[y];
            for (x, c) in l.chars().enumerate() {
                map[y][x] = (c as u8) - 48;
            }
        }

        self.w = w as isize;
        self.h = h as isize;
        self.parsed = map;
    }
    fn run_p1(&self) -> usize {
        let low_points = self.find_low_points();

        low_points
            .iter()
            .map(|e| self.parsed[e.1][e.0] as usize + 1)
            .sum()
    }
    fn run_p2(&self) -> usize {
        let low_points = self.find_low_points();
        let mut visited = HashSet::new();
        let mut basins: Vec<usize> = low_points
            .iter()
            .map(|&e| self.flood_fill(e.into(), &mut visited))
            .collect();

        basins.sort();

        basins
            .iter()
            .skip(basins.len() - 3)
            .take(3)
            .fold(1usize, |acc, e| acc * e)
    }
}
