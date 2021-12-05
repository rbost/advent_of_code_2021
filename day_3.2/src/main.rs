use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagnosticBit {
    Zero,
    One,
}
impl Default for DiagnosticBit {
    fn default() -> Self {
        DiagnosticBit::Zero
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiagnosticLine<const N: usize>([DiagnosticBit; N]);

impl<const N: usize> From<DiagnosticLine<N>> for u64 {
    fn from(line: DiagnosticLine<N>) -> u64 {
        line.0.into_iter().fold(0u64, |acc, bit| match bit {
            DiagnosticBit::Zero => 2 * acc,
            DiagnosticBit::One => 2 * acc + 1,
        })
    }
}

impl<const N: usize> Default for DiagnosticLine<N> {
    fn default() -> Self {
        DiagnosticLine::<N>([DiagnosticBit::Zero; N])
    }
}

struct DiagnosticAccumulator<const N: usize>([usize; N]);

impl<const N: usize> Default for DiagnosticAccumulator<N> {
    fn default() -> Self {
        DiagnosticAccumulator::<N>([0usize; N])
    }
}

impl<const N: usize> Add<DiagnosticLine<N>> for DiagnosticAccumulator<N> {
    type Output = Self;

    fn add(self, other: DiagnosticLine<N>) -> Self {
        let mut r = self;

        r.0.iter_mut()
            .zip(other.0.iter())
            .for_each(|(acc_cell, bit)| {
                if *bit == DiagnosticBit::One {
                    *acc_cell += 1;
                }
            });

        r
    }
}

impl<const N: usize> FromStr for DiagnosticLine<N> {
    type Err = <[DiagnosticBit; N] as TryFrom<Vec<DiagnosticBit>>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elts = s.trim().chars().map(|c| {
            if c == '0' {
                DiagnosticBit::Zero
            } else {
                DiagnosticBit::One
            }
        });
        let r = DiagnosticLine::<N>(elts.collect::<Vec<DiagnosticBit>>().try_into()?);
        Ok(r)
    }
}

fn bit_partition<const N: usize>(
    lines: Vec<DiagnosticLine<N>>,
    bit_pos: usize,
) -> (Vec<DiagnosticLine<N>>, Vec<DiagnosticLine<N>>) {
    // the first element for the ones  the second for the zeros

    lines
        .into_iter()
        .partition(|line| line.0[bit_pos] == DiagnosticBit::One)
}

fn get_oxygen_rating<const N: usize>(lines: Vec<DiagnosticLine<N>>) -> DiagnosticLine<N> {
    let mut lines = lines;
    for i in 0..N {
        let (ones, zeroes) = bit_partition(lines, i);

        lines = if ones.len() >= zeroes.len() {
            // if 0 and 1 are equally common, oxygen gets the values with 1
            ones
        } else {
            zeroes
        };

        if lines.len() == 1 {
            break;
        }
    }
    assert!(!lines.is_empty());
    assert!(lines.iter().all(|&line| line == lines[0]));
    lines[0]
}

fn get_co2_rating<const N: usize>(lines: Vec<DiagnosticLine<N>>) -> DiagnosticLine<N> {
    let mut lines = lines;
    for i in 0..N {
        let (ones, zeroes) = bit_partition(lines, i);

        lines = if ones.len() >= zeroes.len() {
            // if 0 and 1 are equally common, co2 gets the values with 0
            zeroes
        } else {
            ones
        };
        if lines.len() == 1 {
            break;
        }
    }
    assert!(!lines.is_empty());
    assert!(lines.iter().all(|&line| line == lines[0]));
    lines[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const DIAG_SIZE: usize = 12;
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<DiagnosticLine<DIAG_SIZE>> = reader
        .lines()
        .map(|line| line.unwrap().parse::<DiagnosticLine<DIAG_SIZE>>().unwrap())
        .collect();

    let ox_line = get_oxygen_rating(lines.clone());
    let ox_rating: u64 = ox_line.into();
    println!("Ox: {}", ox_rating);

    let co2_line = get_co2_rating(lines);
    let co2_rating: u64 = co2_line.into();
    println!("Co2: {}", co2_rating);

    println!("Life support: {}", ox_rating * co2_rating);
}
