use crate::runner::Runner;

mod runner;
mod aoc1;

fn get_runner(day: usize) -> impl Runner {
    match day {
        1 => aoc1::AOC1{},
        _ => panic!("Runner for day {} not implemented", day),
    }
}

fn main() {
    let day = 1;

    let runner = get_runner(day);

    let test_data = get_test_data(day);
    let test_answer = runner.test(&test_data.input);

    if test_answer != test_data.answer {
        println!("Failed test: expected {} - got answer {}", test_data.answer, test_answer);
        return;
    }
    println!("Test success");

    let input = get_input(day);
    println!("Part 1: {}", runner.run_p1(&input));
    println!("Part 2: {}", runner.run_p2(&input));
}

fn get_input(day: usize) -> Vec<String> {
    let path = format!("./resources/day{}.txt", day);
    let content = std::fs::read_to_string(path).unwrap();
    return content.lines().map(|e| e.to_string()).collect();
}

struct TestData {
    input: Vec<String>,
    answer: usize,
}
impl TestData {
    pub fn new(input: Vec<String>, answer: usize) -> Self {
        Self {
            input: input,
            answer: answer
        }
    }
}

fn get_test_data(day: usize) -> TestData {
    let path = format!("./resources/day{}_test.txt", day);
    let content = std::fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    
    let answer: usize = lines[0].parse::<usize>().unwrap();
    let input: Vec<String> = lines.iter().skip(2).map(|e| e.to_string()).collect();

    TestData::new(input, answer)
}

mod tests {
    #[test]
    fn parse_test_data_test() {
        let test_data = super::get_test_data(1);
        assert_eq!(7, test_data.answer);
        assert_eq!(10, test_data.input.len());
    }
}