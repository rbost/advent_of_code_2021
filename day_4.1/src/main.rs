use ansi_term::Colour::Red;
use itertools::Itertools;
use std::{
    env, fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BingoGridNumber {
    Marked(u32),
    Unmarked(u32),
}

impl BingoGridNumber {
    fn mark(&mut self, number: u32) -> bool {
        match *self {
            BingoGridNumber::Unmarked(v) if v == number => {
                *self = BingoGridNumber::Marked(v);
                true
            }
            _ => false,
        }
    }

    fn is_marked(&self) -> bool {
        matches!(*self, BingoGridNumber::Marked(_))
    }
}

#[derive(Clone, PartialEq, Eq)]
struct BingoGrid<const N: usize>([[BingoGridNumber; N]; N]);

impl<const N: usize> BingoGrid<N> {
    fn mark(&mut self, number: u32) -> Option<(usize, usize)> {
        self.0
            .iter_mut()
            .map(|line| line.iter_mut().position(|val| val.mark(number)))
            .enumerate()
            .find_map(|(line, pos)| pos.map(|p| (line, p)))
    }

    fn check_complete(&self, marked_pos: (usize, usize)) -> bool {
        let completed_col = self.0.iter().all(|line| line[marked_pos.1].is_marked());

        let completed_line = self.0[marked_pos.0].iter().all(|v| v.is_marked());

        completed_col || completed_line
    }

    fn compute_grid_score(&self) -> u32 {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .map(|v| match *v {
                        BingoGridNumber::Unmarked(v) => v,
                        _ => 0,
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

impl<const N: usize> fmt::Display for BingoGrid<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|grid_line| {
            grid_line.iter().try_for_each(|v| match *v {
                BingoGridNumber::Marked(v) => write!(f, "{:>2} ", Red.paint(v.to_string())),
                BingoGridNumber::Unmarked(v) => write!(f, "{:>2} ", v),
            })?;
            writeln!(f)
        })
    }
}

type StringTuple5 = (String, String, String, String, String);
impl TryFrom<StringTuple5> for BingoGrid<5> {
    type Error = <[BingoGridNumber; 5] as TryFrom<Vec<BingoGridNumber>>>::Error;

    fn try_from(string_lines: StringTuple5) -> Result<Self, Self::Error> {
        let line_0 = string_lines
            .0
            .trim()
            .split_whitespace()
            .map(|s| BingoGridNumber::Unmarked(s.parse().unwrap()))
            .collect::<Vec<BingoGridNumber>>()
            .try_into()?;
        let line_1 = string_lines
            .1
            .trim()
            .split_whitespace()
            .map(|s| BingoGridNumber::Unmarked(s.parse().unwrap()))
            .collect::<Vec<BingoGridNumber>>()
            .try_into()?;
        let line_2 = string_lines
            .2
            .trim()
            .split_whitespace()
            .map(|s| BingoGridNumber::Unmarked(s.parse().unwrap()))
            .collect::<Vec<BingoGridNumber>>()
            .try_into()?;
        let line_3 = string_lines
            .3
            .trim()
            .split_whitespace()
            .map(|s| BingoGridNumber::Unmarked(s.parse().unwrap()))
            .collect::<Vec<BingoGridNumber>>()
            .try_into()?;
        let line_4 = string_lines
            .4
            .trim()
            .split_whitespace()
            .map(|s| BingoGridNumber::Unmarked(s.parse().unwrap()))
            .collect::<Vec<BingoGridNumber>>()
            .try_into()?;

        Ok(BingoGrid::<5>([line_0, line_1, line_2, line_3, line_4]))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const GRID_SIZE: usize = 5;
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines_iterator = reader.lines();
    let numbers_line = lines_iterator.next().unwrap().unwrap();

    let mut grids = Vec::<BingoGrid<GRID_SIZE>>::new();

    while let Some(_) = lines_iterator.next() {
        if let Some(grid_lines) = lines_iterator.next_tuple().map(|(r0, r1, r2, r3, r4)| {
            (
                r0.unwrap(),
                r1.unwrap(),
                r2.unwrap(),
                r3.unwrap(),
                r4.unwrap(),
            )
        }) {
            let grid: BingoGrid<GRID_SIZE> = grid_lines.try_into().unwrap();
            grids.push(grid);
        } else {
            break;
        }
    }

    let score = numbers_line
        .split(',')
        .map(|s| s.parse().unwrap())
        .find_map(|number| {
            // check if there is a completed grid after marking number
            let completed_grid_index = grids.iter_mut().position(|grid| {
                grid.mark(number)
                    .map_or(false, |pos| grid.check_complete(pos))
            });

            // compute the score if a grid is complete
            completed_grid_index.map(|grid_index| grids[grid_index].compute_grid_score() * number)
        });

    println!("Score {}", score.unwrap());
}
