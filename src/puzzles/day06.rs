use crate::file_reader::read_file;

fn diff(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(|w| w.iter().enumerate().all(|(idx, c)| !w[..idx].contains(c)))
        .unwrap_or(input.len())
        + size
}

fn part1(input: &str, size: usize) {
    let res = diff(input, size);

    println!("Part 1 - Result: {:?}", res);
}

fn part2(input: &str, size: usize) {
    let res = diff(input, size);

    println!("Part 2- Result: {:?}", res);
}
pub fn run() {
    let input = read_file("input_06.txt");

    part1(&input, 4);
    part2(&input, 14);
}
