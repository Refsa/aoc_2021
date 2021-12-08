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
        _ => 'X',
    }
}

const ZERO: &[u8] = &[0, 1, 2, 4, 5, 6];
const ONE: &[u8] = &[2, 5];
const TWO: &[u8] = &[0, 2, 3, 4, 6];
const THREE: &[u8] = &[0, 2, 3, 5, 6];
const FOUR: &[u8] = &[1, 2, 3, 5];
const FIVE: &[u8] = &[0, 1, 3, 5, 6];
const SIX: &[u8] = &[0, 1, 3, 4, 5, 6];
const SEVEN: &[u8] = &[0, 2, 5];
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
        .map(|v| {
            let mut e = v.chars().map(|c| to_segment_id(&c)).collect::<Vec<u8>>();
            e.sort();
            e
        })
        .collect()
}

impl Runner for AOC8 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        self.parsed = input
            .iter()
            .map(|e| {
                e.split_once(" | ")
                    .map(|(l, r)| Line {
                        left_part: parse_digits(l),
                        right_part: parse_digits(r),
                    })
                    .unwrap()
            })
            .collect();
    }
    fn run_p1(&self) -> usize {
        let wanted = [2, 3, 4, 7];
        self.parsed
            .iter()
            .flat_map(|e| e.right_part.clone())
            .filter(|e| wanted.contains(&e.len()))
            .count()
    }
    fn run_p2(&self) -> usize {
        self.parsed
            .iter()
            .fold(0usize, |acc, e| acc + solve_line(e))
    }
}

fn solve_line(line: &Line) -> usize {
    let lookup = solve_signal(line);

    line.right_part
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, n)| {
            let mut digit = Vec::new();
            for d in n {
                digit.push(lookup.iter().position(|v| v == d).unwrap() as u8);
            }
            digit.sort();
            let num = to_digit(&digit[..]).unwrap() as usize;
            acc + 10u32.pow((3 - i) as u32) as usize * num
        })
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

lens:
    2 : 1 - c, f
    3 : 7 - a, c f
    4 : 4 - b, c, d, f
    5 : 2, 3, 5
    6 : 0, 6, 9
    7 : 8 - a, b, c, d, e, f
*/

fn solve_signal(input: &Line) -> [u8; 7] {
    let mut counts = vec![Vec::new(); 7];
    let mut ordering = [255u8; 7];

    for (i, d) in input.left_part.iter().enumerate() {
        for v in d {
            counts[*v as usize].push(i as u8);
        }
    }

    let one = input
        .left_part
        .iter()
        .filter(|e| e.len() == 2)
        .nth(0)
        .unwrap();
    let seven = input
        .left_part
        .iter()
        .filter(|e| e.len() == 3)
        .nth(0)
        .unwrap();
    let eight = input
        .left_part
        .iter()
        .filter(|e| e.len() == 7)
        .nth(0)
        .unwrap();

    for i in 0..7 {
        let c = &counts[i];
        match c.len() {
            4 => ordering[to_segment_id(&'e') as usize] = i as u8,
            6 => ordering[to_segment_id(&'b') as usize] = i as u8,
            9 => ordering[to_segment_id(&'f') as usize] = i as u8,
            _ => (),
        }
    }

    let b = ordering[to_segment_id(&'b') as usize];
    let e = ordering[to_segment_id(&'e') as usize];
    let f = ordering[to_segment_id(&'f') as usize];
    let (a_one, b_one) = (one[0], one[1]);
    if f == a_one {
        ordering[to_segment_id(&'c') as usize] = b_one;
    } else {
        ordering[to_segment_id(&'c') as usize] = a_one;
    }

    let c = ordering[to_segment_id(&'c') as usize];
    let (a_sev, b_sev, c_sev) = (seven[0], seven[1], seven[2]);
    if (c == a_sev && f == b_sev) || (c == b_sev && f == a_sev) {
        ordering[to_segment_id(&'a') as usize] = c_sev;
    } else if (c == b_sev && f == c_sev) || (c == c_sev && f == b_sev) {
        ordering[to_segment_id(&'a') as usize] = a_sev;
    } else if (c == a_sev && f == c_sev) || (c == c_sev && f == a_sev) {
        ordering[to_segment_id(&'a') as usize] = b_sev;
    }

    let a = ordering[to_segment_id(&'a') as usize];
    let known = [a, b, c, e, f];

    let g_filter: Vec<Vec<&u8>> = input
        .left_part
        .iter()
        .filter(|e| e.len() == 6)
        .map(|e| {
            e.iter()
                .filter(|v| !known.contains(v))
                .collect::<Vec<&u8>>()
        })
        .filter(|e| e.len() == 1)
        .collect();
    ordering[to_segment_id(&'g') as usize] = *g_filter[0][0];
    let g = ordering[to_segment_id(&'g') as usize];

    let known = [a, b, c, e, f, g];
    let d = eight.iter().find(|e| !known.contains(e)).unwrap();
    ordering[to_segment_id(&'d') as usize] = *d;

    ordering
}
