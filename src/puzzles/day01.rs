use crate::file_reader::read_file;

fn part1(elfs_calories: &Vec<Vec<u32>>) {
    let max_total_calories: u32 = elfs_calories
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap();
    println!("Part 1 - Total max calories: {:?}", max_total_calories);
}

fn part2(elfs_calories: &Vec<Vec<u32>>) {
    let mut cal_per_elf = elfs_calories
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();
    cal_per_elf.sort();
    let max_total_calories: u32 = cal_per_elf.iter().rev().take(3).sum();

    println!(
        "Part 2 - Total max of first 3 elfs: {:?}",
        max_total_calories
    );
}

pub fn run() {
    let input = read_file("input_01.txt");

    let calories = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|elf| elf.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|elf| {
            elf.iter()
                .filter_map(|&cal| cal.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    part1(&calories);
    part2(&calories);
}
