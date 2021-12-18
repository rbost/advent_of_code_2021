use core::fmt;
use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
};

#[derive(Debug, Clone)]
enum Node {
    Number(u32),
    Pair(Box<Node>, Box<Node>),
}

impl Node {
    fn split(&mut self) -> bool {
        match self {
            Node::Number(d) if *d >= 10 => {
                *self = Node::Pair(
                    Box::new(Node::Number(*d / 2)),
                    Box::new(Node::Number(*d - *d / 2)),
                );
                true
            }
            Node::Pair(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn add_leftmost_number(&mut self, num: u32) {
        match self {
            Node::Number(d) => *d += num,
            Node::Pair(left, _right) => left.add_leftmost_number(num),
        }
    }
    fn add_rightmost_number(&mut self, num: u32) {
        match self {
            Node::Number(d) => *d += num,
            Node::Pair(_left, right) => right.add_rightmost_number(num),
        }
    }

    fn explode_aux(&mut self, depth: usize) -> (bool, (Option<u32>, Option<u32>)) {
        // if depth >= 4 {

        match self {
            Node::Pair(left, right) => match (left.as_ref(), right.as_ref()) {
                (Node::Number(lv), Node::Number(rv)) if depth >= 4 => {
                    let r = (Some(*lv), Some(*rv));
                    // println!("Explode : {:?}", r);
                    *self = Node::Number(0);
                    (true, r)
                }
                (Node::Number(_lv), Node::Number(_rv)) => (false, (None, None)),
                (Node::Number(_), Node::Pair(..)) => {
                    let (has_exploded, (spill_left, spill_right)) = right.explode_aux(depth + 1);
                    if has_exploded {
                        // println!("Right child exploded : {:?}", (spill_left, spill_right));
                        if let Some(d) = spill_left {
                            left.add_rightmost_number(d);
                        }
                        (true, (None, spill_right))
                    } else {
                        (false, (None, None))
                    }
                }
                (Node::Pair(..), Node::Number(_)) => {
                    let (has_exploded, (spill_left, spill_right)) = left.explode_aux(depth + 1);

                    if has_exploded {
                        // println!("Left child exploded : {:?}", (spill_left, spill_right));

                        if let Some(d) = spill_right {
                            right.add_leftmost_number(d);
                        }
                        (true, (spill_left, None))
                    } else {
                        (false, (None, None))
                    }
                }
                (Node::Pair(..), Node::Pair(..)) => {
                    let (has_exploded, (spill_left, spill_right)) = left.explode_aux(depth + 1);
                    if has_exploded {
                        // println!("Left child exploded : {:?}", (spill_left, spill_right));

                        if let Some(d) = spill_right {
                            right.add_leftmost_number(d);
                        }
                        (true, (spill_left, None))
                    } else {
                        let (has_exploded, (spill_left, spill_right)) =
                            right.explode_aux(depth + 1);
                        if has_exploded {
                            // println!("Right child exploded : {:?}", (spill_left, spill_right));
                            if let Some(d) = spill_left {
                                left.add_rightmost_number(d);
                            }
                            (true, (None, spill_right))
                        } else {
                            (false, (None, None))
                        }
                    }
                }
            },
            Node::Number(_) => (false, (None, None)),
        }
    }

    fn reduce_step(&mut self) -> bool {
        self.explode_aux(0).0 || self.split()
    }

    fn reduce(&mut self) {
        while self.reduce_step() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            Node::Number(d) => *d,
            Node::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(d) => write!(f, "{}", d),
            Node::Pair(left, right) => write!(f, "[{}, {}]", left, right),
        }
    }
}

impl Add for Node {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Node::Pair(Box::new(self), Box::new(other))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<Node> = reader
        .lines()
        .map(|l| {
            let mut node_stack = VecDeque::<Node>::new();

            l.unwrap().chars().for_each(|c| {
                match c {
                    '[' | ',' => (),
                    ']' => {
                        // closing a pair
                        let n_right = node_stack.pop_back().unwrap();
                        let n_left = node_stack.pop_back().unwrap();
                        let n = Node::Pair(Box::new(n_left), Box::new(n_right));
                        node_stack.push_back(n);
                    }
                    c => {
                        if let Some(d) = c.to_digit(10) {
                            node_stack.push_back(Node::Number(d));
                        } else {
                            unimplemented!()
                        }
                    }
                }
            });

            assert!(node_stack.len() == 1);
            node_stack.pop_back().unwrap()
        })
        .collect();

    let max_mag = numbers[..numbers.len() - 2]
        .iter()
        .enumerate()
        .map(|(i, n1)| {
            numbers[i + 1..]
                .iter()
                .map(|n2| {
                    let mut ab = n1.clone() + n2.clone();
                    ab.reduce();
                    let mut ba = n2.clone() + n1.clone();
                    ba.reduce();
                    ab.magnitude().max(ba.magnitude())
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Maximum magnitude {}", max_mag);
}
