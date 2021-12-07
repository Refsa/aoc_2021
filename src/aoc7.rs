use crate::Runner;

pub struct AOC7 {}
impl AOC7 {
    fn parse(input: &Vec<String>) -> Vec<isize> {
        input[0].split_terminator(",").map(|e| e.parse::<isize>().unwrap()).collect()
    }
}

fn sum(num: isize) -> isize {
    match num {
        0 => 0,
        _ => num + sum(num - 1)
    }
}

impl Runner for AOC7 {
    fn run_p1(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let parsed = Self::parse(input);

        let (min, max) = parsed.iter().fold((0isize, 0isize), |acc, &e| (acc.0.min(e), acc.1.max(e)));

        let mut costs: Vec<isize> = vec![0isize; parsed.len()];
        let mut min_cost = (0, 1 << 32);

        for i in min..=max {
            for (idx, p) in parsed.iter().enumerate() {
                costs[idx] = (p - i).abs();
            }
            
            let cost = costs.iter().sum();
            if cost < min_cost.1 {
                min_cost.1 = cost;
                min_cost.0 = i;
            }
        }

        min_cost.1 as usize
    }
    fn run_p2(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let parsed = Self::parse(input);

        let (min, max) = parsed.iter().fold((0isize, 0isize), |acc, &e| (acc.0.min(e), acc.1.max(e)));

        let mut costs: Vec<isize> = vec![0isize; parsed.len()];
        let mut min_cost = 1 << 32;

        for i in min..=max {
            for (idx, p) in parsed.iter().enumerate() {
                costs[idx] = sum((p - i).abs());
            }
            
            let cost = costs.iter().sum();
            if cost < min_cost {
                min_cost = cost;
            }
        }

        min_cost as usize
    }
}
