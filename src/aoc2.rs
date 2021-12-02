use crate::Runner;

pub struct AOC2 {}

impl Runner for AOC2 {
    fn run_p1(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let parsed = AOC2::parse_input(input);

        let mut depth: isize = 0;
        let mut pos: isize = 0;

        for dir in parsed {
            match dir {
                Direction::Forward(val) => pos += val,
                Direction::Down(val) => depth += val,
                Direction::Up(val) => depth -= val,
            }
        }

        (depth * pos) as usize
    }
    fn run_p2(&self, input: &std::vec::Vec<std::string::String>) -> usize {
        let parsed = AOC2::parse_input(input);

        let mut depth = 0isize;
        let mut pos = 0isize;
        let mut aim = 0isize;
        
        for dir in parsed {
            match dir {
                Direction::Forward(val) => {
                    pos += val;
                    depth += aim * val;
                }
                Direction::Down(val) => {
                    aim += val;
                }
                Direction::Up(val) => {
                    aim -= val;
                }
            }
        }

        (depth * pos) as usize
    }
}

enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize)
}

impl AOC2 {
    fn parse_input(input: &Vec<String>) -> Vec<Direction> {
        input.iter().map(|e| {
            let (dir, val) =  e.split_once(" ").unwrap();
            let val: isize = val.parse::<isize>().unwrap();
            match dir {
                "forward" => Direction::Forward(val),
                "up" => Direction::Up(val),
                "down" => Direction::Down(val),
                _ => panic!("Input not handled {}", dir)
            }
        })
        .collect()
    }
}