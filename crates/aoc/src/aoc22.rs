use std::collections::HashSet;

use crate::runner::Runner;

#[derive(Default)]
pub struct AOC22 {
    cuboids: Vec<Cuboid>,
}

fn parse_range(input: &str) -> (isize, isize) {
    let (x0, x1) = &input.trim()[2..]
        .split_once("..")
        .map(|e| (e.0.parse::<isize>().unwrap(), e.1.parse::<isize>().unwrap()))
        .unwrap();

    (*x0, *x1)
}

impl Runner for AOC22 {
    fn parse(&mut self, input: &Vec<String>) {
        self.cuboids = input
            .iter()
            .map(|e| {
                let state = if e.starts_with("on") { true } else { false };

                let xyz = e.replace("on ", "").replace("off ", "");
                let mut xyz = xyz.split_terminator(',');

                let x = parse_range(xyz.next().unwrap());
                let y = parse_range(xyz.next().unwrap());
                let z = parse_range(xyz.next().unwrap());

                Cuboid { state, x, y, z }
            })
            .collect();
    }

    fn run_p1(&self) -> usize {
        /* let mut map: HashSet<(isize, isize, isize)> = HashSet::new();

        for c in &self.cuboids {
            for x in c.x.0..=c.x.1 {
                if x < -50 || x > 50 {
                    continue;
                }
                for y in c.y.0..=c.y.1 {
                    if y < -50 || y > 50 {
                        continue;
                    }
                    for z in c.z.0..=c.z.1 {
                        if z < -50 || z > 50 {
                            continue;
                        }
                        if c.state {
                            map.insert((x, y, z));
                        } else {
                            map.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        map.len() */

        let cuboids = self
            .cuboids
            .iter()
            .filter(|e| {
                e.x.0 >= -50
                    && e.x.1 <= 50
                    && e.y.0 >= -50
                    && e.y.1 <= 50
                    && e.z.0 >= -50
                    && e.z.1 <= 50
            })
            .map(|e| e.clone())
            .collect::<Vec<Cuboid>>();
        run(&cuboids)
    }

    fn run_p2(&self) -> usize {
        run(&self.cuboids)
    }
}

fn run(cuboids: &Vec<Cuboid>) -> usize {
    let mut tot = 0;

    for i in 0..cuboids.len() {
        let outer = &cuboids[i];
        let mut count = if outer.state { outer.count() } else { 0 };

        for j in i..cuboids.len() {
            let inner = &cuboids[j];

            if outer.overlaps(inner) {
                count -= outer.intersection(&inner).count();
            }
        }

        tot += count;
    }

    tot as usize
}

#[derive(Default, Debug, Clone)]
struct Cuboid {
    state: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn new(state: bool, x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Cuboid {
        Self {
            state: state,
            x: x,
            y: y,
            z: z,
        }
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        self.x.0 >= other.x.0 && self.x.0 <= other.x.1
            || self.y.0 >= other.y.0 && self.y.0 <= other.y.1
            || self.z.0 >= other.z.0 && self.z.0 <= other.z.1
            || self.x.1 >= other.x.0 && self.x.1 <= other.x.1
            || self.y.1 >= other.y.0 && self.y.1 <= other.y.1
            || self.z.1 >= other.z.0 && self.z.1 <= other.z.1
    }

    fn count(&self) -> isize {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn intersection(&self, other: &Cuboid) -> Cuboid {
        Cuboid {
            state: self.state && other.state,
            x: find_overlap(self.x, other.x),
            y: find_overlap(self.y, other.y),
            z: find_overlap(self.z, other.z),
        }
    }
}

fn find_overlap(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    if a.0 < b.0 {
        (b.0, a.1)
    } else {
        (a.0, b.1)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_cuboid_overlaps() {
        let ca = Cuboid {
            state: false,
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
        };

        let cb = Cuboid {
            state: false,
            x: (11, 13),
            y: (11, 13),
            z: (11, 13),
        };

        assert!(ca.overlaps(&cb));
    }

    #[test]
    fn test_cuboid_dont_overlaps() {
        let ca = Cuboid {
            state: false,
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
        };

        let cb = Cuboid {
            state: false,
            x: (13, 16),
            y: (13, 16),
            z: (13, 16),
        };

        let cc = Cuboid {
            state: false,
            x: (5, 9),
            y: (5, 9),
            z: (5, 9),
        };

        assert!(!ca.overlaps(&cb));
        assert!(!ca.overlaps(&cc));
    }

    #[test]
    fn test_cuboid_count() {
        let ca = Cuboid {
            state: false,
            x: (10, 12),
            y: (10, 12),
            z: (10, 12),
        };

        assert_eq!(27, ca.count());
    }

    #[test]
    fn test_cuboid_union() {
        let ca = Cuboid::new(false, (10, 12), (10, 12), (10, 12));
        let cb = Cuboid::new(false, (11, 13), (11, 13), (11, 13));

        let its = ca.intersection(&cb);
        let cnt = ca.count() + cb.count() - its.count();

        assert_eq!(27 + 19, cnt);
    }
}
