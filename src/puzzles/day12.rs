use crate::file_reader::read_file;
use std::collections::hash_set::HashSet;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut neigh = Vec::new();

        if self.y > 0 {
            neigh.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y < rows - 1 {
            neigh.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }

        if self.x > 0 {
            neigh.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x < cols - 1 {
            neigh.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }

        neigh
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    start: Coord,
    end: Coord,
    map: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

fn shortest_path(maze: &Maze) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut posible_nodes = BinaryHeap::new();

    posible_nodes.push(Node {
        cost: 0,
        coord: maze.start,
    });
    visited.insert(maze.start);

    while let Some(Node { cost, coord }) = posible_nodes.pop() {
        if coord == maze.end {
            return Some(cost);
        }

        let current_height = maze.map[coord.y][coord.x];
        let neigh = coord.neighbours(maze.rows, maze.cols);

        let posible_steps: Vec<_> = neigh
            .iter()
            .filter(|coord| {
                let height = maze.map[coord.y][coord.x];
                height <= current_height || height == current_height + 1
            })
            .collect();

        for ps in posible_steps {
            if visited.insert(*ps) {
                posible_nodes.push(Node {
                    cost: cost + 1,
                    coord: *ps,
                })
            }
        }
    }
    None
}

fn part1(maze: &Maze) {
    println!("Part 1  Result: {:?}", shortest_path(maze));
}

fn part2(maze: Maze, all_a: Vec<Coord>) {
    let mut maze = maze;

    let res = all_a
        .iter()
        .filter_map(|coord| {
            maze.start = *coord;
            shortest_path(&maze)
        })
        .min();

    println!("Part 2  Result: {:?}", res);
}

pub fn run() {
    let input = read_file("input_12.txt");

    let rows = input.lines().count();
    let cols = input.lines().last().unwrap().len();
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };
    let mut map = vec![vec![0; cols]; rows];
    let mut all_a: Vec<Coord> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let letter = match ch {
                'a' => {
                    all_a.push(Coord { x: col, y: row });
                    ch
                }
                'S' => {
                    start.x = col;
                    start.y = row;
                    'a'
                }
                'E' => {
                    end.x = col;
                    end.y = row;
                    'z'
                }

                'a'..='z' => ch,
                _ => panic!("Invalid input"),
            };

            map[row][col] = letter as u8 - b'a';
        }
    }

    let maze = Maze {
        start,
        end,
        map,
        rows,
        cols,
    };

    part1(&maze);
    part2(maze, all_a);
}
