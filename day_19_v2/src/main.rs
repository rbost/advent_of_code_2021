use std::{
    collections::HashSet,
    env,
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Probe {
    x: i64,
    y: i64,
    z: i64,
    distances: Vec<(usize, u64)>,
}
#[derive(Debug)]
struct Scanner(Vec<Probe>);

impl Probe {
    fn diff_coords(&self, other: &Probe) -> (i64, i64, i64) {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        let diff_z = self.z - other.z;

        (diff_x, diff_y, diff_z)
    }

    fn distance_squared(&self, other: &Probe) -> u64 {
        let (diff_x, diff_y, diff_z) = self.diff_coords(other);

        (diff_x * diff_x + diff_y * diff_y + diff_z * diff_z) as u64
    }

    fn switch_x_y(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
    fn switch_x_z(&mut self) {
        std::mem::swap(&mut self.x, &mut self.z);
    }
    fn switch_y_z(&mut self) {
        std::mem::swap(&mut self.y, &mut self.z);
    }

    fn reverse_x(&mut self) {
        self.x = -self.x;
    }
    fn reverse_y(&mut self) {
        self.y = -self.y;
    }
    fn reverse_z(&mut self) {
        self.z = -self.z;
    }

    fn apply_offset(&mut self, (x, y, z): (i64, i64, i64)) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn compute_distances(probes: &mut Vec<Probe>) {
    let size = probes.len();
    (0..size).for_each(|i| {
        probes[i].distances = (0..size)
            .map(|j| (j, probes[i].distance_squared(&probes[j])))
            .collect();

        probes[i].distances.sort_unstable_by_key(|c| c.1);
    })
}

fn find_common_distances(list1: &[(usize, u64)], list2: &[(usize, u64)]) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();
    let mut i = 0;
    let mut j = 0;

    while i < list1.len() && j < list2.len() {
        let (pos_1, d_1) = list1[i];
        let (pos_2, d_2) = list2[j];

        match d_1.cmp(&d_2) {
            std::cmp::Ordering::Equal => {
                result.push((pos_1, pos_2));
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                j += 1;
            }
            std::cmp::Ordering::Less => {
                i += 1;
            }
        }
    }

    result
}

fn match_distances(scanner_1: &Scanner, scanner_2: &Scanner) -> Vec<(usize, usize)> {
    let mut common_probes = Vec::new();
    scanner_1.0.iter().enumerate().for_each(|(i, p_1)| {
        scanner_2.0.iter().enumerate().for_each(|(j, p_2)| {
            let common = find_common_distances(&p_1.distances, &p_2.distances);

            if common.len() == 12 {
                common_probes.push((i, j));
                // println!("{:?}", common);
            }
        });
        // println!();
    });
    common_probes
}

fn put_to_same_axis(
    base_scanner: &Scanner,
    scanner: &mut Scanner,
    common_probes: &[(usize, usize)],
) {
    let (p_0_i, p_0_j) = common_probes[0];
    let (p_1_i, p_1_j) = common_probes[1];
    let diff_base = base_scanner.0[p_0_i].diff_coords(&base_scanner.0[p_1_i]);
    let mut diff = scanner.0[p_0_j].diff_coords(&scanner.0[p_1_j]);

    assert_ne!(diff_base.0.abs(), diff_base.1.abs());
    assert_ne!(diff_base.0.abs(), diff_base.2.abs());
    assert_ne!(diff_base.1.abs(), diff_base.2.abs());

    if diff_base.0.abs() != diff.0.abs() {
        // the x axis is not the same
        // find it in the other base
        if diff_base.0.abs() == diff.1.abs() {
            // x -> y
            scanner.0.iter_mut().for_each(|p| p.switch_x_y());
            std::mem::swap(&mut diff.0, &mut diff.1);
        } else if diff_base.0.abs() == diff.2.abs() {
            // x -> z
            scanner.0.iter_mut().for_each(|p| p.switch_x_z());
            std::mem::swap(&mut diff.0, &mut diff.2);
        } else {
            unreachable!("We should have almost identical triples up to permutation");
        }
    }
    // now diff_base.0.abs() == diff.0.abs()
    assert_eq!(diff_base.0.abs(), diff.0.abs());
    if diff_base.0 == -diff.0 {
        scanner.0.iter_mut().for_each(|p| p.reverse_x());
        diff.0 = -diff.0;
    }

    if diff_base.1.abs() != diff.1.abs() {
        // the y axis is not the same
        // find it in the other base
        // it has to be the z axis, as the x axis are the same for both scanners now
        if diff_base.1.abs() == diff.2.abs() {
            // y -> z
            scanner.0.iter_mut().for_each(|p| p.switch_y_z());
            std::mem::swap(&mut diff.1, &mut diff.2);
        } else {
            unreachable!("We should have almost identical triples up to permutation");
        }
    }

    // now diff_base.1.abs() == diff.1.abs()
    assert_eq!(diff_base.1.abs(), diff.1.abs());
    if diff_base.1 == -diff.1 {
        scanner.0.iter_mut().for_each(|p| p.reverse_y());
        diff.1 = -diff.1;
    }

    // now diff_base.2.abs() == diff.2.abs()
    assert_eq!(diff_base.2.abs(), diff.2.abs());
    if diff_base.2 == -diff.2 {
        scanner.0.iter_mut().for_each(|p| p.reverse_z());
        diff.2 = -diff.2;
    }
}

fn compute_and_apply_offset(
    base_scanner: &Scanner,
    scanner: &mut Scanner,
    common_probes: &[(usize, usize)],
) -> (i64, i64, i64) {
    let (i, j) = common_probes[0];
    let offset = base_scanner.0[i].diff_coords(&scanner.0[j]);

    scanner.0.iter_mut().for_each(|p| p.apply_offset(offset));

    offset
}

fn manhattan_distance((x0, y0, z0): (i64, i64, i64), (x1, y1, z1): (i64, i64, i64)) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut scanners = Vec::<Scanner>::new();
    // let mut probes = Vec::<Probe>::new();

    let remaining_probes = reader.lines().fold(Vec::<Probe>::new(), |mut probes, l| {
        let l = l.unwrap();
        if l.is_empty() {
            // end of scanner
            scanners.push(Scanner(probes));
            Vec::<Probe>::new()
        } else if l.starts_with("--- scanner") {
            // do nothing, the probe vector has already been initialized
            probes
        } else {
            let mut components_iter = l.split(',');
            let x = components_iter.next().unwrap().parse().unwrap();
            let y = components_iter.next().unwrap().parse().unwrap();
            let z = components_iter.next().unwrap().parse().unwrap();

            probes.push(Probe {
                x,
                y,
                z,
                distances: Vec::new(),
            });
            probes
        }
    });

    // don't forget the last scanner
    if !remaining_probes.is_empty() {
        scanners.push(Scanner(remaining_probes));
    }

    scanners
        .iter_mut()
        .for_each(|s| compute_distances(&mut s.0));

    let mut base_indices = vec![(0, (0, 0, 0))];
    let mut completed_indices = vec![];
    let mut unprocessed_indices: Vec<_> = (1..scanners.len()).collect();

    while let Some((base, origin)) = base_indices.pop() {
        let new_unprocessed_indices = unprocessed_indices
            .iter()
            .filter_map(|&index| {
                let common_probes = match_distances(&scanners[base], &scanners[index]);
                if common_probes.is_empty() {
                    Some(index)
                } else {
                    let offset = if index > base {
                        let (s_i, s_j) = scanners.split_at_mut(index);
                        put_to_same_axis(&s_i[base], &mut s_j[0], &common_probes);
                        compute_and_apply_offset(&s_i[base], &mut s_j[0], &common_probes)
                    } else {
                        let (s_i, s_j) = scanners.split_at_mut(base);
                        put_to_same_axis(&s_j[0], &mut s_i[index], &common_probes);
                        compute_and_apply_offset(&s_j[0], &mut s_i[index], &common_probes)
                    };
                    base_indices.push((index, offset));
                    None
                }
            })
            .collect();

        unprocessed_indices = new_unprocessed_indices;
        completed_indices.push((base, origin));
    }

    assert!(unprocessed_indices.is_empty());

    let set: HashSet<(i64, i64, i64)> = scanners
        .iter()
        .flat_map(|s| s.0.iter().map(|p| (p.x, p.y, p.z)))
        .collect();

    // count the  probes
    let count: usize = set.len();
    println!("Unique Count {}", count);

    let scanner_locs: Vec<_> = completed_indices
        .iter()
        .chain(base_indices.iter())
        .map(|c| c.1)
        .collect();

    let max_dist = scanner_locs
        .iter()
        .filter_map(|loc1| {
            scanner_locs
                .iter()
                .map(|loc2| manhattan_distance(*loc1, *loc2))
                .max()
        })
        .max()
        .unwrap();
    println!("Max distance {}", max_dist);
}
