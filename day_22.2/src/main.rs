use std::{
    collections::btree_map::Iter,
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    ops::{Deref, RangeInclusive},
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    z_range: RangeInclusive<isize>,
    value: bool,
}

impl Area {
    fn size(&self) -> usize {
        let x_size = (self.x_range.end() - self.x_range.start()) as usize + 1;
        let y_size = (self.y_range.end() - self.y_range.start()) as usize + 1;
        let z_size = (self.z_range.end() - self.z_range.start()) as usize + 1;

        x_size * y_size * z_size
    }

    fn intersects(&self, other: &Area) -> bool {
        let x_intersection = intersects(&self.x_range, &other.x_range);
        let y_intersection = intersects(&self.y_range, &other.y_range);
        let z_intersection = intersects(&self.z_range, &other.z_range);

        x_intersection && y_intersection && z_intersection
    }

    fn is_contained_by(&self, other: &Area) -> bool {
        is_contained_by(&self.x_range, &other.x_range)
            && is_contained_by(&self.y_range, &other.y_range)
            && is_contained_by(&self.z_range, &other.z_range)
    }

    fn cut_x_plane(&self, cut: isize, cut_min: bool) -> impl Iterator<Item = Area> {
        let new_x_ranges = cut_interval(&self.x_range, cut, cut_min);
        let y_range = self.y_range.clone();
        let z_range = self.z_range.clone();
        let value = self.value;
        new_x_ranges.into_iter().map(move |x_range| Area {
            x_range,
            y_range: y_range.clone(),
            z_range: z_range.clone(),
            value,
        })
    }
    fn cut_y_plane(&self, cut: isize, cut_min: bool) -> impl Iterator<Item = Area> {
        let new_y_ranges = cut_interval(&self.y_range, cut, cut_min);
        let x_range = self.x_range.clone();
        let z_range = self.z_range.clone();
        let value = self.value;
        new_y_ranges.into_iter().map(move |y_range| Area {
            x_range: x_range.clone(),
            y_range,
            z_range: z_range.clone(),
            value,
        })
    }
    fn cut_z_plane(&self, cut: isize, cut_min: bool) -> impl Iterator<Item = Area> {
        let new_z_ranges = cut_interval(&self.z_range, cut, cut_min);
        let x_range = self.x_range.clone();
        let y_range = self.y_range.clone();
        let value = self.value;
        new_z_ranges.into_iter().map(move |z_range| Area {
            x_range: x_range.clone(),
            y_range: y_range.clone(),
            z_range,
            value,
        })
    }

    fn cut_area(&self, cut: &Area) -> Box<dyn Iterator<Item = Area>> {
        if self.intersects(cut) {
            let x_start = *cut.x_range.start();
            let x_end = *cut.x_range.end();
            let y_start = *cut.y_range.start();
            let y_end = *cut.y_range.end();
            let z_start = *cut.z_range.start();
            let z_end = *cut.z_range.end();
            let new_areas = self.cut_x_plane(x_start, false);
            let new_areas = new_areas
                .into_iter()
                .flat_map(move |a| a.cut_x_plane(x_end, true));
            let new_areas = new_areas
                .into_iter()
                .flat_map(move |a| a.cut_y_plane(y_start, false));
            let new_areas = new_areas
                .into_iter()
                .flat_map(move |a| a.cut_y_plane(y_end, true));
            let new_areas = new_areas
                .into_iter()
                .flat_map(move |a| a.cut_z_plane(z_start, false));
            Box::new(
                new_areas
                    .into_iter()
                    .flat_map(move |a| a.cut_z_plane(z_end, true)),
            )
        } else {
            Box::new(iter::once(self.clone()))
        }
    }

    fn substract_area(self, cut: Area) -> impl Iterator<Item = Area> {
        let new_areas = self.cut_area(&cut);

        new_areas.filter(move |a| !a.is_contained_by(&cut))
        //         .cloned()
        // let mut res = Vec::<Area>::with_capacity(new_areas.len());
        // res.extend(
        //     new_areas
        //         .iter()
        //         .filter(|a| !a.is_contained_by(cut))
        //         .cloned(),
        // );

        // res
    }
}

fn parse_range(s: &str) -> RangeInclusive<isize> {
    // remove the first two chars
    let (_, r) = s.split_once('=').unwrap();
    let (min_s, max_s) = r.split_once("..").unwrap();
    let min = min_s.parse::<isize>().unwrap();
    let max = max_s.parse::<isize>().unwrap();

    RangeInclusive::new(min, max)
}

// true if s contains r
pub fn is_contained_by(r: &RangeInclusive<isize>, s: &RangeInclusive<isize>) -> bool {
    s.start() <= r.start() && s.end() >= r.end()
}

pub fn intersects(s: &RangeInclusive<isize>, r: &RangeInclusive<isize>) -> bool {
    s.end() >= r.start() && s.start() <= r.end()
}

pub fn cut_interval(
    interval: &RangeInclusive<isize>,
    cut: isize,
    cut_min: bool,
) -> Vec<RangeInclusive<isize>> {
    if (cut < *interval.start() && cut < *interval.end())
        || (cut > *interval.start() && cut > *interval.end())
        || (interval.start() == interval.end())
    {
        vec![interval.clone()]
    } else if cut_min {
        if cut == *interval.end() {
            vec![interval.clone()]
        } else {
            vec![
                RangeInclusive::new(*interval.start(), cut),
                RangeInclusive::new(cut + 1, *interval.end()),
            ]
        }
    } else {
        // !cut_min
        if cut == *interval.start() {
            vec![interval.clone()]
        } else {
            vec![
                RangeInclusive::new(*interval.start(), cut - 1),
                RangeInclusive::new(cut, *interval.end()),
            ]
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
    let orders: Vec<_> = reader
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

            Area {
                x_range,
                y_range,
                z_range,
                value: new_val,
            }
        })
        .collect();

    let mut on_areas = Vec::<Area>::new();

    for order in orders {
        let new_on = on_areas
            .into_iter()
            .flat_map(|a| a.substract_area(order.clone()))
            .collect();

        on_areas = new_on;

        if order.value {
            // 'ON' order
            on_areas.push(order);
        }
    }

    let on_cubes: usize = on_areas.iter().map(|a| a.size()).sum();
    println!("{}", on_cubes);
}
