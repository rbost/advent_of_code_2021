use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename_string = &args[1];
    let filename_transform = &args[2];
    // Open the file in read-only mode (ignoring errors).
    let file_string = File::open(filename_string).unwrap();
    let file_transform = File::open(filename_transform).unwrap();
    let reader_string = BufReader::new(file_string);
    let reader_transform = BufReader::new(file_transform);

    let mut char_vec: Vec<char> = reader_string
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();

    let transforms: HashMap<(char, char), char> = reader_transform
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut line_iter = l.chars();
            let c1 = line_iter.next().unwrap();
            let c2 = line_iter.next().unwrap();
            line_iter.next().unwrap(); // space
            line_iter.next().unwrap(); // -
            line_iter.next().unwrap(); // >
            line_iter.next().unwrap(); // space
            let cend = line_iter.next().unwrap();

            ((c1, c2), cend)
        })
        .collect();

    const N_STEPS: usize = 40;

    for _ in 0..N_STEPS {
        let insertions = char_vec
            .iter()
            .tuple_windows()
            .map(|(c1, c2)| transforms.get(&(*c1, *c2)).unwrap());
        char_vec = char_vec
            .iter()
            .interleave(insertions)
            .copied()
            .collect::<Vec<_>>();
    }

    let counts = char_vec.iter().counts();
    let (min, max) = counts
        .iter()
        .minmax_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .into_option()
        .unwrap();

    println!("Min/Max {:?}, {:?}", min, max);
    println!("Value {}", max.1 - min.1);
}
