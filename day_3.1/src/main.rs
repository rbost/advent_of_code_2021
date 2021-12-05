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

struct DiagnosticLine<const N: usize>([DiagnosticBit; N]);

impl<const N: usize> DiagnosticLine<N> {
    fn invert(&self) -> DiagnosticLine<N> {
        let mut line = DiagnosticLine::default();

        for i in 0..N {
            match self.0[i] {
                DiagnosticBit::Zero => line.0[i] = DiagnosticBit::One,
                DiagnosticBit::One => line.0[i] = DiagnosticBit::Zero,
            }
        }
        line
    }
}

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

impl<const N: usize> DiagnosticAccumulator<N> {
    fn get_summary_line(&self, line_count: usize) -> DiagnosticLine<N> {
        let mut summary_line = DiagnosticLine::default();

        for i in 0..N {
            if 2 * self.0[i] > line_count {
                // might overflow ...
                summary_line.0[i] = DiagnosticBit::One;
            }
        }
        summary_line
    }
}
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

fn main() {
    let args: Vec<String> = env::args().collect();
    const DIAG_SIZE: usize = 12usize;
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let (acc, line_count) = reader
        .lines()
        .map(|line| line.unwrap().parse::<DiagnosticLine<DIAG_SIZE>>().unwrap())
        .fold(
            (DiagnosticAccumulator::<DIAG_SIZE>::default(), 0usize),
            |(acc, count), line| (acc + line, count + 1),
        );

    let summary = acc.get_summary_line(line_count);
    let inv_summary = summary.invert();
    let gamma: u64 = summary.into();
    let epsilon: u64 = inv_summary.into();

    let power = gamma * epsilon;

    println!("Power: {}", power);
}
