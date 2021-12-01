use crate::runner::Runner;

pub struct AOC1 {}

impl Runner for AOC1 {
    fn run_p1(&self, input: &Vec<std::string::String>) -> usize {
        let parsed = AOC1::parse_input(input);
        parsed.windows(2).filter(|e| (e[1] - e[0]) > 0).count()
    }
    fn run_p2(&self, input: &Vec<std::string::String>) -> usize {
        let parsed = AOC1::parse_input(input);

        parsed
            .windows(3)
            .map(|e| e[0] + e[1] + e[2])
            .collect::<Vec<i64>>()
            .windows(2)
            .filter(|e| (e[1] - e[0]) > 0)
            .count()
    }
    fn test(&self, input: &Vec<std::string::String>) -> usize {
        let parsed = AOC1::parse_input(input);
        parsed.windows(2).filter(|e| (e[1] - e[0]) > 0).count()
    }
}

impl AOC1 {
    fn parse_input(input: &Vec<String>) -> Vec<i64> {
        input.iter().map(|e| e.parse::<i64>().unwrap()).collect()
    }
}
