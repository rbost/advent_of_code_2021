use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct BaseGraphNode {
    weight: u32,
    path_weight: Option<u32>,
    in_shortest_path_tree: bool,
}

impl Ord for BaseGraphNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.path_weight, other.path_weight) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => (Ordering::Greater),
            (Some(_), None) => (Ordering::Less),
            (Some(x), Some(y)) => (x.cmp(&y)),
        }
    }
}
impl PartialOrd for BaseGraphNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct GraphNode {
    x: usize,
    y: usize,
    weight: u32,
    path_weight: u32,
}

fn get_weight_in_grid(base_grid: &Vec<Vec<BaseGraphNode>>, (x, y): (usize, usize)) -> u32 {
    let width = base_grid[0].len();
    let height = base_grid.len();

    let base_weight = base_grid[y % height][x % width].weight;

    let offset = x / width + y / height;
    let adjusted_weight = ((base_weight + offset as u32 - 1) % 9) + 1;

    adjusted_weight
}

fn find_min_path_weight<'a>(frontier: &'a Vec<GraphNode>) -> (usize, u32) {
    let (i, n, _) = frontier
        .iter()
        .enumerate()
        .map(|(i, node)| (i, node, node.path_weight))
        .min_by(|(_, _, w1), (_, _, w2)| w1.cmp(w2))
        .unwrap();

    (i, n.weight)
}

// fn is_explored((x, y): (usize, usize), frontier: &Vec<GraphNode>) -> bool {
//     // let min_y = frontier.iter().min_by(|n1, n2| n1.x.cmp(&n2.x)).unwrap();
//     // if
//     let min_x_node = frontier
//         .iter()
//         .filter(|n| n.y == y)
//         .min_by(|n1, n2| n1.x.cmp(&n2.x));

//     if min_x_node.is_some() {
//         min_x_node.unwrap().x >= x
//     } else {
//         // no frontier node on the line
//         // either the frontier is above the node (unexplored) or it is under
//         frontier.iter().all(|n| n.y >= y)
//     }
// }

fn is_in_frontier((x, y): (usize, usize), frontier: &Vec<GraphNode>) -> Option<usize> {
    frontier
        .iter()
        .find_position(|n| (n.x, n.y) == (x, y))
        .map(|opt| opt.0)
}
fn update_node(
    grid: &mut Vec<Vec<BaseGraphNode>>,
    frontier: &mut Vec<GraphNode>,
    explored: &mut Vec<Vec<bool>>,
    (x, y): (usize, usize),
    path_weight: u32,
) -> Option<GraphNode> {
    let weight = get_weight_in_grid(grid, (x, y));

    let new_path_weight = path_weight + weight;

    if let Some(node_index) = is_in_frontier((x, y), frontier) {
        let node = &mut frontier[node_index];
        if node.path_weight > new_path_weight {
            node.path_weight = new_path_weight;
        }
        None
    } else if !(explored[y][x]) {
        // !is_explored((x, y), frontier) {
        // not visited
        let new_node = GraphNode {
            x,
            y,
            weight,
            path_weight: new_path_weight,
        };
        // println!("Insert new node in frontier: {:?}", new_node);
        explored[y][x] = true;
        Some(new_node)
    } else {
        None
    }

    // if !is_explored((x, y), frontier) {
    //     // not visited
    //     let new_node = GraphNode {
    //         x,
    //         y,
    //         weight,
    //         path_weight: new_path_weight,
    //     };
    //     println!("Insert new node in frontier: {:?}", new_node);
    //     Some(new_node)
    // } else if let Some(node_index) = is_in_frontier((x, y), frontier) {
    //     let node = &mut frontier[node_index];
    //     if node.path_weight > new_path_weight {
    //         node.path_weight = new_path_weight;
    //     }
    //     None
    // } else {
    //     None
    // }
}

fn update_neighbors(
    grid: &mut Vec<Vec<BaseGraphNode>>,
    frontier: &mut Vec<GraphNode>,
    explored: &mut Vec<Vec<bool>>,
    node_index: usize,
) -> Vec<GraphNode> {
    let path_weight = frontier[node_index].path_weight;
    let x = frontier[node_index].x;
    let y = frontier[node_index].y;
    let mut new_border_nodes = vec![];

    if x > 0 {
        if let Some(n) = update_node(grid, frontier, explored, (x - 1, y), path_weight) {
            new_border_nodes.push(n);
        }
    }
    if y > 0 {
        if let Some(n) = update_node(grid, frontier, explored, (x, y - 1), path_weight) {
            new_border_nodes.push(n);
        }
    }
    if y < 5 * grid.len() - 1 {
        if let Some(n) = update_node(grid, frontier, explored, (x, y + 1), path_weight) {
            new_border_nodes.push(n);
        }
    }
    if x < 5 * grid[0].len() - 1 {
        if let Some(n) = update_node(grid, frontier, explored, (x + 1, y), path_weight) {
            new_border_nodes.push(n);
        }
    }

    new_border_nodes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<BaseGraphNode>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| BaseGraphNode {
                    weight: c.to_digit(10).unwrap() as u32,
                    path_weight: None,
                    in_shortest_path_tree: false,
                })
                .collect()
        })
        .collect();

    let (target_x, target_y) = ((5 * grid[0].len()) - 1, (5 * grid.len()) - 1);

    let origin = GraphNode {
        x: 0,
        y: 0,
        weight: 0,
        path_weight: 0,
    };
    // grid[0][0].in_shortest_path_tree = true;
    // grid[0][0].path_weight = Some(0);

    let mut explored = vec![vec![false; 5 * grid[0].len()]; 5 * grid.len()];
    explored[origin.y][origin.x] = true;

    let mut frontier = vec![origin];
    // update_neighbors(&mut grid, &mut frontier, (0, 0));

    let node_count = 5 * grid.len() * 5 * grid[0].len();

    let mut path_weight = None;
    for _node_index in 1..=node_count {
        // println!("Frontier: {:?} {}", frontier, node_index);

        let (min_node_f_index, _) = find_min_path_weight(&frontier);

        // println!(
        //     "{:?}",
        //     (frontier[min_node_f_index].x, frontier[min_node_f_index].y)
        // );

        if (frontier[min_node_f_index].x, frontier[min_node_f_index].y) == (target_x, target_y) {
            // println!("Found target!");

            path_weight = Some(frontier[min_node_f_index].path_weight);
            break;
        }

        let mut new_nodes =
            update_neighbors(&mut grid, &mut frontier, &mut explored, min_node_f_index);

        frontier.remove(min_node_f_index);
        frontier.append(&mut new_nodes);
    }

    // let end_node = &grid[grid.len() - 1][grid[0].len() - 1];
    // assert!(end_node.in_shortest_path_tree);
    // assert!(end_node.path_weight.is_some());

    println!("Path weight: {}", path_weight.unwrap());
}
