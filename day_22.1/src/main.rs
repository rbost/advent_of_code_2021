use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn parse_range(s: &str) -> RangeInclusive<isize> {
    // remove the first two chars
    let (_, r) = s.split_once('=').unwrap();
    let (min_s, max_s) = r.split_once("..").unwrap();
    let min = min_s.parse::<isize>().unwrap();
    let max = max_s.parse::<isize>().unwrap();

    RangeInclusive::new(min, max)
}

fn offset_range(
    initial_range: &RangeInclusive<isize>,
    offset: usize,
) -> Option<RangeInclusive<usize>> {
    let min = initial_range.start() + (offset as isize);
    let max = initial_range.end() + (offset as isize);

    if min < 0 || max as usize > 2 * offset {
        None
    } else {
        assert!(min >= 0);
        assert!(min as usize <= 2 * offset);
        assert!(max >= 0);
        assert!(max as usize <= 2 * offset);

        Some(RangeInclusive::new(min as usize, max as usize))
    }
}

fn set_grid(
    grid: &mut Vec<Vec<Vec<bool>>>,
    new_value: bool,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    z_range: RangeInclusive<usize>,
) {
    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                grid[z][y][x] = new_value;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut max_coord: usize = 0;
    let ranges: Vec<_> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();

            let (left, right) = l.split_once(' ').unwrap();
            let new_val = left == "on";
            let mut range_iter = right.split(',');

            let x_str = range_iter.next().unwrap();
            let x_range = parse_range(x_str);
            let y_str = range_iter.next().unwrap();
            let y_range = parse_range(y_str);
            let z_str = range_iter.next().unwrap();
            let z_range = parse_range(z_str);

            max_coord = max_coord
                .max(x_range.start().abs() as usize)
                .max(x_range.end().abs() as usize);
            max_coord = max_coord
                .max(y_range.start().abs() as usize)
                .max(y_range.end().abs() as usize);
            max_coord = max_coord
                .max(z_range.start().abs() as usize)
                .max(z_range.end().abs() as usize);

            // if x_range.is_some() && y_range.is_some() && z_range.is_some() {
            //     set_grid(
            //         &mut grid,
            //         new_val,
            //         x_range.unwrap(),
            //         y_range.unwrap(),
            //         z_range.unwrap(),
            //     );
            // }
            ((x_range, y_range, z_range), new_val)
        })
        .collect();

    let pb_size: usize = if args.len() >= 3 {
        args[2].parse().unwrap()
    } else {
        max_coord
    };

    let size = 2 * pb_size + 1;
    let mut grid = vec![vec![vec![false; size]; size]; size];

    ranges
        .iter()
        .for_each(|((x_range, y_range, z_range), new_val)| {
            let x_range = offset_range(x_range, pb_size);
            let y_range = offset_range(y_range, pb_size);
            let z_range = offset_range(z_range, pb_size);
            if x_range.is_some() && y_range.is_some() && z_range.is_some() {
                set_grid(
                    &mut grid,
                    *new_val,
                    x_range.unwrap(),
                    y_range.unwrap(),
                    z_range.unwrap(),
                );
            }
        });

    let sum: usize = grid
        .iter()
        .map(|l| {
            l.iter()
                .map(|c| c.iter().filter(|b| **b).count())
                .sum::<usize>()
        })
        .sum();
    println!("{}", sum);
}
