use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

fn parse_len(len: usize) -> Option<u8> {
    match len {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn get_1_bars(s: &str) -> (char, char) {
    let mut s = s.chars();
    (s.next().unwrap(), s.next().unwrap())
}
fn get_4_bars(s: &str) -> (char, char, char, char) {
    let mut s = s.chars();
    (
        s.next().unwrap(),
        s.next().unwrap(),
        s.next().unwrap(),
        s.next().unwrap(),
    )
}
fn get_7_bars(s: &str) -> (char, char, char) {
    let mut s = s.chars();
    (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
}

fn contains_1_bars(s: &str, chars: (char, char)) -> bool {
    s.contains(chars.0) && s.contains(chars.1)
}

fn compute_4_partials(
    bars_4: (char, char, char, char),
    bars_7: (char, char, char),
) -> (char, char) {
    let c7 = [bars_7.0, bars_7.1, bars_7.2];
    let c4 = [bars_4.0, bars_4.1, bars_4.2, bars_4.3];

    let c4: Vec<_> = c4.iter().filter(|c| !c7.contains(c)).collect();
    assert_eq!(c4.len(), 2);
    (*c4[0], *c4[1])
}

fn contains_4_bars_partial(s: &str, chars: (char, char)) -> bool {
    s.contains(chars.0) && s.contains(chars.1)
}

struct TruthTable {
    len: usize,
    contains_1: bool,
    contains_4_partials: bool,
}

impl TruthTable {
    fn find_digit(&self) -> u8 {
        match self.len {
            5 => {
                // for 2, 3 and 5
                if self.contains_4_partials {
                    5
                } else if self.contains_1 {
                    3
                } else {
                    2
                }
            }
            6 => {
                // for 0,6 and 9
                if !self.contains_1 {
                    6
                } else if self.contains_4_partials {
                    9
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();

    String::from_iter(chars)
}

fn compute_digit_map(prefix: &str) -> [Option<String>; 10] {
    let digits: Vec<(&str, usize)> = prefix
        .trim()
        .split_whitespace()
        .map(|s| (s, s.len()))
        .collect();

    let mut digit_map = [None, None, None, None, None, None, None, None, None, None];

    let digits: Vec<(&str, usize)> = digits
        .into_iter()
        .filter_map(|(s, len)| match parse_len(len) {
            Some(v) => {
                digit_map[v as usize] = Some(sort_string(s));
                None
            } // filter out the digits that are easy to find
            None => Some((s, len)),
        })
        .collect();

    let tuple_1_bars = get_1_bars(digit_map[1].as_ref().unwrap());
    let tuple_4_bars = get_4_bars(digit_map[4].as_ref().unwrap());
    let tuple_7_bars = get_7_bars(digit_map[7].as_ref().unwrap());
    let tuple_4_partial = compute_4_partials(tuple_4_bars, tuple_7_bars);

    digits
        .iter()
        .map(|&(s, len)| {
            (
                s,
                TruthTable {
                    len,
                    contains_1: contains_1_bars(s, tuple_1_bars),
                    contains_4_partials: contains_4_bars_partial(s, tuple_4_partial),
                    // contains_7: contains_7_bars(s, tuple_7_bars),
                },
            )
        })
        .map(|(s, tt)| (s, tt.find_digit()))
        .for_each(|(s, v)| digit_map[v as usize] = Some(sort_string(s)));

    digit_map
}
fn process_line(line: String) -> u64 {
    let delimiter_pos = line.find('|').unwrap();
    let mut prefix = line;
    let mut suffix = prefix.split_off(delimiter_pos);
    let bar = suffix.remove(0);
    assert_eq!(bar, '|');

    let digit_map = compute_digit_map(&prefix);

    suffix
        .trim()
        .split_whitespace()
        .map(|s| sort_string(s))
        .map(|s| {
            digit_map
                .iter()
                .position(|v| Some(&s).eq(&v.as_ref()))
                .unwrap() as u8
        })
        .fold(0u64, |acc, v| acc * 10 + (v as u64))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let res: u64 = reader.lines().map(|l| process_line(l.unwrap())).sum();

    println!("Result: {}", res);
}
