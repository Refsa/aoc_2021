use crate::Runner;
use std::collections::HashSet;

#[derive(Default)]
pub struct AOC9 {
    w: isize,
    h: isize,
    parsed: Vec<Vec<u8>>,
}

impl AOC9 {
    fn valid_pos(&self, pos: (isize, isize)) -> Option<(usize, usize)> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.w || pos.1 >= self.h {
            None
        } else {
            Some((pos.0 as usize, pos.1 as usize))
        }
    }

    fn get_value(&self, pos: (usize, usize)) -> u8 {
        self.parsed[pos.1][pos.0]
    }

    fn is_low_point(&self, pos: (isize, isize)) -> bool {
        let point = self.get_value((pos.0 as usize, pos.1 as usize));
        CARDINAL_DIRS
            .iter()
            .filter(|dir| {
                let pos = (pos.0 + dir.0, pos.1 + dir.1);

                if let Some(pos) = self.valid_pos(pos) {
                    let val = self.get_value(pos);
                    if val > point {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .count()
            == 0
    }

    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = Vec::new();

        for y in 0..self.h as isize {
            for x in 0..self.w as isize {
                if self.is_low_point((x, y)) {
                    low_points.push((x as usize, y as usize));
                }
            }
        }

        low_points
    }

    fn flood_fill(&self, point: (isize, isize), visited: &mut HashSet<(isize, isize)>) -> usize {
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
            .map(|e| self.flood_fill((e.0 + point.0, e.1 + point.1), visited));
        let neighbours: usize = neighbours.sum();
        1usize + neighbours
    }
}

const CARDINAL_DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Runner for AOC9 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        let h = input.len();
        let w = input[0].len();
        let mut map = vec![vec![0u8; w]; h];

        for y in 0..h {
            let l = &input[y];
            for (x, c) in l.chars().enumerate() {
                map[y][x] = (c.to_string()).parse::<u8>().unwrap();
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
        let mut visited = HashSet::new();
        let low_points = self.find_low_points();
        let mut basins: Vec<usize> = low_points
            .iter()
            .map(|e| self.flood_fill((e.0 as isize, e.1 as isize), &mut visited))
            .collect();
        basins.sort();
        basins
            .iter()
            .skip(basins.len() - 3)
            .take(3)
            .fold(1usize, |acc, e| acc * e)
    }
}
