use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{Itertools, Position};

fn get_low_points(height_map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let candidates: Vec<Vec<_>> = height_map
        .iter()
        .map(|l| {
            l.iter()
                .tuple_windows()
                .enumerate()
                .with_position()
                .filter_map(|elt| match elt {
                    Position::Only(_) => unimplemented!(),
                    Position::Middle((pos, (v0, v1, v2))) => {
                        if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                    Position::First((pos, (v0, v1, v2))) => {
                        if v0 < v1 {
                            Some(pos)
                        } else if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                    Position::Last((pos, (v0, v1, v2))) => {
                        if v2 < v1 {
                            Some(pos + 2)
                        } else if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    candidates
        .iter()
        .enumerate()
        .with_position()
        .map(|elt| match elt {
            Position::First((y, cands)) => cands
                .iter()
                .filter(|x| height_map[y][**x as usize] < height_map[y + 1][**x as usize])
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Middle((y, cands)) => cands
                .iter()
                .filter(|x| {
                    height_map[y][**x as usize] < height_map[y + 1][**x as usize]
                        && height_map[y][**x as usize] < height_map[y - 1][**x as usize]
                })
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Last((y, cands)) => cands
                .iter()
                .filter(|x| height_map[y][**x as usize] < height_map[y - 1][**x as usize])
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Only(_) => todo!(),
        })
        .flatten()
        .collect_vec()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let height_map: Vec<Vec<u8>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let low_points = get_low_points(&height_map);

    let risk_level = low_points
        .iter()
        .map(|&(x, y)| (height_map[y][x] + 1) as u64)
        .sum::<u64>();

    println!("Risk: {}", risk_level);
}
