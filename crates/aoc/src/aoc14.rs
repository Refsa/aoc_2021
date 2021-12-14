use std::collections::HashMap;

use crate::runner::Runner;

type Pattern = (u8, u8);
type Element = u8;
type PairCounter = HashMap<Pattern, usize>;

#[derive(Default)]
pub struct AOC14 {
    template: Vec<Element>,
    pair_lookup: HashMap<Pattern, Element>,
}

impl Runner for AOC14 {
    fn parse(&mut self, input: &Vec<String>) {
        let template = input[0].clone().into_bytes();
        let pairs: HashMap<Pattern, Element> = input[2..]
            .iter()
            .map(|e| {
                let (l, r) = e.split_once(" -> ").unwrap();
                let l = l.as_bytes();
                let l = (l[0], l[1]);
                let v = (l, r.bytes().nth(0).unwrap());
                v
            })
            .collect();

        self.pair_lookup = pairs;
        self.template = template;
    }

    fn run_p1(&self) -> usize {
        run(&self, 10)
    }

    fn run_p2(&self) -> usize {
        run(&self, 40)
    }
}

fn run(aoc: &AOC14, iter: usize) -> usize {
    let mut map: PairCounter = HashMap::new();
    for i in 0..aoc.template.len() - 1 {
        let v = (aoc.template[i], aoc.template[i + 1]);
        map.entry(v).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..iter {
        map = step(&map, &aoc.pair_lookup);
    }

    let mut counts = HashMap::new();
    for ((a, _), n) in map {
        counts.entry(a).and_modify(|e| *e += n).or_insert(n);
    }
    *counts.get_mut(&aoc.template.last().unwrap()).unwrap() += 1;

    let (min, max) = counts
        .values()
        .fold((usize::MAX, 0), |(min, max), &v| (min.min(v), max.max(v)));

    (max + 1) - (min + 1)
}

fn step(
    map: &PairCounter,
    pairs: &HashMap<Pattern, Element>,
) -> PairCounter {
    let mut new_map = HashMap::new();

    for (&(a, b), &n) in map {
        if let Some(&c) = pairs.get(&(a, b)) {
            let l = (a, c);
            let r = (c, b);
            new_map.entry(l).and_modify(|e| *e += n).or_insert(n);
            new_map.entry(r).and_modify(|e| *e += n).or_insert(n);
        }
    }

    new_map
}
