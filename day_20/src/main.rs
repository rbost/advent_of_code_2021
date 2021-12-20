use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn add_margins(grid: &mut Vec<Vec<bool>>, margin: usize) {
    // add the left and right margins
    grid.iter_mut().for_each(|l| {
        // left margin
        for _ in 0..margin {
            l.insert(0, false);
        }

        // right margin
        for _ in 0..margin {
            l.push(false);
        }
    });

    // add the top margin
    let width = grid[0].len();
    for _ in 0..margin {
        grid.insert(0, vec![false; width]);
    }

    // add the bottom margin
    for _ in 0..margin {
        grid.push(vec![false; width]);
    }
}

fn get_pos(l1: (bool, bool, bool), l2: (bool, bool, bool), l3: (bool, bool, bool)) -> usize {
    let v1 = (l1.0 as usize, l1.1 as usize, l1.2 as usize);
    let v2 = (l2.0 as usize, l2.1 as usize, l2.2 as usize);
    let v3 = (l3.0 as usize, l3.1 as usize, l3.2 as usize);
    let p1 = 2 * (2 * v1.0 + v1.1) + v1.2;
    let p2 = 2 * (2 * v2.0 + v2.1) + v2.2;
    let p3 = 2 * (2 * v3.0 + v3.1) + v3.2;

    p3 + 8 * (p2 + 8 * p1)
}

fn get_grid_value(grid: &Vec<Vec<bool>>, x: isize, y: isize, oob_value: bool) -> bool {
    match x {
        x if (x >= 0) && ((x as usize) < grid[0].len()) => match y {
            y if (y >= 0) && ((y as usize) < grid.len()) => grid[y as usize][x as usize],

            _ => oob_value,
        },
        _ => oob_value,
    }
}

fn get_surrounding_value(grid: &Vec<Vec<bool>>, x: usize, y: usize, oob_value: bool) -> usize {
    let x = x as isize;
    let y = y as isize;
    let l1 = (
        get_grid_value(grid, x - 1, y - 1, oob_value),
        get_grid_value(grid, x, y - 1, oob_value),
        get_grid_value(grid, x + 1, y - 1, oob_value),
    );
    let l2 = (
        get_grid_value(grid, x - 1, y, oob_value),
        get_grid_value(grid, x, y, oob_value),
        get_grid_value(grid, x + 1, y, oob_value),
    );
    let l3 = (
        get_grid_value(grid, x - 1, y + 1, oob_value),
        get_grid_value(grid, x, y + 1, oob_value),
        get_grid_value(grid, x + 1, y + 1, oob_value),
    );

    get_pos(l1, l2, l3)
}

fn apply_transformation(
    grid: &Vec<Vec<bool>>,
    transform: &Vec<bool>,
    even: bool,
) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut out = vec![vec![false; width]; height];

    let oob_value = if transform[0] { !even } else { false };

    for y in 0..height {
        for x in 0..width {
            let pos = get_surrounding_value(grid, x, y, oob_value);
            out[y][x] = transform[pos];
        }
    }

    out
}

fn count_light(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter().map(|l| l.iter().filter(|v| **v).count()).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename_map = &args[1];
    let filename_transform = &args[2];
    let iterations = args[3].parse::<usize>().unwrap();

    // Open the file in read-only mode (ignoring errors).
    let file_map = File::open(filename_map).unwrap();
    let file_transform = File::open(filename_transform).unwrap();
    let reader_map = BufReader::new(file_map);
    let reader_transform = BufReader::new(file_transform);

    let transform: Vec<bool> = reader_transform
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
        .collect();

    let mut grid: Vec<Vec<bool>> = reader_map
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    add_margins(&mut grid, iterations);

    println!("Original light count: {}", count_light(&grid));

    for i in 0..iterations {
        grid = apply_transformation(&grid, &transform, i % 2 == 0);
    }
    println!("Final light count: {}", count_light(&grid));
}
