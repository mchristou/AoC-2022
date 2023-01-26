use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::file_reader::read_file;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn move_head(current: &Point, new: &Point) -> Point {
    match (new.x, new.y) {
        (1, 0) => Point::new(current.x + 1, current.y), // RIGHT
        (-1, 0) => Point::new(current.x - 1, current.y), // LEFT
        (0, 1) => Point::new(current.x, current.y + 1), // UP
        (0, -1) => Point::new(current.x, current.y - 1), // DOWN
        _ => unreachable!(),
    }
}

fn move_tail(head: &Point, tail: &Point) -> Point {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    // Do not do this!!!
    //
    // let dx = dx.abs();
    // let dy = dy.abs();
    //

    if dx.abs() + dy.abs() > 2 || dx.abs() > 1 || dy.abs() > 1 {
        match dx.abs().cmp(&dy.abs()) {
            std::cmp::Ordering::Equal => {
                return Point {
                    x: head.x - dx.signum(),
                    y: head.y - dy.signum(),
                };
            }
            Ordering::Greater => {
                return Point {
                    x: head.x - dx.signum(),
                    y: head.y,
                };
            }
            _ => {
                return Point {
                    x: head.x,
                    y: head.y - dy.signum(),
                };
            }
        };
    }

    *tail
}

fn part1(commands: &[(Point, i32)]) {
    let mut points_visited = std::collections::HashMap::new();

    let mut head_point = Point::new(0, 0);
    let mut tail_point = Point::new(0, 0);
    points_visited.insert(tail_point, 0);

    for command in commands.iter() {
        for _ in 0..command.1 {
            head_point = move_head(&head_point, &command.0);
            tail_point = move_tail(&head_point, &tail_point);

            points_visited.insert(tail_point, 0);
        }
    }

    let res = points_visited.len();
    println!("Part 1 - Result: {:?}", res);
}

pub fn run() {
    let input = read_file("input_09.txt");

    let commands = input
        .lines()
        .map(|line| {
            let (direction, steps) = line.split_once(' ').unwrap();
            let point = match direction {
                "R" => Point::new(1, 0),
                "L" => Point::new(-1, 0),
                "U" => Point::new(0, 1),
                "D" => Point::new(0, -1),
                _ => unreachable!(),
            };
            let steps = steps.parse::<i32>().unwrap();

            (point, steps)
        })
        .collect::<Vec<(Point, i32)>>();

    part1(&commands);
}
