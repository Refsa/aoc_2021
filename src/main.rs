use aoc::runner::Runner;

fn get_runner(day: usize) -> Box<dyn Runner> {
    match day {
        1 => Box::new(aoc::aoc1::AOC1::default()),
        2 => Box::new(aoc::aoc2::AOC2::default()),
        3 => Box::new(aoc::aoc3::AOC3::default()),
        4 => Box::new(aoc::aoc4::AOC4::default()),
        5 => Box::new(aoc::aoc5::AOC5::default()),
        6 => Box::new(aoc::aoc6::AOC6::default()),
        7 => Box::new(aoc::aoc7::AOC7::default()),
        8 => Box::new(aoc::aoc8::AOC8::default()),
        9 => Box::new(aoc::aoc9::AOC9::default()),
        10 => Box::new(aoc::aoc10::AOC10::default()),
        11 => Box::new(aoc::aoc11::AOC11::default()),
        12 => Box::new(aoc::aoc12::AOC12::default()),
        13 => Box::new(aoc::aoc13::AOC13::default()),
        14 => Box::new(aoc::aoc14::AOC14::default()),
        15 => Box::new(aoc::aoc15::AOC15::default()),
        16 => Box::new(aoc::aoc16::AOC16::default()),
        _ => panic!("Runner for day {} not implemented", day),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        panic!("expected first argument to be day as number");
    }
    let day = match args[1].parse::<usize>() {
        Ok(day) => day,
        Err(_) => panic!("expected first argument to be a number"),
    };

    let mut runner = get_runner(day);
    let input = get_input(day);
    let test_data = get_test_data(day);

    eprint!("Test P1 | ");
    runner.parse(&test_data.input);
    let test_answer = runner.run_p1();
    if test_answer != test_data.answer_p1 {
        eprintln!(
            "Failed: expected {} - got answer {}",
            test_data.answer_p1, test_answer
        );
        return;
    }
    eprintln!("Success");

    eprint!("Part 1  | ");
    runner.parse(&input);
    let p1 = runner.run_p1();
    eprintln!("{}", p1);

    eprint!("Test P2 | ");
    runner.parse(&test_data.input);
    let test_answer = runner.run_p2();
    if test_answer != test_data.answer_p2 {
        eprintln!(
            "Failed: expected {} - got answer {}",
            test_data.answer_p2, test_answer
        );
        return;
    }
    eprintln!("Success");

    eprint!("Part 2  | ");
    runner.parse(&input);
    let p2 = runner.run_p2();
    eprintln!("{}", p2);
}

fn read_file(path: String) -> Result<Vec<String>, String> {
    let pb = std::path::PathBuf::from(&path);
    if !pb.exists() {
        return Err(format!("Couldnt find file at path: {}", path));
    }
    let content = std::fs::read_to_string(pb).unwrap();
    Ok(content.lines().map(|e| e.to_string()).collect())
}

fn get_input(day: usize) -> Vec<String> {
    let path = format!("./resources/day{}.txt", day);
    let content = read_file(path);

    match content {
        Ok(content) => content,
        Err(_) => panic!("Couldnt find puzzle input for day {}", day),
    }
}

struct TestData {
    input: Vec<String>,
    answer_p1: usize,
    answer_p2: usize,
}
impl TestData {
    pub fn new(input: Vec<String>, answer: (usize, usize)) -> Self {
        Self {
            input: input,
            answer_p1: answer.0,
            answer_p2: answer.1,
        }
    }
}

fn get_test_data(day: usize) -> TestData {
    let path = format!("./resources/day{}_test.txt", day);
    let lines = match read_file(path) {
        Err(_) => panic!("Couldnt find test input for day {}", day),
        Ok(lines) => lines,
    };

    let answer: (usize, usize) = {
        let (p1, p2) = lines[0].split_once(" ").unwrap();

        (p1.parse::<usize>().unwrap(), p2.parse::<usize>().unwrap())
    };

    let input: Vec<String> = lines.iter().skip(2).map(|e| e.to_string()).collect();

    TestData::new(input, answer)
}

mod tests {
    #[test]
    fn parse_test_data_test() {
        let test_data = super::get_test_data(1);
        assert_eq!(7, test_data.answer_p1);
        assert_eq!(5, test_data.answer_p2);
        assert_eq!(10, test_data.input.len());
    }
}
