use crate::file_reader::read_file;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn part1(stacks: Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    let mut stacks = stacks;

    for instr in instructions {
        for _ in 0..instr.amount {
            if let Some(moved) = stacks[instr.from].pop() {
                stacks[instr.to].push(moved);
            }
        }
    }

    let res = stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect::<Vec<char>>();

    println!("Part 1 - Result: {:?}", res);
}

fn part2(stacks: Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    let mut stacks = stacks;

    for instr in instructions {
        let stack_len = stacks[instr.from].len();
        let removed = stacks[instr.from].split_off(stack_len - instr.amount);
        stacks[instr.to].extend(removed);
    }

    let res = stacks
        .into_iter()
        .filter_map(|stack| stack.into_iter().last())
        .collect::<Vec<char>>();

    println!("Part 2 - Result: {:?}", res);
}

pub fn run() {
    let input = read_file("input_05.txt");
    let (stacks, instructions_input) = input.split_once("\n\n").unwrap();
    let (stacks_input, platforms) = stacks.rsplit_once('\n').unwrap();

    let num_stacks = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .ok()
        .unwrap();

    let mut instructions = Vec::new();
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stacks_input.lines().rev() {
        for (idx, chunk) in line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .into_iter()
            .enumerate()
        {
            let second = chunk.get(1).cloned().unwrap(); // TODO: remove clone
            if second.is_alphabetic() {
                stacks[idx].push(second);
            }
        }
    }

    for line in instructions_input.lines() {
        let rest = line.strip_prefix("move ").unwrap();
        let (amount, rest) = rest.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let instruction = Instruction {
            amount: amount.parse().ok().unwrap(),
            from: from.parse::<usize>().ok().unwrap() - 1,
            to: to.parse::<usize>().ok().unwrap() - 1,
        };
        instructions.push(instruction);
    }

    part1(stacks.clone(), &instructions); // TODO: remove clone
    part2(stacks, &instructions);
}
