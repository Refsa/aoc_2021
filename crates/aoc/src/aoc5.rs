use crate::runner::Runner;

#[derive(Default)]
pub struct AOC5 {
    parsed: Map,
}

impl AOC5 {
    fn parse_point(p: &str) -> Point {
        let (s, e) = p.split_once(",").unwrap();
        Point {
            x: s.parse::<isize>().unwrap(),
            y: e.parse::<isize>().unwrap(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}
impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x: x, y: y }
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Default, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_cardinal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn get_points_cardinal(&self, container: &mut Vec<Point>) {
        if self.is_horizontal() {
            let (s, e) = (self.p1.x.min(self.p2.x), self.p1.x.max(self.p2.x));
            for p in (s..=e).map(|e| Point { x: e, y: self.p1.y }) {
                container.push(p);
            }
        } else if self.is_vertical() {
            let (s, e) = (self.p1.y.min(self.p2.y), self.p1.y.max(self.p2.y));
            for p in (s..=e).map(|e| Point { x: self.p1.x, y: e }) {
                container.push(p);
            }
        }
    }

    fn get_points_diag(&self, container: &mut Vec<Point>) {
        let dx = (self.p2.x - self.p1.x).abs();
        let sx = if self.p1.x < self.p2.x { 1 } else { -1 };
        let dy = -(self.p2.y - self.p1.y).abs();
        let sy = if self.p1.y < self.p2.y { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x0 = self.p1.x;
        let mut y0 = self.p1.y;

        loop {
            container.push(Point::new(x0, y0));
            if x0 == self.p2.x || y0 == self.p2.y {
                break;
            }

            let e2 = err * 2;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn get_points(&self, container: &mut Vec<Point>) {
        if self.is_horizontal() || self.is_vertical() {
            return self.get_points_cardinal(container);
        } else {
            return self.get_points_diag(container);
        }
    }
}

#[derive(Default, Debug)]
struct Map {
    vents: Vec<Line>,
    size: (usize, usize),
}

impl Map {
    fn to_1d(&self, point: &Point) -> usize {
        (point.x + self.size.0 as isize * point.y) as usize
    }
}

impl Runner for AOC5 {
    fn parse(&mut self, input: &Vec<String>) {
        let mut w = 0usize;
        let mut h = 0usize;
        let mut vents = Vec::new();

        for l in input.iter() {
            let (p1, p2) = l.split_once(" -> ").unwrap();
            let p1 = Self::parse_point(p1);
            let p2 = Self::parse_point(p2);

            w = w.max(p1.x.max(p2.x) as usize);
            h = h.max(p1.y.max(p2.y) as usize);

            vents.push(Line { p1: p1, p2: p2 });
        }

        self.parsed = Map {
            vents: vents,
            size: (w + 1, h + 1),
        };
    }

    fn run_p1(&self) -> usize {
        let map = &self.parsed;

        let mut overlaps = vec![0u8; map.size.0 * map.size.1];
        let mut points = Vec::new();

        map.vents.iter().filter(|e| e.is_cardinal()).fold(0usize, |mut acc, vent| {
            points.clear();
            vent.get_points_cardinal(&mut points);

            for p in &points {
                overlaps[map.to_1d(p)] += 1;
                if overlaps[map.to_1d(p)] == 2 {
                    acc += 1;
                }
            }
            acc
        })
    }
    fn run_p2(&self) -> usize {
        let map = &self.parsed;

        let mut overlaps = vec![0u8; map.size.0 * map.size.1];
        let mut points = Vec::new();

        map.vents.iter().fold(0usize, |mut acc, vent| {
            points.clear();
            vent.get_points(&mut points);

            for p in &points {
                overlaps[map.to_1d(p)] += 1;
                if overlaps[map.to_1d(p)] == 2 {
                    acc += 1;
                }
            }
            acc
        })
    }
}

mod tests {
    #[test]
    fn map_2d_to_1d() {
        let map = super::Map {
            vents: Vec::new(),
            size: (10, 10),
        };

        let p = super::Point { x: 5, y: 5 };
        assert_eq!(55, map.to_1d(&p));

        let p = super::Point { x: 5, y: 4 };
        assert_eq!(45, map.to_1d(&p));

        let p = super::Point { x: 0, y: 3 };
        assert_eq!(30, map.to_1d(&p));
    }

    /* #[test]
    fn bresenham_points() {
        use super::*;

        let l = Line {
            p1: Point::new(3, 4),
            p2: Point::new(1, 4),
        };
        let mut points = Vec::new();
        l.get_points(&mut points);
        let expected = vec![Point::new(3, 4), Point::new(2, 4), Point::new(1, 4)];

        assert_eq!(expected, points);
    } */

    #[test]
    fn bresenham_points_rev() {
        use super::*;

        let l = Line {
            p1: Point::new(3, 3),
            p2: Point::new(0, 0),
        };
        let mut points = Vec::new();
        l.get_points(&mut points);
        assert_eq!(
            vec![
                Point::new(3, 3),
                Point::new(2, 2),
                Point::new(1, 1),
                Point::new(0, 0),
            ],
            points
        );
    }
}
