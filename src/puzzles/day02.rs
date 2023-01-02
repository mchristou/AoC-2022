use crate::file_reader::read_file;

#[derive(Debug)]
enum Op {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Me {
    Rock,
    Paper,
    Scissors,
}

fn part1(input: &Vec<(Op, Me)>) {
    let score: u32 = input
        .iter()
        .map(|(op, me)| {
            let res = match (op, me) {
                (Op::Rock, Me::Paper) | (Op::Paper, Me::Scissors) | (Op::Scissors, Me::Rock) => 6, // win
                (Op::Rock, Me::Rock) | (Op::Paper, Me::Paper) | (Op::Scissors, Me::Scissors) => 3, // draw
                _ => 0, // lost
            };

            let points = match me {
                Me::Rock => 1,
                Me::Paper => 2,
                Me::Scissors => 3,
            };

            res + points
        })
        .sum();

    println!("Part 1 - Your score is: {}", score);
}

fn part2(input: &Vec<(Op, Me)>) {
    let score: u32 = input
        .iter()
        .map(|(op, me)| match (op, me) {
            (Op::Paper, Me::Paper) => 2 + 3,
            (Op::Scissors, Me::Paper) => 3 + 3,
            (Op::Rock, Me::Paper) => 1 + 3,

            (Op::Rock, Me::Rock) => 3,
            (Op::Paper, Me::Rock) => 1,
            (Op::Scissors, Me::Rock) => 2,

            (Op::Rock, Me::Scissors) => 2 + 6,
            (Op::Paper, Me::Scissors) => 3 + 6,
            (Op::Scissors, Me::Scissors) => 1 + 6,
        })
        .sum();

    println!("Part 2 - Your score is: {}", score);
}

pub fn run() {
    let input = read_file("input_02.txt");

    let op: Vec<_> = input
        .lines()
        .flat_map(|v| v.split_once(' '))
        .map(|(op, me)| {
            let opponents = match op {
                "A" => Op::Rock,
                "B" => Op::Paper,
                "C" => Op::Scissors,
                _ => unreachable!(),
            };

            let mine = match me {
                "X" => Me::Rock,
                "Y" => Me::Paper,
                "Z" => Me::Scissors,
                _ => unreachable!(),
            };

            (opponents, mine)
        })
        .collect();

    part1(&op);
    part2(&op);
}
