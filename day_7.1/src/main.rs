use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn compute_fuel_cost(crab_pos: &Vec<u32>, target_pos: u32) -> u32 {
    crab_pos
        .iter()
        .map(|&p| (p as i32 - target_pos as i32).abs() as u32)
        .sum()
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

    let positions: Vec<u32> = line.split(',').map(|v| v.parse().unwrap()).collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let (opt_p, min_f) = (min..=max)
        .map(|target| (target, compute_fuel_cost(&positions, target)))
        .min_by(|(_, f1), (_, f2)| f1.cmp(f2))
        .unwrap();

    println!("Optimal position {}, fuel {}", opt_p, min_f);
}
