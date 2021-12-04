use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .fold((0u32, None), |(mut count, prev), val| match prev {
            None => (count, Some(val)),
            Some(prev) => {
                if prev < val {
                    count += 1u32;
                };
                (count, Some(val))
            }
        })
        .0;

    println!("Increases count: {}", count);
}
