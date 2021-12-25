use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cucumber {
    None,
    East,
    South,
}

fn find_movable(grid: &[Vec<Cucumber>], herd: Cucumber) -> Vec<((usize, usize), (usize, usize))> {
    let height = grid.len();
    let width = grid[0].len();

    (0..height)
        .flat_map(|y| {
            (0..width)
                .filter(move |&x| grid[y][x] == herd)
                .filter_map(move |x| {
                    let (next_x, next_y) = match herd {
                        Cucumber::East => ((x + 1) % width, y),
                        Cucumber::South => (x, (y + 1) % height),
                        Cucumber::None => unimplemented!(),
                    };
                    if grid[next_y][next_x] == Cucumber::None {
                        Some(((x, y), (next_x, next_y)))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn move_cucumbers(
    grid: &mut Vec<Vec<Cucumber>>,
    cucumbers_pos: impl Iterator<Item = ((usize, usize), (usize, usize))>,
) {
    for ((original_x, original_y), (next_x, next_y)) in cucumbers_pos {
        grid[next_y][next_x] = grid[original_y][original_x];
        grid[original_y][original_x] = Cucumber::None;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<_>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '>' => Cucumber::East,
                    'v' => Cucumber::South,
                    _ => Cucumber::None,
                })
                .collect()
        })
        .collect();

    let mut direction = Cucumber::East;
    let mut consecutive_move_failures = 0;
    let mut n_steps = 0;

    loop {
        n_steps += 1;

        let movables = find_movable(&grid, direction);
        if movables.is_empty() {
            consecutive_move_failures += 1;
            if consecutive_move_failures >= 2 {
                break;
            }
        } else {
            move_cucumbers(&mut grid, movables.into_iter());
            consecutive_move_failures = 0;
        }
        direction = match direction {
            Cucumber::East => Cucumber::South,
            Cucumber::South => Cucumber::East,
            _ => unreachable!(),
        };
    }

    n_steps += 1;
    n_steps /= 2;

    println!("Number of steps: {}", n_steps);
}
