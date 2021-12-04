use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Clone, Default)]
struct Location {
    pos: u32,
    depth: u32,
    aim: u32,
}

impl Location {
    fn apply(mut self, command: Command) -> Self {
        // let mut copy = self.clone();
        match command {
            Command::Forward(x) => {
                self.pos += x;
                self.depth += self.aim * x;
            }
            Command::Down(x) => self.aim += x,
            Command::Up(x) => self.aim = self.aim.saturating_sub(x),
        }
        self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let loc = reader
        .lines()
        .map(|line| match line.unwrap().split_once(' ').unwrap() {
            (s, v) if s == "forward" => Command::Forward(v.parse::<u32>().unwrap()),
            (s, v) if s == "down" => Command::Down(v.parse::<u32>().unwrap()),
            (s, v) if s == "up" => Command::Up(v.parse::<u32>().unwrap()),
            _ => unreachable!("Invalid input"),
        })
        .fold(Location::default(), |loc, command| loc.apply(command));

    println!("Result (pos x depth): {}", loc.pos * loc.depth);
}
