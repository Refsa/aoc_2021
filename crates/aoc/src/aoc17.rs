use std::{
    ops::{Add, RangeInclusive, Sub},
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

        let (min_x, max_x) = find_vel_range_x(target_area.min_x, target_area.max_x);
        let max_y = -target_area.min_y;

        let mut max_h = 0;
        for x in min_x..=max_x {
            for y in 1..max_y {
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
        let (min_x, _) = find_vel_range_x(target_area.min_x, target_area.max_x);
        let max_y = -target_area.min_y;

        let mut hits = 0;
        for x in min_x..=target_area.max_x {
            for y in target_area.min_y..max_y {
                let vel = Point(x, y);
                if sim(vel, &target_area) {
                    hits += 1;
                }
            }
        }

        hits as usize
    }
}

fn sum(num: isize) -> isize {
    num * (num + 1) / 2
}

fn find_vel_range_x(xs: isize, xe: isize) -> (isize, isize) {
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