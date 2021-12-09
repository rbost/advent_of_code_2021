use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{Itertools, Position};

fn get_low_points(height_map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let candidates: Vec<Vec<_>> = height_map
        .iter()
        .map(|l| {
            l.iter()
                .tuple_windows()
                .enumerate()
                .with_position()
                .filter_map(|elt| match elt {
                    Position::Only(_) => unimplemented!(),
                    Position::Middle((pos, (v0, v1, v2))) => {
                        if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                    Position::First((pos, (v0, v1, v2))) => {
                        if v0 < v1 {
                            Some(pos)
                        } else if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                    Position::Last((pos, (v0, v1, v2))) => {
                        if v2 < v1 {
                            Some(pos + 2)
                        } else if v0 > v1 && v1 < v2 {
                            Some(pos + 1)
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect();

    candidates
        .iter()
        .enumerate()
        .with_position()
        .map(|elt| match elt {
            Position::First((y, cands)) => cands
                .iter()
                .filter(|x| height_map[y][**x as usize] < height_map[y + 1][**x as usize])
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Middle((y, cands)) => cands
                .iter()
                .filter(|x| {
                    height_map[y][**x as usize] < height_map[y + 1][**x as usize]
                        && height_map[y][**x as usize] < height_map[y - 1][**x as usize]
                })
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Last((y, cands)) => cands
                .iter()
                .filter(|x| height_map[y][**x as usize] < height_map[y - 1][**x as usize])
                .map(|p| (*p, y))
                .collect_vec(),
            Position::Only(_) => todo!(),
        })
        .flatten()
        .collect_vec()
}

struct GraphCell {
    // value: u8,
    go_up: bool,
    go_down: bool,
    go_right: bool,
    go_left: bool,
    visited: bool,
}

fn compute_graph(height_map: &[Vec<u8>]) -> Vec<Vec<GraphCell>> {
    height_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, v)| {
                    let go_up = if y == 0 {
                        false
                    } else {
                        let w = height_map[y - 1][x];
                        *v < w && w != 9
                    };
                    let go_down = if y == height_map.len() - 1 {
                        false
                    } else {
                        let w = height_map[y + 1][x];
                        *v < w && w != 9
                    };
                    let go_left = if x == 0 {
                        false
                    } else {
                        let w = height_map[y][x - 1];
                        *v < w && w != 9
                    };
                    let go_right = if x == line.len() - 1 {
                        false
                    } else {
                        let w = height_map[y][x + 1];
                        *v < w && w != 9
                    };
                    GraphCell {
                        // value: *v,
                        go_up,
                        go_down,
                        go_right,
                        go_left,
                        visited: false,
                    }
                })
                .collect()
        })
        .collect()
}

fn basin_size(graph: &mut Vec<Vec<GraphCell>>, pos: (usize, usize)) -> usize {
    let mut size = 0usize;
    let mut vertices = VecDeque::<(usize, usize)>::new();
    vertices.push_back(pos);

    while let Some((x, y)) = vertices.pop_back() {
        let cell = &mut graph[y][x];
        if !cell.visited {
            size += 1;
            cell.visited = true;

            if cell.go_down {
                vertices.push_back((x, y + 1));
            }
            if cell.go_up {
                vertices.push_back((x, y - 1));
            }
            if cell.go_left {
                vertices.push_back((x - 1, y));
            }
            if cell.go_right {
                vertices.push_back((x + 1, y));
            }
        }
    }

    size
}

fn compute_basin_sizes(height_map: &[Vec<u8>], low_points: &[(usize, usize)]) -> Vec<usize> {
    let mut graph = compute_graph(height_map);

    low_points
        .iter()
        .map(|point| basin_size(&mut graph, *point))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let height_map: Vec<Vec<u8>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let low_points = get_low_points(&height_map);

    let mut basin_sizes = compute_basin_sizes(&height_map, &low_points);
    basin_sizes.sort_unstable();
    let l = basin_sizes.len();
    println!(
        "Result: {}",
        basin_sizes[l - 1] * basin_sizes[l - 2] * basin_sizes[l - 3]
    );
}
