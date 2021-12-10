use crate::runner::Runner;

#[derive(Default)]
pub struct AOC10 {
    parsed: Vec<Vec<char>>,
}

fn to_points_p1(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn to_points_p2(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn is_matching(l: char, r: char) -> bool {
    match l {
        '(' if r == ')' => true,
        '{' if r == '}' => true,
        '[' if r == ']' => true,
        '<' if r == '>' => true,
        _ => false,
    }
}

const OPEN: [char; 4] = ['[', '{', '(', '<'];

impl Runner for AOC10 {
    fn parse(&mut self, input: &std::vec::Vec<std::string::String>) {
        self.parsed = input.iter().map(|e| e.chars().collect()).collect();
    }
    fn run_p1(&self) -> usize {
        let mut postfix = Vec::new();
        let mut sum = 0;

        for l in &self.parsed {
            postfix.clear();
            let mut last_char = ' ';
            for c in l {
                if OPEN.contains(c) {
                    postfix.push(c);
                } else {
                    let e = postfix.pop().unwrap();
                    if !is_matching(*e, *c) {
                        last_char = *c;
                        break;
                    }
                }
            }
            if last_char != ' ' {
                sum += to_points_p1(last_char);
            }
        }

        sum
    }
    fn run_p2(&self) -> usize {
        let mut postfix = Vec::new();
        let mut lsums: Vec<usize> = self
            .parsed
            .iter()
            .filter_map(|l| {
                postfix.clear();
                for c in l {
                    if OPEN.contains(c) {
                        postfix.push(*c);
                    } else {
                        let e = postfix.pop().unwrap();
                        if !is_matching(e, *c) {
                            return None;
                        }
                    }
                }
                Some(
                    postfix
                        .iter()
                        .rev()
                        .fold(0usize, |acc, &e| (5 * acc) + to_points_p2(e)),
                )
            })
            .collect();

        lsums.sort_unstable();

        lsums[lsums.len() / 2]
    }
}
