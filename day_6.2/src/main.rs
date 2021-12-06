use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LanternFish(u8);

impl Default for LanternFish {
    fn default() -> LanternFish {
        LanternFish(8)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let n_days = 256;
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // this approach no longer works because the work to do is exponential
    // let mut fish_vec: Vec<LanternFish> = line
    //     .split(',')
    //     .map(|v| v.parse().unwrap())
    //     .map(LanternFish)
    //     .collect();

    // for i in 0..n_days {
    //     let new_fishes: Vec<LanternFish> =
    //         fish_vec.iter_mut().filter_map(|f| f.next_day()).collect();
    //     fish_vec.extend(new_fishes);
    //     println!("Number of fishes after {} days: {}", i, fish_vec.len());
    // }

    let mut fish_vec: Vec<LanternFish> = line
        .split(',')
        .map(|v| v.parse().unwrap())
        .map(LanternFish)
        .collect();

    fish_vec.sort();

    let mut fish_per_timer = [0u64; 9];

    fish_vec
        .into_iter()
        .dedup_with_count()
        .for_each(|(count, val)| fish_per_timer[val.0 as usize] = count as u64);

    for i in 0..n_days {
        let mut new_state = [0u64; 9];

        new_state[..8].clone_from_slice(&fish_per_timer[1..9]);
        new_state[8] = fish_per_timer[0];
        new_state[6] += fish_per_timer[0];

        fish_per_timer = new_state;

        println!(
            "Number of fishes after {} days: {}",
            i + 1,
            fish_per_timer.iter().sum::<u64>()
        );
    }
}
