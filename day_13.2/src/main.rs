use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn fold(self, fold: &Fold) -> Self {
        match fold {
            Fold::X(x0) => Point {
                x: if self.x > *x0 {
                    2 * x0 - self.x
                } else {
                    self.x
                },
                y: self.y,
            },
            Fold::Y(y0) => Point {
                x: self.x,
                y: if self.y > *y0 {
                    2 * y0 - self.y
                } else {
                    self.y
                },
            },
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

fn print_points(points: &HashSet<Point>) {
    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;

    let mut table = vec![vec!['.'; max_x + 1]; max_y + 1];

    points
        .iter()
        .for_each(|p| table[p.y as usize][p.x as usize] = '#');

    table.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename_points = &args[1];
    let filename_folds = &args[2];
    // Open the file in read-only mode (ignoring errors).
    let file_points = File::open(filename_points).unwrap();
    let file_folds = File::open(filename_folds).unwrap();
    let reader_points = BufReader::new(file_points);
    let reader_folds = BufReader::new(file_folds);

    let points: HashSet<Point> = reader_points
        .lines()
        .map(|l| {
            let line: String = l.unwrap();
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse::<u32>().unwrap();
            let y = iter.next().unwrap().parse::<u32>().unwrap();
            Point { x, y }
        })
        .collect();

    let folds: Vec<Fold> = reader_folds
        .lines()
        .map(|l| {
            let line: String = l.unwrap();
            let (_, line) = line.split_at(11);
            let (coord, digit) = line.split_at(2);
            match coord {
                "x=" => Fold::X(digit.parse().unwrap()),
                "y=" => Fold::Y(digit.parse().unwrap()),
                _ => unimplemented!(),
            }
        })
        .collect();

    let points = folds.iter().fold(points, |pset, fold| {
        pset.into_iter().map(|p| p.fold(fold)).collect()
    });

    print_points(&points);
}
