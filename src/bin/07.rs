use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: HashMap<String, Rc<RefCell<Self>>>,
    file_sizes: Vec<usize>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Dir {
            name: name.to_string(),
            subdirs: HashMap::new(),
            file_sizes: Vec::new(),
        }
    }

    fn size(&self) -> usize {
        self.file_sizes.iter().sum::<usize>()
            + self
                .subdirs
                .iter()
                .map(|(_, dir)| dir.borrow().size())
                .sum::<usize>()
    }

    fn size_if_under_100_000(&self) -> usize {
        let actual_size = self.size();
        let size_below_me = self
            .subdirs
            .iter()
            .map(|(_, dir)| dir.borrow().size_if_under_100_000())
            .sum::<usize>();

        if actual_size <= 100_000 {
            actual_size + size_below_me
        } else {
            size_below_me
        }
    }

    fn get_all_sizes(&self) -> Vec<usize> {
        let mut result = vec![self.size()];
        for (_, dir) in &self.subdirs {
            result.extend(dir.borrow().get_all_sizes());
        }
        result
    }

    fn from(input: &str) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Dir::new("/")));
        let mut current = vec![Rc::clone(&root)];

        for line in input.lines() {
            let args = line.split_ascii_whitespace().collect::<Vec<&str>>();
            match args[..] {
                ["$", "cd", "/"] => {
                    current.truncate(1);
                }
                ["$", "cd", ".."] => {
                    current.pop();
                }
                ["$", "cd", directory] => {
                    let new = Rc::clone(
                        current
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .subdirs
                            .get(directory)
                            .unwrap(),
                    );
                    current.push(new);
                }
                ["$", "ls"] => (),
                ["dir", name] => {
                    current
                        .last_mut()
                        .unwrap()
                        .borrow_mut()
                        .subdirs
                        .insert(String::from(name), Rc::new(RefCell::new(Dir::new(name))));
                }
                [size, _name] => {
                    current
                        .last_mut()
                        .unwrap()
                        .borrow_mut()
                        .file_sizes
                        .push(size.parse::<usize>().unwrap_or(0));
                }
                _ => (),
            }
        }

        root
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let p = Dir::from(input);
    let x = p.borrow();
    Some(x.size_if_under_100_000())
}

pub fn part_two(input: &str) -> Option<usize> {
    let p = Dir::from(input);
    let total_size = p.borrow().size();
    let space_needed = total_size - 40_000_000;
    let mut all_sizes: Vec<usize> = p
        .borrow()
        .get_all_sizes()
        .into_iter()
        .filter(|x| *x > space_needed)
        .collect();
    all_sizes.sort();
    Some(all_sizes[0])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
