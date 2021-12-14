use crate::runner::Runner;

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

        for p in &self.parsed {
            buckets[*p as usize] += 1;
        }

        for _ in 0..80 {
            let fst = buckets[0];
            buckets.rotate_left(1);
            buckets[6] += fst;
            buckets[8] = fst;
        }

        buckets.iter().sum::<usize>() as usize
    } 
    fn run_p2(&self) -> usize {
        let mut buckets = [0usize; 9];

        for p in &self.parsed {
            buckets[*p as usize] += 1;
        }

        for _ in 0..256 {
            let fst = buckets[0];
            buckets.rotate_left(1);
            buckets[6] += fst;
            buckets[8] = fst;
        }

        buckets.iter().sum::<usize>() as usize
    }
}
