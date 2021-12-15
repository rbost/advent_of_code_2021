use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq)]
struct GraphNode {
    weight: u32,
    path_weight: Option<u32>,
    in_shortest_path_tree: bool,
}

impl Ord for GraphNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.path_weight, other.path_weight) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => (Ordering::Greater),
            (Some(_), None) => (Ordering::Less),
            (Some(x), Some(y)) => (x.cmp(&y)),
        }
    }
}
impl PartialOrd for GraphNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_path_weight(
    grid: &Vec<Vec<GraphNode>>,
    frontier: &Vec<(usize, usize)>,
) -> (usize, (usize, usize), u32) {
    let (i, (&x, &y), _) = frontier
        .iter()
        .enumerate()
        .map(|(i, (x, y))| (i, (x, y), grid[*y][*x].path_weight.unwrap()))
        .min_by(|(_, _, w1), (_, _, w2)| w1.cmp(w2))
        .unwrap();

    (i, (x, y), grid[y][x].weight)
}

fn update_node(
    grid: &mut Vec<Vec<GraphNode>>,
    frontier: &mut Vec<(usize, usize)>,
    (x, y): (usize, usize),
    path_weight: u32,
) {
    let node = &mut grid[y][x];

    if !node.in_shortest_path_tree {
        let new_path_weight = path_weight + node.weight;
        if node.path_weight.is_none() {
            // not visited
            frontier.push((x, y));
        }
        match node.path_weight {
            None => node.path_weight = Some(new_path_weight),
            Some(w) if w > new_path_weight => node.path_weight = Some(new_path_weight),
            _ => (),
        }
    }
}

fn update_neighbors(
    grid: &mut Vec<Vec<GraphNode>>,
    frontier: &mut Vec<(usize, usize)>,
    (x, y): (usize, usize),
) {
    let node = &grid[y][x];
    let path_weight = node.path_weight.unwrap();

    if x > 0 {
        update_node(grid, frontier, (x - 1, y), path_weight);
    }
    if y > 0 {
        update_node(grid, frontier, (x, y - 1), path_weight);
    }
    if y < grid.len() - 1 {
        update_node(grid, frontier, (x, y + 1), path_weight);
    }
    if x < grid[y].len() - 1 {
        update_node(grid, frontier, (x + 1, y), path_weight);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<GraphNode>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| GraphNode {
                    weight: c.to_digit(10).unwrap() as u32,
                    path_weight: None,
                    in_shortest_path_tree: false,
                })
                .collect()
        })
        .collect();

    grid[0][0].in_shortest_path_tree = true;
    grid[0][0].path_weight = Some(0);
    let mut frontier = vec![];
    update_neighbors(&mut grid, &mut frontier, (0, 0));

    let node_count = grid.len() * grid[0].len();

    for _node_index in 2..=node_count {
        let (min_node_f_index, min_node_pos, _) = find_min_path_weight(&grid, &frontier);
        grid[min_node_pos.1][min_node_pos.0].in_shortest_path_tree = true;

        frontier.remove(min_node_f_index);

        update_neighbors(&mut grid, &mut frontier, min_node_pos);

        if min_node_pos == (grid[0].len() - 1, grid.len() - 1) {
            break;
        }
    }

    let end_node = &grid[grid.len() - 1][grid[0].len() - 1];
    assert!(end_node.in_shortest_path_tree);
    assert!(end_node.path_weight.is_some());

    println!("Path weight: {}", end_node.path_weight.unwrap());
}
