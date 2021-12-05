use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = (); // I do not want to define a specific error
                   // We panic instead

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elts = s.trim().split(',');
        if let Some(x) = elts.next() {
            let x = x.parse().unwrap();
            if let Some(y) = elts.next() {
                let y = y.parse().unwrap();
                return Ok(Point { x, y });
            }
        }
        panic!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VentLine {
    start: Point,
    end: Point,
}

impl FromStr for VentLine {
    type Err = (); // I do not want to define a specific error
                   // We panic instead

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elts = s.trim().split("->");
        if let Some(start) = elts.next() {
            let start = start.parse().unwrap();
            if let Some(end) = elts.next() {
                let end = end.parse().unwrap();
                return Ok(VentLine { start, end });
            }
        }
        panic!();
    }
}

#[derive(Debug)]
struct Diagram(Vec<Vec<u32>>);

impl Diagram {
    fn new(width: usize, height: usize) -> Self {
        Diagram(vec![vec![0; width]; height])
    }

    fn draw_vent_line(&mut self, vent_line: &VentLine) {
        if vent_line.start.x == vent_line.end.x {
            // vertical line
            let x = vent_line.start.x as usize;
            let min_y = vent_line.start.y.min(vent_line.end.y) as usize;
            let max_y = vent_line.start.y.max(vent_line.end.y) as usize;

            for i in min_y..=max_y {
                self.0[i][x] += 1;
            }
        } else if vent_line.start.y == vent_line.end.y {
            // horizontal
            let y = vent_line.start.y as usize;
            let min_x = vent_line.start.x.min(vent_line.end.x) as usize;
            let max_x = vent_line.start.x.max(vent_line.end.x) as usize;

            for i in min_x..=max_x {
                self.0[y][i] += 1;
            }
        } else {
            // unimplemented!();
        }
    }

    fn count_overlaps(&self) -> usize {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&v| if v > 1 { 1usize } else { 0usize })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let vent_lines: Vec<VentLine> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let max_x = vent_lines
        .iter()
        .map(|vl| vl.start.x.max(vl.end.x))
        .max()
        .unwrap();

    let max_y = vent_lines
        .iter()
        .map(|vl| vl.start.y.max(vl.end.y))
        .max()
        .unwrap();

    let mut diagram = Diagram::new((max_x + 1) as usize, (max_y + 1) as usize);

    vent_lines.iter().for_each(|vl| diagram.draw_vent_line(vl));

    // println!("{:?}", diagram);
    let overlaps = diagram.count_overlaps();

    println!("Overlaps {}", overlaps);
}
