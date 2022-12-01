pub fn part_one(input: &str) -> Option<i32> {
    let mut split = input
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split("\n")
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    split.sort();
    split.pop()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut split = input
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split("\n")
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    split.sort_by(|a, b| b.cmp(a));
    Some(split.into_iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
