use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let score = reader
        .lines()
        .map(|l| {
            let mut queue = VecDeque::new();
            l.unwrap()
                .chars()
                .find_map(|c| match c {
                    '(' | '<' | '[' | '{' => {
                        queue.push_back(c);
                        None
                    }
                    ')' => {
                        if let Some(d) = queue.pop_back() {
                            if d == '(' {
                                return None;
                            }
                        }
                        Some(3)
                    }
                    '>' => {
                        if let Some(d) = queue.pop_back() {
                            if d == '<' {
                                return None;
                            }
                        }
                        Some(25137)
                    }
                    ']' => {
                        if let Some(d) = queue.pop_back() {
                            if d == '[' {
                                return None;
                            }
                        }
                        Some(57)
                    }
                    '}' => {
                        if let Some(d) = queue.pop_back() {
                            if d == '{' {
                                return None;
                            }
                        }
                        Some(1197)
                    }
                    _ => unimplemented!(),
                })
                .unwrap_or(0u64)
        })
        .sum::<u64>();

    println!("Score: {}", score);
}
