pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().fold(0, |acc, x| {
        let ranges = x
            .split(",")
            .map(|r| {
                let v = r
                    .split("-")
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<i32>>();

                std::ops::Range {
                    start: v[0],
                    end: v[1],
                }
            })
            .collect::<Vec<std::ops::Range<i32>>>();

        if contains_range(&ranges[0], &ranges[1]) {
            acc + 1
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input.lines().fold(0, |acc, x| {
        let ranges = x
            .split(",")
            .map(|r| {
                let v = r
                    .split("-")
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<i32>>();

                std::ops::Range {
                    start: v[0],
                    end: v[1],
                }
            })
            .collect::<Vec<std::ops::Range<i32>>>();

        if overlapping_range(&ranges[0], &ranges[1]) {
            acc + 1
        } else {
            acc
        }
    }))
}

fn contains_range(a: &std::ops::Range<i32>, b: &std::ops::Range<i32>) -> bool {
    a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
}

fn overlapping_range(a: &std::ops::Range<i32>, b: &std::ops::Range<i32>) -> bool {
    a.start <= b.end && b.start <= a.end
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
