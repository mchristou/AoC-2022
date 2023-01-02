use std::collections::HashMap;
use std::path::PathBuf;

use crate::file_reader::read_file;

fn parse(input: &str) -> HashMap<PathBuf, u32> {
    let mut sizes = HashMap::new();
    let mut paths = Vec::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => {}
            ["dir", _] => {}
            ["$", "cd", ".."] => {
                paths.pop();
            }
            ["$", "cd", name] => {
                paths.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..paths.len() {
                    let path = PathBuf::from_iter(&paths[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }
    sizes
}

fn part1(input: &str) {
    let res = parse(&input)
        .iter()
        .filter_map(|s| {
            if *s.1 <= 100_000 {
                return Some(s.1);
            }

            None
        })
        .sum::<u32>();

    println!("Part 1 - Result: {:?}", res);
}

fn part2(input: &str) {
    let disk = 70_000_000;
    let needed = 30_000_000;
    let sizes = parse(&input);
    let used_space = sizes.get(&PathBuf::from("/")).unwrap();
    let empty_space = disk - used_space;

    let res = sizes
        .iter()
        .filter_map(|s| {
            if empty_space + *s.1 >= needed {
                return Some(s.1);
            }
            None
        })
        .min()
        .unwrap();

    println!("Part 2 - Result: {:?}", res);
}

pub fn run() {
    let input = read_file("input_07.txt");

    part1(&input);
    part2(&input);
}
