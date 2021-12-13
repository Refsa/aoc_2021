use crate::runner::Runner;

#[derive(Default)]
pub struct AOC13 {
    width: usize,
    height: usize,
    folds: Vec<Fold>,
    dots: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

#[derive(Debug, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl Runner for AOC13 {
    fn parse(&mut self, input: &Vec<String>) {
        let empty_index = input.iter().position(|e| e == "").unwrap();
        let mut max_x = 0;
        let mut max_y = 0;

        let points: Vec<Point> = input[..empty_index]
            .iter()
            .map(|e| e.split_once(",").unwrap())
            .map(|(l, r)| {
                let (l, r) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
                max_x = max_x.max(l);
                max_y = max_y.max(r);
                Point { x: l, y: r }
            })
            .collect();

        let folds: Vec<Fold> = input[empty_index + 1..]
            .iter()
            .map(|e| e.split_once('=').unwrap())
            .map(|(l, r)| match l {
                "fold along y" => Fold::AlongY(r.parse::<usize>().unwrap()),
                "fold along x" => Fold::AlongX(r.parse::<usize>().unwrap()),
                _ => unreachable!(),
            })
            .collect();

        self.width = max_x + 1;
        self.height = max_y + 1;
        self.folds = folds;
        self.dots = points;
    }

    fn run_p1(&self) -> usize {
        let mut grid = Grid::new(self.width, self.height);
        grid.place_points(&self.dots);
        grid.fold(self.folds[0]);

        grid.count_dots()
    }

    fn run_p2(&self) -> usize {
        let mut grid = Grid::new(self.width, self.height);
        grid.place_points(&self.dots);

        for &fold in &self.folds {
            grid.fold(fold);
        }

        println!();
        for l in &grid.data[..grid.height] {
            println!(
                "{:?}",
                &l[..grid.width]
                    .iter()
                    .map(|&e| if e > 0 { '#' } else { '.' })
                    .collect::<Vec<char>>()
            );
        }

        0
    }
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<usize>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![vec![0usize; width]; height],
        }
    }

    fn place_points(&mut self, points: &Vec<Point>) {
        for p in points {
            self.data[p.y][p.x] += 1;
        }
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::AlongX(val) => self.fold_x(val),
            Fold::AlongY(val) => self.fold_y(val),
        }
    }

    fn fold_x(&mut self, x: usize) {
        for j in 0..self.height {
            let (left, right) = self.data[j].split_at_mut(x);
            for i in 0..x {
                left[i] += right[x - i];
            }
        }
        self.width = x;
    }

    fn fold_y(&mut self, y: usize) {
        let (upper_half, lower_half) = self.data.split_at_mut(y);

        for j in 0..y {
            for i in 0..self.width {
                upper_half[j][i] += lower_half[y - j][i];
            }
        }

        self.height = y;
    }

    fn count_dots(&self) -> usize {
        self.data.iter().take(self.height).fold(0usize, |acc, e| {
            let e_sum: usize = e
                .iter()
                .take(self.width)
                .map(|&e| if e > 0 { 1 } else { 0 })
                .sum();
            acc + e_sum
        })
    }
}
