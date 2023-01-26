use std::cmp::Ordering;

use crate::file_reader::read_file;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (self::Packet::Num(a), Self::Num(b)) => a.cmp(b),
            (self::Packet::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(b),
            (self::Packet::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (self::Packet::List(a), Self::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(pairs: &[(Packet, Packet)]) {
    let mut res = 0;
    for (idx, pair) in pairs.iter().enumerate() {
        if pair.0 < pair.1 {
            res += idx + 1; // The plus one is needed cz enumerate is starting from 0
        }
    }

    println!("Part 1 - Result: {}", res);
}

fn part2(pairs: Vec<(Packet, Packet)>) {
    let mut flat_pairs = Vec::new();
    for pair in pairs.into_iter() {
        flat_pairs.push(pair.0);
        flat_pairs.push(pair.1);
    }
    let div_2 = parse_packet(&mut "[2]]".chars());
    let div_6 = parse_packet(&mut "[6]]".chars());

    flat_pairs.push(div_2.clone());
    flat_pairs.push(div_6.clone());
    flat_pairs.sort_unstable();

    let mut div_2_idx = 0;
    let mut div_6_idx = 0;
    for (idx, packet) in flat_pairs.iter().enumerate() {
        if *packet == div_2 {
            div_2_idx = idx + 1;
        }
        if *packet == div_6 {
            div_6_idx = idx + 1;
        }
    }

    println!("Part 2 - Result: {}", div_2_idx * div_6_idx);
}

pub fn run() {
    let input = read_file("input_13.txt");

    let posible_packets: Vec<_> = input.split("\n\n").collect();
    let mut pairs = vec![];
    for packet in posible_packets {
        let (left, right) = packet.split_once('\n').unwrap();
        let mut left = left.chars();
        let _ = left.next().unwrap(); // consuming the first '['

        let mut right = right.chars();
        let _ = right.next().unwrap(); // consuming the first '['

        let left = parse_packet(&mut left);
        let right = parse_packet(&mut right);
        pairs.push((left, right));
    }

    part1(&pairs);
    part2(pairs);
}

fn parse_packet(packet: &mut std::str::Chars) -> Packet {
    let mut res = Vec::new();
    let mut num = -1;
    while let Some(ch) = packet.next() {
        match ch {
            '[' => res.push(parse_packet(packet)),
            ']' => {
                if num > -1 {
                    res.push(Packet::Num(num));
                }

                return Packet::List(res);
            }
            ',' => {
                if num > -1 {
                    res.push(Packet::Num(num));
                    num = -1;
                }
            }
            '0'..='9' => {
                if num < 0 {
                    num = (ch as u8 - b'0') as i32;
                } else {
                    num = (num * 10) + (ch as u8 - b'0') as i32;
                }
            }
            _ => panic!("Invalid data"),
        }
    }
    Packet::List(res)
}
