use std::collections::HashMap;

use crate::runner::Runner;

#[derive(Debug, Default)]
struct Pair {
    left: [u8;2],
    right: u8,
}

#[derive(Default)]
pub struct AOC14 {
    template: Vec<u8>,
    pair_lookup: HashMap<[u8;2], u8>
}

impl Runner for AOC14 {
    fn parse(&mut self, input: &Vec<String>) {
        let template = input[0].clone().into_bytes();
        let pairs: Vec<Pair> = input[2..]
            .iter()
            .map(|e| {
                let (l, r) = e.split_once(" -> ").unwrap();
                let l = l.as_bytes();
                let l = [l[0], l[1]];
                Pair {
                    left: l,
                    right: r.bytes().nth(0).unwrap(),
                }
            })
            .collect();

        let mut pair_lookup = HashMap::new();
        for p in &pairs {
            pair_lookup.insert(p.left.clone(), p.right);
        }

        self.pair_lookup = pair_lookup;
        self.template = template;
    }

    fn run_p1(&self) -> usize {
        let mut seq = self.template.clone();
        let mut inserts = Vec::new();

        for _ in 0..10 {
            step(&mut seq, &mut inserts, &self.pair_lookup);
        }
        
        let mut counts = HashMap::new();
        for s in seq {
            let cnt = counts.entry(s).or_insert(0usize);
            *cnt += 1;
        }

        let mut largest = 0;
        let mut smallest = 1 << 32;
        for (_, v) in counts {
            largest = largest.max(v);
            smallest = smallest.min(v);
        }

        largest - smallest
    }

    fn run_p2(&self) -> usize {
        let mut seq = self.template.clone();
        let mut inserts = Vec::new();

        for i in 0..40 {
            println!("{}", i);
            step(&mut seq, &mut inserts, &self.pair_lookup);
        }
        
        let mut counts = HashMap::new();
        for s in seq {
            let cnt = counts.entry(s).or_insert(0usize);
            *cnt += 1;
        }

        let mut largest = 0;
        let mut smallest = 1 << 32;
        for (_, v) in counts {
            largest = largest.max(v);
            smallest = smallest.min(v);
        }

        largest - smallest
    }
}

fn step(seq: &mut Vec<u8>, inserts: &mut Vec<(usize, u8)>, pairs: &HashMap<[u8;2], u8>) {
    /* for i in (0..(seq.len() - 1)).rev() {
        let wnd = &seq[i..i+2];
        let pair = pairs.get(wnd);
        if let Some(pair) = pair {
            inserts.push((i + 1, *pair))
        }
    } */

    for (i, c) in inserts.iter() {
        seq.insert(*i, *c);
    }
    inserts.clear();
}
