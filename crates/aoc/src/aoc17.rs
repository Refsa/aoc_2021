use std::{
    collections::HashSet,
    ops::{Add, Div, RangeInclusive, Sub},
};

use crate::runner::Runner;

pub struct AOC17 {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}
impl Default for AOC17 {
    fn default() -> Self {
        Self {
            x_range: (0..=0),
            y_range: (0..=0),
        }
    }
}

impl Runner for AOC17 {
    fn parse(&mut self, input: &Vec<String>) {
        let (_, r) = input[0].split_at(13);
        let (x, y) = r.split_once(",").unwrap();

        let (x1, x2) = &x[2..].split_once("..").unwrap();
        let (y1, y2) = &y[3..].split_once("..").unwrap();

        let x = (x1.parse::<isize>().unwrap(), x2.parse::<isize>().unwrap());
        let y = (y1.parse::<isize>().unwrap(), y2.parse::<isize>().unwrap());

        self.x_range = x.0..=x.1;
        self.y_range = y.0..=y.1;
    }

    fn run_p1(&self) -> usize {
        let target_area: TargetArea = (self.x_range.clone(), self.y_range.clone()).into();

        let (min_x, max_x) = find_vel_range(target_area.min_x, target_area.max_x);

        let mut max_h = 0;
        for x in min_x..=max_x {
            for y in 1..1_000 {
                let vel = Point(x, y);

                if sim(vel, &target_area) {
                    max_h = max_h.max(sum(y));
                }
            }
        }

        max_h as usize
    }

    fn run_p2(&self) -> usize {
        let target_area: TargetArea = (self.x_range.clone(), self.y_range.clone()).into();
        let (min_x, _) = find_vel_range(target_area.min_x, target_area.max_x);

        let mut vels = Vec::new();
        let mut hits = 0;
        for x in min_x..=target_area.max_x {
            for y in target_area.min_y..1_000 {
                let vel = Point(x, y);
                if sim(vel, &target_area) {
                    hits += 1;
                    vels.push(Point(x, y));
                }
            }
        }

        hits as usize
    }
}

fn sum(num: isize) -> isize {
    num * (num + 1) / 2
}

fn find_vel_range(xs: isize, xe: isize) -> (isize, isize) {
    let mut min = isize::MAX;
    let mut max = isize::MIN;

    for i in 0..xe {
        let s = sum(i);
        if s < xs {
            continue;
        } else if s > xe {
            break;
        }
        min = min.min(i);
        max = max.max(i);
    }

    (min, max)
}

fn sim(mut vel: Point, target: &TargetArea) -> bool {
    let mut pos = Point(0, 0);
    loop {
        pos = pos + vel;

        vel.0 = vel.0 + -dir(vel.0);
        vel.1 = vel.1 - 1;
        
        if target.contains(pos) {
            return true;
        }

        if target.passed(pos) {
            break;
        }
    }

    false
}

fn dir(num: isize) -> isize {
    if num == 0 {
        0
    } else if num < 0 {
        -1
    } else {
        1
    }
}

#[derive(Debug)]
struct TargetArea {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl From<(RangeInclusive<isize>, RangeInclusive<isize>)> for TargetArea {
    fn from(ranges: (RangeInclusive<isize>, RangeInclusive<isize>)) -> Self {
        let mut mmx = Point(isize::MAX, isize::MIN);
        let mut mmy = Point(isize::MAX, isize::MIN);

        for x in ranges.0 {
            for y in ranges.1.clone() {
                mmx.0 = mmx.0.min(x);
                mmx.1 = mmx.1.max(x);
                mmy.0 = mmy.0.min(y);
                mmy.1 = mmy.1.max(y);
            }
        }

        Self {
            min_x: mmx.0,
            min_y: mmy.0,
            max_x: mmx.1,
            max_y: mmy.1,
        }
    }
}

impl TargetArea {
    fn contains(&self, point: Point) -> bool {
        point.0 >= self.min_x
            && point.0 <= self.max_x
            && point.1 >= self.min_y
            && point.1 <= self.max_y
    }

    fn passed(&self, point: Point) -> bool {
        point.0 > self.max_x || point.1 < self.min_y
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
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

impl Point {
    fn div(&self, by: isize) -> Self {
        Self(self.0 / by, self.1 / by)
    }
}

fn wanted() -> HashSet<Point> {
    [
        Point(23, -10),
        Point(25, -9),
        Point(27, -5),
        Point(29, -6),
        Point(22, -6),
        Point(21, -7),
        Point(9, 0),
        Point(27, -7),
        Point(24, -5),
        Point(25, -7),
        Point(26, -6),
        Point(25, -5),
        Point(6, 8),
        Point(11, -2),
        Point(20, -5),
        Point(29, -10),
        Point(6, 3),
        Point(28, -7),
        Point(8, 0),
        Point(30, -6),
        Point(29, -8),
        Point(20, -10),
        Point(6, 7),
        Point(6, 4),
        Point(6, 1),
        Point(14, -4),
        Point(21, -6),
        Point(26, -10),
        Point(7, -1),
        Point(7, 7),
        Point(8, -1),
        Point(21, -9),
        Point(6, 2),
        Point(20, -7),
        Point(30, -10),
        Point(14, -3),
        Point(20, -8),
        Point(13, -2),
        Point(7, 3),
        Point(28, -8),
        Point(29, -9),
        Point(15, -3),
        Point(22, -5),
        Point(26, -8),
        Point(25, -8),
        Point(25, -6),
        Point(15, -4),
        Point(9, -2),
        Point(15, -2),
        Point(12, -2),
        Point(28, -9),
        Point(12, -3),
        Point(24, -6),
        Point(23, -7),
        Point(25, -10),
        Point(7, 8),
        Point(11, -3),
        Point(26, -7),
        Point(7, 1),
        Point(23, -9),
        Point(6, 0),
        Point(22, -10),
        Point(27, -6),
        Point(8, 1),
        Point(22, -8),
        Point(13, -4),
        Point(7, 6),
        Point(28, -6),
        Point(11, -4),
        Point(12, -4),
        Point(26, -9),
        Point(7, 4),
        Point(24, -10),
        Point(23, -8),
        Point(30, -8),
        Point(7, 0),
        Point(9, -1),
        Point(10, -1),
        Point(26, -5),
        Point(22, -9),
        Point(6, 5),
        Point(7, 5),
        Point(23, -6),
        Point(28, -10),
        Point(10, -2),
        Point(11, -1),
        Point(20, -9),
        Point(14, -2),
        Point(29, -7),
        Point(13, -3),
        Point(23, -5),
        Point(24, -8),
        Point(27, -9),
        Point(30, -7),
        Point(28, -5),
        Point(21, -10),
        Point(7, 9),
        Point(6, 6),
        Point(21, -5),
        Point(27, -10),
        Point(7, 2),
        Point(30, -9),
        Point(21, -8),
        Point(22, -7),
        Point(24, -9),
        Point(20, -6),
        Point(6, 9),
        Point(29, -5),
        Point(8, -2),
        Point(27, -8),
        Point(30, -5),
        Point(24, -7),
    ].into()
}