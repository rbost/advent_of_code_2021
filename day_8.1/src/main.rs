use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let digit_count = reader
        .lines()
        .map(|line| {
            let mut line = line.unwrap();
            let delimiter_pos = line.find('|').unwrap();
            let mut suffix = line.split_off(delimiter_pos);
            let bar = suffix.remove(0);
            assert_eq!(bar, '|');
            suffix
                .trim()
                .split_whitespace()
                .map(|subs| subs.len())
                .filter_map(parse_len)
                .count()
        })
        .sum::<usize>();

    println!("Found {} digits", digit_count);
}
