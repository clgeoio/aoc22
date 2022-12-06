use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let (ps, ins) = input.split_at(
        input
            .as_bytes()
            .windows(2)
            .position(|x| x == b"\n\n")
            .unwrap()
            + 2,
    );

    let n = ps.chars().rev().skip(3).take(1).collect::<Vec<char>>();
    let size = n[0].to_digit(10).unwrap();

    // println!("parsed size {:?}", size);

    let mut stack: Vec<Vec<char>> = vec![Default::default(); size as usize];

    ps.split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .skip(3)
        .for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_i, c)| c != &' ')
                .for_each(|(i, c)| stack[i].push(c))
        });

    // println!("parsed stacks {:?}", stack);

    ins.split("\n").for_each(|i| {
        let (n, a, b): (usize, _, _) = i
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();

        // println!("move {:?} from {:?} to {:?}", n, a, b);
        for _ in 0..n {
            let tmp = stack[a - 1].pop();
            stack[b - 1].push(tmp.unwrap())
        }

        // println!("stack {:?}", stack);
    });

    let ff = stack.iter().map(|s| s.last()).map(Option::unwrap).join("");

    Some(ff)
}

pub fn part_two(input: &str) -> Option<String> {
    let (ps, ins) = input.split_at(
        input
            .as_bytes()
            .windows(2)
            .position(|x| x == b"\n\n")
            .unwrap()
            + 2,
    );

    let n = ps.chars().rev().skip(3).take(1).collect::<Vec<char>>();
    let size = n[0].to_digit(10).unwrap();

    // println!("parsed size {:?}", size);

    let mut stack: Vec<Vec<char>> = vec![Default::default(); size as usize];

    ps.split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .skip(3)
        .for_each(|l| {
            l.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_i, c)| c != &' ')
                .for_each(|(i, c)| stack[i].push(c))
        });

    // println!("parsed stacks {:?}", stack);

    ins.split("\n").for_each(|i| {
        let (n, a, b): (usize, _, _) = i
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();

        // println!("move {:?} from {:?} to {:?}", n, a, b);

        let mut tmp: Vec<char> = Default::default();

        // println!("move {:?} from {:?} to {:?}", n, a, b);
        for _ in 0..n {
            tmp.push(stack[a - 1].pop().unwrap());
        }

        tmp.reverse();
        stack[b - 1].append(&mut (tmp));
    });

    let ff = stack.iter().map(|s| s.last()).map(Option::unwrap).join("");

    Some(ff)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
