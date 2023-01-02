use crate::file_reader::read_file;

fn part1(input: &String) {
    let res = input
        .lines()
        .map(|line| line.as_bytes())
        .filter_map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            right
                .iter()
                .find(|num| left.contains(*num))
                .map(|num| match num {
                    b'a'..=b'z' => (num - b'a') as u32 + 1,
                    _ => (num - b'A') as u32 + 1 + 26,
                })
        })
        .sum::<u32>()
        .to_string();

    println!("Part 1 - The sum of priorities are: {}", res);
}

pub fn run() {
    let input = read_file("input_03.txt");

    part1(&input);
}
