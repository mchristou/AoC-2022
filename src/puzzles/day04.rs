use crate::file_reader::read_file;

fn part1(input: &Vec<[[u32; 2]; 2]>) {
    let res = input
        .iter()
        .filter(|[[min1, max1], [min2, max2]]| {
            (min1 >= min2 && max1 <= max2) || (min2 >= min1 && max2 <= max1)
        })
        .count()
        .to_string();

    println!("Part 1 - Result: {}", res);
}

fn part2(input: &Vec<[[u32; 2]; 2]>) {
    let res = input
        .iter()
        .filter(|[[min1, max1], [min2, max2]]| min1 <= max2 && max1 >= min2)
        .count()
        .to_string();

    println!("Part 2 - Result: {}", res);
}
pub fn run() {
    let input = read_file("input_04.txt");
    let mut pairs = Vec::new();
    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',').unwrap();
        let (min1, max1) = elf1.split_once('-').unwrap();
        let (min2, max2) = elf2.split_once('-').unwrap();
        pairs.push([
            [min1.parse::<u32>().unwrap(), max1.parse::<u32>().unwrap()],
            [min2.parse::<u32>().unwrap(), max2.parse::<u32>().unwrap()],
        ]);
    }

    part1(&pairs);
    part2(&pairs);
}
