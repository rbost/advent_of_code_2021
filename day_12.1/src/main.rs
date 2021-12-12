use std::{
    collections::{BTreeMap, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Node {
    name: String,
    is_small: bool,
    children: Vec<usize>,
    is_end: bool,
}

fn insert_node(
    name: &str,
    nodes: &mut Vec<Node>,
    node_dict: &mut BTreeMap<String, usize>,
) -> usize {
    let n = Node {
        name: name.to_string(),
        is_small: name.chars().next().unwrap().is_lowercase(),
        children: Vec::<usize>::new(),
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

    find_paths_aux(nodes, &mut found_paths, init);

    found_paths
}

fn find_paths_aux(nodes: &Vec<Node>, found_paths: &mut Vec<Vec<usize>>, path: VecDeque<usize>) {
    let current_node = &nodes[*path.back().unwrap()];
    if current_node.is_end {
        found_paths.push(path.into());
    } else {
        for c in &current_node.children {
            if nodes[*c].is_small && path.contains(c) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push_back(*c);
            find_paths_aux(nodes, found_paths, new_path);
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
    nodes[end].is_end = true;

    let path = find_paths(&nodes, start);

    println!("Found {} paths", path.len());
}
