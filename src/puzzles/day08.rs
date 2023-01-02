use crate::file_reader::read_file;

fn part1(grid: &Vec<Vec<u32>>) {
    let mut res = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let height = grid[row][col];
            if grid[..row].iter().all(|x| x[col] < height)
                || grid[row][..col].iter().all(|x| x < &height)
                || grid[row + 1..].iter().all(|x| x[col] < height)
                || grid[row][col + 1..].iter().all(|x| x < &height)
            {
                res += 1;
            }
        }
    }

    println!("Part 1 - Result: {:?}", res);
}

fn part2(_: &Vec<Vec<u32>>) {
    // TODO
}

pub fn run() {
    let input = read_file("input_08.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    part1(&grid);
    part2(&grid);
}
