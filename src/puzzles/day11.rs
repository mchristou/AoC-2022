use crate::file_reader::read_file;

fn part1(mut monkeys: Vec<Monkey>) {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys[idx].clone();
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            for item in items {
                inspections[idx] += 1;

                let new = match &monkey.operation {
                    Operation::Add(val) => match val {
                        Value::Num(v) => item + v,
                        Value::Old => item + item,
                    },
                    Operation::Multiply(val) => match val {
                        Value::Num(v) => item * v,
                        Value::Old => item * item,
                    },
                };

                let new = new / 3;

                let index = if new % monkey.test.divisible == 0 {
                    monkey.test.true_case
                } else {
                    monkey.test.false_case
                };

                let receiver = &mut monkeys[index];
                receiver.items.push(new);
            }
        }
    }

    inspections.sort_unstable();
    let res: usize = inspections.iter().rev().take(2).product();
    println!("Part 1 - Result: {:?}", res);
}

fn part2(mut monkeys: Vec<Monkey>) {
    let mut inspections = vec![0; monkeys.len()];
    let common_mod: u64 = monkeys.iter().map(|m| m.test.divisible).product();

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            let monkey = monkeys[idx].clone();
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            for item in items {
                inspections[idx] += 1;

                let new = match &monkey.operation {
                    Operation::Add(val) => match val {
                        Value::Num(v) => item + v,
                        Value::Old => item + item,
                    },
                    Operation::Multiply(val) => match val {
                        Value::Num(v) => item * v,
                        Value::Old => item * item,
                    },
                };

                let new = new % common_mod;

                let index = if new % monkey.test.divisible == 0 {
                    monkey.test.true_case
                } else {
                    monkey.test.false_case
                };

                let receiver = &mut monkeys[index];
                receiver.items.push(new);
            }
        }
    }
    inspections.sort_unstable();
    let res: usize = inspections.iter().rev().take(2).product();
    println!("Part 2 - Result: {:?}", res);
}
pub fn run() {
    let input = read_file("input_11.txt");
    let mut monkeys = vec![];

    for monkey in input.split("\n\n") {
        let mut lines = monkey.lines().skip(1);

        let items: Vec<u64> = lines
            .next()
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|i| i.trim().parse().ok())
            .collect();

        let (_, operation) = lines.next().unwrap().split_once("= old").unwrap();
        let (operator, operand) = operation.trim().split_once(" ").unwrap();
        let value = match operand {
            "old" => Value::Old,
            _ => {
                let num = operand.parse().unwrap();
                Value::Num(num)
            }
        };
        let operation = match operator {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!("Invalid Operator"),
        };

        let divisible = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(3)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let true_case = lines
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        let false_case = lines
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test: Test {
                divisible,
                true_case,
                false_case,
            },
        });
    }

    part1(monkeys.clone());
    part2(monkeys);
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
struct Test {
    divisible: u64,
    true_case: usize,
    false_case: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Value),
    Multiply(Value),
}

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u64),
}
