use std::{
    collections::{BTreeMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Node {
    is_small: bool,
    children: Vec<usize>,
    is_start: bool,
    is_end: bool,
}

fn insert_node(
    name: &str,
    nodes: &mut Vec<Node>,
    node_dict: &mut BTreeMap<String, usize>,
) -> usize {
    let n = Node {
        is_small: name.chars().next().unwrap().is_lowercase(),
        children: Vec::<usize>::new(),
        is_start: false,
        is_end: false,
    };

    let index = nodes.len();
    nodes.push(n);
    node_dict.insert(name.to_string(), index);

    index
}

fn find_paths(nodes: &Vec<Node>, start: usize) -> Vec<Vec<usize>> {
    let mut found_paths = Vec::<Vec<usize>>::new();

    let mut init = VecDeque::<usize>::new();
    init.push_back(start);

    find_paths_aux(nodes, &mut found_paths, init, false);

    found_paths
}

fn find_paths_aux(
    nodes: &Vec<Node>,
    found_paths: &mut Vec<Vec<usize>>,
    path: VecDeque<usize>,
    has_visited_small_twice: bool,
) {
    let current_node = &nodes[*path.back().unwrap()];
    if current_node.is_end {
        found_paths.push(path.into());
    } else {
        for c in &current_node.children {
            if nodes[*c].is_start {
                continue;
            }
            if !nodes[*c].is_small || !path.contains(c) {
                let mut new_path = path.clone();
                new_path.push_back(*c);
                find_paths_aux(nodes, found_paths, new_path, has_visited_small_twice);
            } else if !has_visited_small_twice {
                // at this point c is small and has been visited
                let mut new_path = path.clone();
                new_path.push_back(*c);
                find_paths_aux(nodes, found_paths, new_path, true);
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

    let mut nodes = Vec::<Node>::new();
    let mut node_dict = BTreeMap::<String, usize>::new();

    reader.lines().for_each(|l| {
        let edge: Vec<_> = l
            .unwrap()
            .split('-')
            .map(|name| {
                node_dict
                    .get(name)
                    .copied()
                    .unwrap_or_else(|| insert_node(name, &mut nodes, &mut node_dict))
            })
            .collect();

        assert_eq!(edge.len(), 2);

        nodes[edge[0]].children.push(edge[1]);
        nodes[edge[1]].children.push(edge[0]);
    });

    let start = *node_dict.get("start").unwrap();
    let end = *node_dict.get("end").unwrap();
    nodes[start].is_start = true;
    nodes[end].is_end = true;

    let path = find_paths(&nodes, start);

    println!("Found {} paths", path.len());
}
