use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct LanternFish(u8);

impl Default for LanternFish {
    fn default() -> LanternFish {
        LanternFish(8)
    }
}

impl LanternFish {
    fn next_day(&mut self) -> Option<LanternFish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(LanternFish::default())
        } else {
            self.0 -= 1;
            None
        }
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

    let mut fish_vec: Vec<LanternFish> = line
        .split(',')
        .map(|v| v.parse().unwrap())
        .map(LanternFish)
        .collect();

    for i in 0..n_days {
        let new_fishes: Vec<LanternFish> =
            fish_vec.iter_mut().filter_map(|f| f.next_day()).collect();
        fish_vec.extend(new_fishes);
        println!("Number of fishes after {} days: {}", i, fish_vec.len());
    }
}
