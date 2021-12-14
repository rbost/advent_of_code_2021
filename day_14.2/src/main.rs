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

    let polymer_string = reader_string.lines().next().unwrap().unwrap();
    let mut char_couples: HashMap<(char, char), usize> =
        polymer_string.chars().tuple_windows().counts();

    let mut char_counts = polymer_string.chars().counts();

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
        let insertions: Vec<((char, char), usize)> = char_couples
            .iter()
            .flat_map(|(couple, count)| {
                let insert = transforms.get(couple).unwrap();

                match char_counts.get_mut(insert) {
                    Some(v) => *v += count,
                    None => {
                        char_counts.insert(*insert, *count);
                    }
                }

                let new_couple_1 = (couple.0, *insert);
                let new_couple_2 = (*insert, couple.1);

                let inserted_couple = vec![(new_couple_1, *count), (new_couple_2, *count)];
                inserted_couple
            })
            .collect();

        char_couples = HashMap::new();

        insertions.iter().for_each(|(c, v)| {
            let count = char_couples.get_mut(c);
            match count {
                Some(count) => *count += v,
                None => {
                    char_couples.insert(*c, *v);
                }
            }
        });
    }

    let (min, max) = char_counts
        .iter()
        .minmax_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .into_option()
        .unwrap();

    println!("Min/Max {:?}, {:?}", min, max);
    println!("Value {}", max.1 - min.1);
}
