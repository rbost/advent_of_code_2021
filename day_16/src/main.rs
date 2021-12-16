use std::io::{Cursor, Read};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    content: PacketContent,
}

#[derive(Debug)]
enum PacketContent {
    Number(u64),
    Operator(Vec<Packet>),
}

impl Packet {
    fn get_sum_version(&self) -> u64 {
        match &self.content {
            PacketContent::Operator(v) => {
                self.version + v.iter().map(|p| p.get_sum_version()).sum::<u64>()
            }
            _ => self.version,
        }
    }

    fn compute(&self) -> u64 {
        match &self.content {
            PacketContent::Operator(v) => {
                let values: Vec<_> = v.iter().map(|p| p.compute()).collect();

                match self.type_id {
                    0 => values.iter().sum::<u64>(),
                    1 => values.iter().product::<u64>(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 => {
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            PacketContent::Number(v) => *v,
        }
    }
}

fn char_to_bits(c: char) -> Vec<u8> {
    match c {
        '0' => vec![0, 0, 0, 0],
        '1' => vec![0, 0, 0, 1],
        '2' => vec![0, 0, 1, 0],
        '3' => vec![0, 0, 1, 1],
        '4' => vec![0, 1, 0, 0],
        '5' => vec![0, 1, 0, 1],
        '6' => vec![0, 1, 1, 0],
        '7' => vec![0, 1, 1, 1],
        '8' => vec![1, 0, 0, 0],
        '9' => vec![1, 0, 0, 1],
        'A' => vec![1, 0, 1, 0],
        'B' => vec![1, 0, 1, 1],
        'C' => vec![1, 1, 0, 0],
        'D' => vec![1, 1, 0, 1],
        'E' => vec![1, 1, 1, 0],
        'F' => vec![1, 1, 1, 1],
        _ => unimplemented!(),
    }
}

fn read_integer(n_bits: usize, cursor: &mut Cursor<&[u8]>) -> u64 {
    let mut buf = vec![2; n_bits];
    println!("Pos {}", cursor.position());
    cursor.read_exact(&mut buf).unwrap();

    buf.iter().fold(0u64, |acc, v| acc * 2 + (*v as u64))
}

fn read_number_chunk(cursor: &mut Cursor<&[u8]>) -> (u64, bool) {
    let data = read_integer(5, cursor);

    (data & 15, data < 16)
}

fn read_number(cursor: &mut Cursor<&[u8]>) -> u64 {
    let mut acc = 0;
    loop {
        let (v, is_last) = read_number_chunk(cursor);

        acc = acc * 16 + v;
        if is_last {
            break;
        }
    }
    acc
}

fn parse_packet(cursor: &mut Cursor<&[u8]>) -> Packet {
    // parse the version
    let version = read_integer(3, cursor);
    let type_id = read_integer(3, cursor);

    if type_id == 4 {
        // read a number
        println!("Read a number v = {}", version);

        let n = read_number(cursor);
        Packet {
            version,
            type_id,
            content: PacketContent::Number(n),
        }
    } else {
        let length_type_id = read_integer(1, cursor);
        println!(
            "Read a packet v = {}, type = {}, length_type = {}",
            version, type_id, length_type_id
        );

        if length_type_id == 0 {
            let total_length = read_integer(15, cursor);
            let mut subbuffer = vec![2; total_length as usize];
            cursor.read_exact(&mut subbuffer).unwrap();

            let mut sub_cursor = Cursor::new(&subbuffer[..]);
            let mut subpackets = Vec::<Packet>::new();

            while sub_cursor.position() < total_length {
                let p = parse_packet(&mut sub_cursor);
                subpackets.push(p);
            }

            Packet {
                version,
                type_id,
                content: PacketContent::Operator(subpackets),
            }
        } else {
            let n_packets = read_integer(11, cursor);
            println!("# subpackets = {}", n_packets);

            let mut subpackets = Vec::<Packet>::new();

            for _i in 0..n_packets {
                let p = parse_packet(cursor);
                subpackets.push(p);
            }
            Packet {
                version,
                type_id,
                content: PacketContent::Operator(subpackets),
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

    let data: Vec<u8> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .flat_map(char_to_bits)
        .inspect(|v| assert!(*v < 2))
        .collect();

    println!("Buffer {:?}", data);

    let mut cursor = Cursor::new(&data[..]);

    let packet = parse_packet(&mut cursor);

    println!("Packet {:?}", packet);
    println!("Sum version {}", packet.get_sum_version());
    println!("Result {}", packet.compute());
}
