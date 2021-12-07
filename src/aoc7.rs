use crate::Runner;

#[derive(Default)]
pub struct AOC7 {
    parsed: Vec<isize>
}

fn sum(num: isize) -> isize {
    num * (num + 1) / 2
}

impl Runner for AOC7 {
    fn parse(&mut self, input: &Vec<String>) {
        self.parsed = input[0]
            .split_terminator(",")
            .map(|e| e.parse::<isize>().unwrap())
            .collect()
    }

    fn run_p1(&self) -> usize {
        let (min, max) = self.parsed
            .iter()
            .fold((0isize, 0isize), |acc, &e| (acc.0.min(e), acc.1.max(e)));

        let min_cost = (min..=max).fold(1 << 32, |acc, i| {
            acc.min(self.parsed.iter().map(|e| (e - i).abs()).sum())
        });

        min_cost as usize
    }
    fn run_p2(&self) -> usize {
        let (min, max) = self.parsed
            .iter()
            .fold((0isize, 0isize), |acc, &e| (acc.0.min(e), acc.1.max(e)));

        let min_cost = (min..=max).fold(1 << 32, |acc, i| {
            acc.min(self.parsed.iter().map(|e| sum((e - i).abs())).sum())
        });

        min_cost as usize
    }
}
