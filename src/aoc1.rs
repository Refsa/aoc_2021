use crate::runner::Runner;

#[derive(Default)]
pub struct AOC1 {
    parsed: Vec<i64>,
}

impl Runner for AOC1 {
    fn parse(&mut self, input: &Vec<String>) {
        self.parsed = input.iter().map(|e| e.parse::<i64>().unwrap()).collect();
    }

    fn run_p1(&self) -> usize {
        self.parsed.windows(2).filter(|e| (e[1] - e[0]) > 0).count()
    }
    fn run_p2(&self) -> usize {
        self.parsed
            .windows(3)
            .map(|e| e[0] + e[1] + e[2])
            .collect::<Vec<i64>>()
            .windows(2)
            .filter(|e| (e[1] - e[0]) > 0)
            .count()
    }
}
