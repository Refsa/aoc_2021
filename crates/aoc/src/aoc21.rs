use crate::runner::Runner;

#[derive(Default)]
pub struct AOC21 {
    p1_start: usize,
    p2_start: usize,
}

impl Runner for AOC21 {
    fn parse(&mut self, input: &Vec<String>) {
        self.p1_start = (input[0].bytes().last().unwrap() - 48) as usize;
        self.p2_start = (input[1].bytes().last().unwrap() - 48) as usize;
    }

    fn run_p1(&self) -> usize {
        let mut board = Board::new(
            self.p1_start,
            self.p2_start,
            1000,
            Some(|roll| (roll % 100) + 1),
        );

        loop {
            if let Some(loser) = board.do_turn() {
                return board.players[loser].1 as usize * board.rolls as usize;
            }
        }

        0
    }

    fn run_p2(&self) -> usize {
        let mut steps = 0;

        let board = Board::new(self.p1_start, self.p2_start, 21, None);

        let sum = sim_board(board, 0, &mut steps);
        println!("{}, {}", sum[0], sum[1]);
        sum[0].max(sum[1])
    }
}

const BELL: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn sim_board(mut board: Board, roll: u16, steps: &mut usize) -> [usize; 2] {
    if roll > 0 {
        if let Some(turn) = board.do_roll(roll) {
            *steps += 1;
            if *steps % 100_000_000 == 0 {
                println!("{}", steps);
            }

            return match turn {
                0 => [0, BELL[roll as usize - 3]],
                1 => [BELL[roll as usize - 3], 0],
                _ => unreachable!(),
            };
        }
    }

    let rolls = [
        sim_board(board.clone(), 3, steps),
        sim_board(board.clone(), 4, steps),
        sim_board(board.clone(), 5, steps),
        sim_board(board.clone(), 6, steps),
        sim_board(board.clone(), 7, steps),
        sim_board(board.clone(), 8, steps),
        sim_board(board.clone(), 9, steps),
    ];

    rolls
        .iter()
        .enumerate()
        .fold([0, 0], |acc, (i, e)| [acc[0] + e[0], acc[1] + e[1]])
}

#[derive(Clone)]
struct Board {
    board: [u8; 10],
    rolls: u16,
    players: [(u16, u16); 2],
    turn: u8,
    dice: Option<fn(u16) -> u16>,
    end_score: u16,
}

impl Board {
    fn new(p1: usize, p2: usize, end_score: u16, dice: Option<fn(u16) -> u16>) -> Self {
        let mut board = [0u8; 10];
        for i in 1..11 {
            board[i - 1] = i as u8;
        }

        Board {
            board: board,
            rolls: 0,
            players: [(p1 as u16, 0), (p2 as u16, 0)],
            dice: dice,
            turn: 0,
            end_score: end_score,
        }
    }

    fn do_turn(&mut self) -> Option<usize> {
        let dice = self.dice.unwrap();
        let rolls = (dice)(self.rolls) + (dice)(self.rolls + 1) + (dice)(self.rolls + 2);
        self.rolls += 3;

        let status = self.move_player(self.turn, rolls);

        self.turn = (self.turn + 1) % 2;
        if status {
            Some(self.turn as usize)
        } else {
            None
        }
    }

    fn do_roll(&mut self, roll: u16) -> Option<usize> {
        self.rolls += 3;

        let status = self.move_player(self.turn, roll);

        self.turn = (self.turn + 1) % 2;
        if status {
            Some(self.turn as usize)
        } else {
            None
        }
    }

    fn move_player(&mut self, player: u8, by: u16) -> bool {
        let p = &mut self.players[player as usize];
        p.0 += by;
        p.0 = (p.0 - 1) % 10 + 1;

        p.1 += self.board[p.0 as usize - 1] as u16;

        p.1 >= self.end_score
    }
}