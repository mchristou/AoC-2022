use crate::file_reader::read_file;

fn part1(instr: &Vec<Instruction>) {
    let mut cycle = 0;
    let mut x = 1;
    let mut check = 20;
    let mut strength = 0;

    for instr in instr {
        match instr {
            Instruction::Noop => {
                cycle += 1;
                if cycle >= check {
                    strength += check * x;
                    check += 40;
                }
            }
            Instruction::Addx(v) => {
                cycle += 2;
                if cycle >= check {
                    strength += check * x;
                    check += 40;
                }
                x += v;
            }
        }
    }

    println!("Part 1 - Result: {:?}", strength);
}

fn pixel(cycle: u32, x: i32) -> char {
    if cycle as i32 % 40 > x + 1 || cycle as i32 % 40 < x - 1 {
        '.'
    } else {
        '#'
    }
}

fn part2(instr: Vec<Instruction>) {
    let mut cycle = 0;
    let mut x = 1;
    let mut check = 20;
    let mut ctr = "".to_string();
    let mut print_res = vec![];
    for instr in instr {
        match instr {
            Instruction::Noop => {
                if cycle % 40 == 0 {
                    print_res.push(ctr.to_string());
                    ctr = "".to_string();
                }

                ctr.push(pixel(cycle, x));

                cycle += 1;
                if cycle >= check {
                    check += 40;
                }
            }
            Instruction::Addx(v) => {
                if cycle % 40 == 0 {
                    print_res.push(ctr.to_string());
                    ctr = "".to_string();
                }

                ctr.push(pixel(cycle, x));
                cycle += 1;

                if cycle % 40 == 0 {
                    print_res.push(ctr.to_string());
                    ctr = "".to_string();
                }

                ctr.push(pixel(cycle, x));
                cycle += 1;

                if cycle >= check {
                    check += 40;
                }
                x += v;
            }
        }
    }

    print_res.push(ctr.to_string());
    println!("Part 2 - Result: \n");
    for line in print_res {
        println!("{line}");
    }
}

pub fn run() {
    let input = read_file("input_10.txt");
    let mut instuctions = vec![];

    for line in input.lines() {
        let splitted_line: Vec<_> = line.split(' ').collect();
        match splitted_line[0] {
            "noop" => instuctions.push(Instruction::Noop),
            "addx" => instuctions.push(Instruction::Addx(splitted_line[1].parse().unwrap())),
            _ => panic!("Invalid instruction"),
        };
    }

    part1(&instuctions);
    part2(instuctions);
}

enum Instruction {
    Noop,
    Addx(i32),
}
