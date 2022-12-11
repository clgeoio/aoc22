use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

struct Step {
    dir: Direction,
    count: usize,
}

#[derive(Debug)]
struct State {
    tail_visited: HashSet<Coord>,
    rope: Vec<Coord>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut s = State::new(1);

    input
        .lines()
        .map(|s| {
            let (cmd, rest) = s.split_at(1);
            (cmd, str::parse::<usize>(rest.trim()).unwrap())
        })
        .map(|x| match x {
            ("U", count) => Step {
                dir: Direction::Up,
                count,
            },
            ("R", count) => Step {
                dir: Direction::Right,
                count,
            },
            ("L", count) => Step {
                dir: Direction::Left,
                count,
            },
            ("D", count) => Step {
                dir: Direction::Down,
                count,
            },
            _ => panic!("Unexpected cmd {:?}", x),
        })
        .for_each(|step| s.process(step));

    Some(s.tail_visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut s = State::new(10);

    input
        .lines()
        .map(|s| {
            let (cmd, rest) = s.split_at(1);
            (cmd, str::parse::<usize>(rest.trim()).unwrap())
        })
        .map(|x| match x {
            ("U", count) => Step {
                dir: Direction::Up,
                count,
            },
            ("R", count) => Step {
                dir: Direction::Right,
                count,
            },
            ("L", count) => Step {
                dir: Direction::Left,
                count,
            },
            ("D", count) => Step {
                dir: Direction::Down,
                count,
            },
            _ => panic!("Unexpected cmd {:?}", x),
        })
        .for_each(|step| s.process(step));

    Some(s.tail_visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

impl State {
    fn new(len: usize) -> Self {
        State {
            rope: vec![Coord { x: 0, y: 0 }; len + 1],
            tail_visited: Default::default(),
        }
    }

    fn process(&mut self, step: Step) {
        for _i in 0..step.count {
            let head = self.rope[0];
            let new_head = match step.dir {
                Direction::Up => Coord {
                    x: head.x,
                    y: head.y + 1,
                },
                Direction::Down => Coord {
                    x: head.x,
                    y: head.y - 1,
                },
                Direction::Left => Coord {
                    x: head.x - 1,
                    y: head.y,
                },
                Direction::Right => Coord {
                    x: head.x + 1,
                    y: head.y,
                },
            };
            self.rope[0] = new_head;

            for segment in 1..self.rope.len() {
                let h = self.rope[segment - 1];
                let t = self.rope[segment];

                let diff = h - t;
                let (dx, dy) = match (diff.x, diff.y) {
                    // overlapping
                    (0, 0) => (0, 0),
                    // touching up/left/down/right
                    (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                    // touching diagonally
                    (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                    // need to move up/left/down/right
                    (0, 2) => (0, 1),
                    (0, -2) => (0, -1),
                    (2, 0) => (1, 0),
                    (-2, 0) => (-1, 0),
                    // need to move to the right diagonally
                    (2, 1) => (1, 1),
                    (2, -1) => (1, -1),
                    // need to move to the left diagonally
                    (-2, 1) => (-1, 1),
                    (-2, -1) => (-1, -1),
                    // need to move up/down diagonally
                    (1, 2) => (1, 1),
                    (-1, 2) => (-1, 1),
                    (1, -2) => (1, -1),
                    (-1, -2) => (-1, -1),
                    (2, 2) => (1, 1),
                    _ => panic!("unhandled case: tail - head = {diff:?}"),
                };

                self.rope[segment].x += dx;
                self.rope[segment].y += dy;

                if segment == self.rope.len() - 1 {
                    println!("segment {:?}", self.rope[segment]);
                    self.tail_visited.insert(self.rope[segment]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
