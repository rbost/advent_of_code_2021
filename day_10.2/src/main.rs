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

    let mut scores: Vec<u64> = reader
        .lines()
        .filter_map(|l| {
            let mut queue = VecDeque::new();
            let line_score = l.unwrap().chars().find_map(|c| match c {
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
            });
            if line_score.is_none() {
                Some(queue)
            } else {
                None
            }
        })
        .map(|queue| {
            queue
                .into_iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '<' => 4,
                    '[' => 2,
                    '{' => 3,
                    _ => unimplemented!(),
                })
                .fold(0, |acc, v| acc * 5 + v)
        })
        .collect();

    scores.sort_unstable();
    println!("Middle score: {}", scores[scores.len() / 2]);
}
