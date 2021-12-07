use crate::Runner;
use rayon::prelude::*;

#[derive(Default)]
pub struct AOC6 {
    parsed: Vec<i8>,
}

impl Runner for AOC6 {
    fn parse(&mut self, input: &Vec<String>){
        self.parsed = input[0]
            .split_terminator(",")
            .map(|e| e.parse::<i8>().unwrap())
            .collect();
    }

    fn run_p1(&self) -> usize {
        let mut buckets = [0usize; 9];
        let mut temp = [0usize; 9];

        for p in &self.parsed {
            buckets[*p as usize] += 1;
        }

        for i in 0..80 {
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
    fn run_p2(&self) -> usize {
        let mut buckets = [0usize; 9];
        let mut temp = [0usize; 9];

        for p in &self.parsed {
            buckets[*p as usize] += 1;
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
