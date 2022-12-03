use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|x| x.split_at(x.len() / 2).shared_value())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .tuples::<(_, _, _)>()
            .map(|x| x.shared_value())
            .sum(),
    )
}

pub trait TupleSharedValue {
    fn shared_value(&self) -> u32;
}

impl TupleSharedValue for (&str, &str) {
    fn shared_value(&self) -> u32 {
        self.0
            .chars()
            .find(|&char| self.1.contains(char))
            .unwrap()
            .priority()
    }
}

impl TupleSharedValue for (&str, &str, &str) {
    fn shared_value(&self) -> u32 {
        self.0
            .chars()
            .find(|&char| self.1.contains(char) && self.2.contains(char))
            .unwrap()
            .priority()
    }
}

pub trait CharPriority {
    fn priority(&self) -> u32;
}
impl CharPriority for char {
    fn priority(&self) -> u32 {
        let is_upper = self.is_uppercase();
        return (self.to_ascii_lowercase() as u32) - 96 + if is_upper { 26 } else { 0 };
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
