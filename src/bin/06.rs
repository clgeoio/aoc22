use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .enumerate()
            .find(|(_i, slice)| slice.iter().all_unique())
            .unwrap()
            .0
            + 4,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(14)
            .enumerate()
            .find(|(_i, slice)| slice.iter().all_unique())
            .unwrap()
            .0
            + 14,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
