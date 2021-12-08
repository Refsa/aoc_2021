use crate::Runner;
use std::cmp::Ordering;

#[derive(Default)]
pub struct AOC8 {
    parsed: Vec<Line>,
}

#[derive(Default)]
struct Line {
    left_part: Vec<Vec<u8>>,
    right_part: Vec<Vec<u8>>,
}

fn to_segment_id(input: &char) -> u8 {
    match input {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("input out of range"),
    }
}

fn to_segment_char(input: &u8) -> char {
    match input {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        _ => panic!("input out of range"),
    }
}

const ZERO: &[u8] = &[0, 1, 2, 4, 5, 6];
const ONE: &[u8] = &[2, 5];
const TWO: &[u8] = &[0, 2, 3, 4, 6];
const THREE: &[u8] = &[0, 2, 3, 5, 6];
const FOUR: &[u8] = &[1, 2, 3, 5];
const FIVE: &[u8] = &[0, 1, 3, 5, 6];
const SIX: &[u8] = &[0, 1, 3, 4, 5, 6];
const SEVEN: &[u8] = &[0, 3, 5];
const EIGHT: &[u8] = &[0, 1, 2, 3, 4, 5, 6];
const NINE: &[u8] = &[0, 1, 2, 3, 5, 6];

fn to_digit(input: &[u8]) -> Option<u8> {
    match input {
        ZERO => Some(0),
        ONE => Some(1),
        TWO => Some(2),
        THREE => Some(3),
        FOUR => Some(4),
        FIVE => Some(5),
        SIX => Some(6),
        SEVEN => Some(7),
        EIGHT => Some(8),
        NINE => Some(9),
        _ => None,
    }
}

fn sort_by_len(a: &Vec<u8>, b: &Vec<u8>) -> Ordering {
    a.len().cmp(&b.len())
}

fn parse_digits(input: &str) -> Vec<Vec<u8>> {
    input
        .split_terminator(" ")
        .map(|v| v.chars())
        .map(|v| v.map(|c| to_segment_id(&c)).collect())
        .collect()
}

impl Runner for AOC8 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        let lines: Vec<Line> = input
            .iter()
            .map(|e| {
                let (l, r) = e.split_once(" | ").unwrap();

                let mut l = parse_digits(l);
                // l.sort_by(sort_by_len);
                let r = parse_digits(r);
                // r.sort_by(sort_by_len);

                Line {
                    left_part: l,
                    right_part: r,
                }
            })
            .collect();
        self.parsed = lines;
    }
    fn run_p1(&self) -> usize {
        self.parsed
            .iter()
            .flat_map(|e| e.right_part.clone())
            .filter(|e| match e.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            })
            .count()
    }
    fn run_p2(&self) -> usize {
        self.parsed
            .iter()
            .fold(0usize, |acc, e| acc + solve_line(e))

        // solve_line(&self.parsed[1]);
        // solve_line(&self.parsed[2]);
        // 0
    }
}

fn solve_line(line: &Line) -> usize {
    let lookup = solve_signal(line);

    let mut num = "".to_string();
    for n in &line.right_part {
        /* if [2, 3, 4].contains(&n.len()) {
            let digit = match n.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                _ => panic!(),
            };
            num = format!("{}{}", num, digit);
        } else */ /* {
                      let mut digit = Vec::new();
                      for d in n {
                          digit.push(lookup.iter().position(|v| v == d).unwrap() as u8);
                      }
                      digit.sort();
                      print!("{:?} - {:?} - ", n, digit);

                      let digit = to_digit(&digit[..]);
                      println!("{:?} ", digit);

                      num = format!("{}{}", num, digit);
                  } */
    }
    let num = num.parse::<usize>().unwrap();
    println!("\n{}", num);
    num
}

/*
 aaa
b   c
b   c
b   c
 ddd
e   f
e   f
e   f
 ggg

counts:
    e: 4 - 0, 2, 6, 8

    b: 6 - 0, 4, 5, 6, 8, 9

    f: 9 - 0, 1, 3, 4, 5, 6, 7, 8, 9

    d: 7 - 2, 3, 4, 5, 6, 8, 9 | 4 unique
    g: 7 - 0, 2, 3, 5, 6, 8, 9 | 4 unique

    a: 8 - 0, 2, 3, 5, 6, 7, 8, 9 | 5 unique
    c: 8 - 0, 1, 2, 3, 4, 7, 8, 9 | 5 unique
*/

fn solve_signal(input: &Line) -> Vec<[u8; 7]> {
    let mut counts = vec![(0u8, Vec::new()); 7];
    println!();

    for (i, d) in input.left_part.iter().enumerate() {
        // println!("{:?}", d);

        for v in d {
            counts[*v as usize].0 += 1;
            counts[*v as usize].1.push(i);
        }
    }

    println!("Counts: {:?}", counts);

    let mut ordering = [0u8; 7];
    // e
    {
        let idx = {
            let cidxs: Vec<(usize, &(u8, Vec<usize>))> = counts
                .iter()
                .enumerate()
                .filter(|(i, e)| e.0 == 4)
                .collect();
            ordering[to_segment_id(&'e') as usize] = cidxs[0].0 as u8;
            cidxs[0].0
        };
        // counts.remove(idx);
    }

    // b
    {
        let idx = {
            let cidxs: Vec<(usize, &(u8, Vec<usize>))> = counts
                .iter()
                .enumerate()
                .filter(|(i, e)| e.0 == 6)
                .collect();
            ordering[to_segment_id(&'b') as usize] = cidxs[0].0 as u8;
            cidxs[0].0
        };
        // counts.remove(idx);
    }

    // f
    {
        let idx = {
            let cidxs: Vec<(usize, &(u8, Vec<usize>))> = counts
                .iter()
                .enumerate()
                .filter(|(i, e)| e.0 == 9)
                .collect();
            ordering[to_segment_id(&'f') as usize] = cidxs[0].0 as u8;
            cidxs[0].0
        };
        // counts.remove(idx);
    }

    // a or c
    {
        let cidxs: Vec<(usize, &(u8, Vec<usize>))> = counts
            .iter()
            .enumerate()
            .filter(|(i, e)| e.0 == 8)
            .collect();

        let numba_one = input
            .left_part
            .iter()
            .enumerate()
            .filter(|(i, e)| e.len() == 2)
            .nth(0)
            .unwrap();

        println!("{:?} | {:?}", cidxs, numba_one);

        if cidxs[0].1 .1.contains(&(numba_one.1[0] as usize))
            && cidxs[0].1 .1.contains(&(numba_one.1[1] as usize))
        {
            ordering[to_segment_id(&'c') as usize] = cidxs[0].0 as u8;
            ordering[to_segment_id(&'a') as usize] = cidxs[1].0 as u8;
        } else {
            ordering[to_segment_id(&'a') as usize] = cidxs[0].0 as u8;
            ordering[to_segment_id(&'c') as usize] = cidxs[1].0 as u8;
        }
    }

    // g or d
    {
        let cidxs: Vec<(usize, &(u8, Vec<usize>))> = counts
            .iter()
            .enumerate()
            .filter(|(i, e)| e.0 == 7)
            .collect();

        let numba_four = input
            .left_part
            .iter()
            .enumerate()
            .filter(|(i, e)| e.len() == 4)
            .nth(0)
            .unwrap();

        println!("{:?} | {:?}", cidxs, numba_four);

        if cidxs[0].1 .1.contains(&(numba_four.1[0] as usize))
            && cidxs[0].1 .1.contains(&(numba_four.1[1] as usize))
            && cidxs[0].1 .1.contains(&(numba_four.1[2] as usize))
            && cidxs[0].1 .1.contains(&(numba_four.1[3] as usize))
        {
            ordering[to_segment_id(&'d') as usize] = cidxs[0].0 as u8;
            ordering[to_segment_id(&'g') as usize] = cidxs[1].0 as u8;
        } else {
            ordering[to_segment_id(&'g') as usize] = cidxs[0].0 as u8;
            ordering[to_segment_id(&'d') as usize] = cidxs[1].0 as u8;
        }
    }

    println!("Ordering: {:?}", ordering);
    for o in &ordering {
        println!("{}", to_segment_char(o));
    }

    vec![ordering]
}
