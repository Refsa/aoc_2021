use std::collections::HashMap;

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
        let mut board = Board::new(self.p1_start, self.p2_start, |roll| (roll % 100) + 1);

        loop {
            if let Some(loser) = board.do_turn() {
                return board.players[loser].1 as usize * board.rolls as usize;
            }
        }
    }

    fn run_p2(&self) -> usize {
        let mut lookup = HashMap::new();
        let board = QuantumBoard::new(self.p1_start, self.p2_start);

        let sum = sim_board(board, 0, &mut lookup);
        sum[0].max(sum[1])
    }
}

const _BELL: [u128; 7] = [1, 3, 6, 7, 6, 3, 1];
type LookupKey = [u16; 2];

fn sim_board(
    mut board: QuantumBoard,
    roll: u16,
    lookup: &mut HashMap<LookupKey, [usize; 2]>,
) -> [usize; 2] {
    if roll > 0 {
        if let Some(turn) = board.do_roll(roll) {
            return match turn {
                0 => [0, 1],
                1 => [1, 0],
                _ => unreachable!(),
            };
        }
    }

    if let Some(state) = lookup.get(&board.key()) {
        return state.clone();
    }

    let score = vec![
        (sim_board(board.clone(), 3, lookup), 1),
        (sim_board(board.clone(), 9, lookup), 1),
        (sim_board(board.clone(), 4, lookup), 3),
        (sim_board(board.clone(), 8, lookup), 3),
        (sim_board(board.clone(), 5, lookup), 6),
        (sim_board(board.clone(), 7, lookup), 6),
        (sim_board(board.clone(), 6, lookup), 7),
    ]
    .into_iter()
    .fold([0, 0], |mut acc, (e, i)| {
        acc[0] += e[0] * i;
        acc[1] += e[1] * i;
        acc
    });

    lookup.insert(board.key(), score);

    score
}

#[derive(Clone)]
struct QuantumBoard {
    players: [(u16, u16); 2],
    turn: u8,
}

impl QuantumBoard {
    fn new(p1: usize, p2: usize) -> Self {
        QuantumBoard {
            players: [(p1 as u16, 0), (p2 as u16, 0)],
            turn: 0,
        }
    }

    fn do_roll(&mut self, roll: u16) -> Option<usize> {
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

        p.1 += p.0;
        p.1 >= 21
    }

    fn key(&self) -> LookupKey {
        [
            self.turn as u16,
            self.players[0].1 * 10 * 21 * 10
                + self.players[0].0 * 21 * 10
                + self.players[1].1 * 10
                + self.players[1].0,
        ]
    }
}

struct Board {
    rolls: u16,
    players: [(u16, u16); 2],
    turn: u8,
    dice: fn(u16) -> u16,
}

impl Board {
    fn new(p1: usize, p2: usize, dice: fn(u16) -> u16) -> Self {
        Board {
            rolls: 0,
            players: [(p1 as u16, 0), (p2 as u16, 0)],
            dice: dice,
            turn: 0,
        }
    }

    fn do_turn(&mut self) -> Option<usize> {
        let rolls =
            (self.dice)(self.rolls) + (self.dice)(self.rolls + 1) + (self.dice)(self.rolls + 2);
        self.rolls += 3;

        let status = self.move_player(self.turn, rolls);

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

        p.1 += p.0;
        p.1 >= 1000
    }
}
