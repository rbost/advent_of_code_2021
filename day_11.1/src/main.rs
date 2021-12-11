use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

enum Octopus {
    Flashed(),
    Idle(u8),
}

impl Octopus {
    fn reset(&mut self) {
        *self = Octopus::Idle(0)
    }
    fn will_flash(&self) -> bool {
        matches!(self, Octopus::Idle(v) if *v > 9)
    }
    fn increment(&mut self) {
        if let Octopus::Idle(v) = self {
            *v += 1
        }
    }
}

fn increment_all(grid: &mut Vec<Vec<Octopus>>) {
    grid.iter_mut()
        .for_each(|v| v.iter_mut().for_each(|o| o.increment()));
}
fn reset_all_flashed(grid: &mut Vec<Vec<Octopus>>) -> usize {
    grid.iter_mut()
        .map(|v| {
            v.iter_mut()
                .map(|o| match o {
                    Octopus::Flashed() => {
                        o.reset();
                        1
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn increment_surroundings(grid: &mut Vec<Vec<Octopus>>, (x, y): (usize, usize)) {
    if x > 0 {
        if y > 0 {
            grid[y - 1][x - 1].increment();
        }
        grid[y][x - 1].increment();
        if y < grid.len() - 1 {
            grid[y + 1][x - 1].increment();
        }
    }
    if y > 0 {
        grid[y - 1][x].increment();
    }
    if y < grid.len() - 1 {
        grid[y + 1][x].increment();
    }
    if x < grid[y].len() - 1 {
        if y > 0 {
            grid[y - 1][x + 1].increment();
        }
        grid[y][x + 1].increment();
        if y < grid.len() - 1 {
            grid[y + 1][x + 1].increment();
        }
    }
}

fn get_flashing(grid: &Vec<Vec<Octopus>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(x, o)| if o.will_flash() { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<Octopus>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| Octopus::Idle(c.to_digit(10).unwrap() as u8))
                .collect()
        })
        .collect();

    let mut n_flashes = 0usize;
    for _ in 0..100 {
        increment_all(&mut grid);

        loop {
            let flashing = get_flashing(&grid);
            if !flashing.is_empty() {
                flashing.iter().for_each(|&(x, y)| {
                    increment_surroundings(&mut grid, (x, y));
                    grid[y][x] = Octopus::Flashed();
                });
            } else {
                break;
            }
        }

        n_flashes += reset_all_flashed(&mut grid);
    }

    println!("Number of flashes: {}", n_flashes);
}
