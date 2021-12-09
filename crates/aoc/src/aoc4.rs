use crate::runner::Runner;

#[derive(Default)]
pub struct AOC4 {
    parsed: Bingo,
}

#[derive(Default, Debug, Clone, Copy)]
struct Board {
    data: [u8; 25],
    marks: [bool; 25],
}

impl Board {
    fn check_num(&mut self, num: u8) {
        let idx = self.data.iter().position(|&e| e == num);
        if let Some(idx) = idx {
            self.marks[idx] = true;
        }
    }

    fn check_bingo(&self) -> Option<&Self> {
        for y in 0..5 {
            let mut sum_h = 0;
            for x in 0..5 {
                let idx = y * 5 + x;
                if self.marks[idx] {
                    sum_h += 1;
                }
            }
            if sum_h == 5 {
                return Some(self);
            }
        }

        for i in 0..5 {
            let mut sum_v = 0;
            for j in 0..5 {
                let idx_v = i + j * 5;
                if self.marks[idx_v] {
                    sum_v += 1;
                }
            }
            if sum_v == 5 {
                return Some(self);
            }
        }

        None
    }

    fn sum_unmarked(&self) -> usize {
        self.data.iter().enumerate().fold(0usize, |acc, e| {
            if !self.marks[e.0] {
                acc + *e.1 as usize
            } else {
                acc
            }
        })
    }
}

#[derive(Default, Debug, Clone)]
struct Bingo {
    draws: Vec<u8>,
    boards: Vec<Board>,
}

impl Runner for AOC4 {
    fn parse(&mut self, input: &Vec<String>) {
        let draws: Vec<u8> = input[0]
            .split_terminator(",")
            .map(|e| e.parse::<u8>().unwrap())
            .collect();

        let mut boards: Vec<Board> = Vec::new();
        for i in 0..((input.len() - 1) / 6) {
            let mut data = [0u8; 25];
            for j in 1..6 {
                for (idx, v) in input[i * 6 + j + 1]
                    .split_whitespace()
                    .map(|e| e.parse::<u8>().unwrap())
                    .enumerate()
                {
                    data[idx + (j - 1) * 5] = v;
                }
            }
            boards.push(Board {
                data: data,
                marks: [false; 25],
            });
        }

        self.parsed = Bingo {
            draws: draws,
            boards: boards,
        };
    }

    fn run_p1(&self) -> usize {
        let mut bingo = self.parsed.clone();

        let mut last_draw = 0u8;
        let mut winner: Option<Board> = None;
        for draw in &bingo.draws {
            last_draw = *draw;

            if let Some(Some(w)) = bingo
                .boards
                .iter_mut()
                .map(|e| {
                    e.check_num(last_draw);
                    e.check_bingo()
                })
                .filter(Option::is_some)
                .nth(0)
            {
                winner = Some(*w);
                break;
            }
        }

        if let Some(winner) = winner {
            winner.sum_unmarked() * last_draw as usize
        } else {
            unreachable!()
        }
    }

    fn run_p2(&self) -> usize {
        let mut bingo = self.parsed.clone();

        let mut last_draw = 0u8;
        let mut winner: Option<Board> = None;
        for draw in &bingo.draws {
            last_draw = *draw;

            for i in (0..bingo.boards.len()).rev() {
                let board = &mut bingo.boards[i];
                board.check_num(last_draw);
                if let Some(_) = board.check_bingo() {
                    winner = Some(bingo.boards.remove(i));
                }
            }

            if bingo.boards.len() == 0 {
                break;
            }
        }

        if let Some(winner) = winner {
            winner.sum_unmarked() * last_draw as usize
        } else {
            unreachable!()
        }
    }
}

mod tests {
    /* bingo.boards[0].check_num(22);
    bingo.boards[0].check_num(13);
    bingo.boards[0].check_num(17);
    bingo.boards[0].check_num(11);
    bingo.boards[0].check_num(0);
    println!("{:?}", bingo.boards[0].check_bingo());
    return 0; */

    /* bingo.boards[0].check_num(22);
    bingo.boards[0].check_num(8);
    bingo.boards[0].check_num(21);
    bingo.boards[0].check_num(6);
    bingo.boards[0].check_num(1);
    println!("{:?}", bingo.boards[0].check_bingo());
    return 0; */
}
