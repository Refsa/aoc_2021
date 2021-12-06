use crate::Runner;
use rayon::prelude::*;

pub struct AOC6 {}

impl AOC6 {
    fn parse(input: &Vec<String>) -> Vec<i8> {
        input[0]
            .split_terminator(",")
            .map(|e| e.parse::<i8>().unwrap())
            .collect()
    }
}

impl Runner for AOC6 {
    fn run_p1(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let mut parsed = Self::parse(input);

        for i in 0..80 {
            let len = parsed.len();
            for j in 0..len {
                parsed[j] = parsed[j] - 1;
                if parsed[j] < 0 {
                    parsed[j] = 6;
                    parsed.push(8);
                }
            }
        }

        parsed.len()
    } 
    fn run_p2(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let parsed = Self::parse(input);
        let mut buckets = [0usize; 9];
        let mut temp = [0usize; 9];

        for p in parsed {
            buckets[p as usize] += 1;
        }

        for i in 0..256 {
            for j in 1..buckets.len() {
                temp[j - 1] = buckets[j];
            }
            temp[6] += buckets[0];
            temp[8] = buckets[0];

            for j in 0..9 {
                buckets[j] = temp[j];
            }
        }

        buckets.iter().sum::<usize>() as usize
    }
}
