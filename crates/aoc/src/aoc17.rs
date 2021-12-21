use std::ops::{Add, Sub};

use crate::runner::Runner;

#[derive(Default)]
pub struct AOC17 {
    x_range: (isize, isize),
    y_range: (isize, isize),
}

impl Runner for AOC17 {
    fn parse(&mut self, input: &Vec<String>) {
        let (_, r) = input[0].split_at(13);
        let (x, y) = r.split_once(",").unwrap();

        let (x1, x2) = &x[2..].split_once("..").unwrap();
        let (y1, y2) = &y[3..].split_once("..").unwrap();

        self.x_range = (x1.parse::<isize>().unwrap(), x2.parse::<isize>().unwrap());
        self.y_range = (y1.parse::<isize>().unwrap(), y2.parse::<isize>().unwrap());
    }

    fn run_p1(&self) -> usize {
        /*
        the maximum height we can go before the velocity when reaching y = 0
        is equal to the lower y bounds of the target area

        since "gravity" is 1 we can just do the triangle formula sum on the lower
        y bounds of the target area minus one to find the maximum height.
        using that formula works since we just subtract one from the veloicty each step

        the x velocity doesnt matter since we can just assume it's the lowest
        amount for the x velocity to stop at the lower x bound of the target area
        */
        sum(-self.y_range.0 - 1) as usize
    }

    fn run_p2(&self) -> usize {
        let target_area: TargetArea = (self.x_range, self.y_range).into();
        let max_y = -target_area.min_y;

        let mut hits = 0;
        for x in 0..=target_area.max_x {
            if sum(x) < target_area.min_x {
                continue;
            }

            for y in target_area.min_y..max_y {
                if sim(Point(x, y), &target_area) {
                    hits += 1;
                }
            }
        }

        hits as usize
    }
}

#[inline]
fn sum(num: isize) -> isize {
    num * (num + 1) / 2
}

#[inline]
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

#[inline]
fn dir(num: isize) -> isize {
    match num {
        0 => 0,
        _ if num < 0 => -1,
        _ => 1,
    }
}

#[derive(Debug)]
struct TargetArea {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl From<((isize, isize), (isize, isize))> for TargetArea {
    fn from(ranges: ((isize, isize), (isize, isize))) -> Self {
        Self {
            min_x: ranges.0 .0,
            min_y: ranges.1 .0,
            max_x: ranges.0 .1,
            max_y: ranges.1 .1,
        }
    }
}

impl TargetArea {
    #[inline]
    fn contains(&self, point: Point) -> bool {
        point.0 >= self.min_x
            && point.0 <= self.max_x
            && point.1 >= self.min_y
            && point.1 <= self.max_y
    }

    #[inline]
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
