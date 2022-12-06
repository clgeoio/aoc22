use itertools::Itertools;

struct Stack {
    stacks: Vec<Vec<char>>,
}

impl Stack {
    fn from(ps: &str) -> Self {
        let n = ps.chars().rev().skip(3).take(1).collect::<Vec<char>>();
        let size = n[0].to_digit(10).unwrap();

        // println!("parsed size {:?}", size);

        let mut stacks: Vec<Vec<char>> = vec![Default::default(); size as usize];

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
                    .for_each(|(i, c)| stacks[i].push(c))
            });

        Stack { stacks }
    }

    fn process_one(self, ins: Instruction) -> Self {
        let mut tmp_stack = self.stacks;

        for _ in 0..ins.n {
            let tmp = tmp_stack[ins.from - 1].pop();
            tmp_stack[ins.to - 1].push(tmp.unwrap())
        }

        Stack { stacks: tmp_stack }
    }

    fn process_multi(self, ins: Instruction) -> Self {
        let mut tmp_stack = self.stacks;
        // println!("move {:?} from {:?} to {:?}", n, a, b);

        let mut tmp: Vec<char> = Default::default();

        // println!("move {:?} from {:?} to {:?}", n, a, b);
        for _ in 0..ins.n {
            tmp.push(tmp_stack[ins.from - 1].pop().unwrap());
        }

        tmp.reverse();
        tmp_stack[ins.to - 1].append(&mut (tmp));

        Stack { stacks: tmp_stack }
    }
}

struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(input: &str) -> Self {
        let (n, from, to): (usize, _, _) = input
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();

        Instruction { n, from, to }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (ps, ins) = input.split_at(
        input
            .as_bytes()
            .windows(2)
            .position(|x| x == b"\n\n")
            .unwrap()
            + 2,
    );
    let s = Stack::from(ps);
    // println!("parsed stacks {:?}", stack);
    let instructions = ins.split("\n").map(Instruction::from);
    let last = instructions.fold(s, |acc, x| acc.process_one(x));
    let ff = last
        .stacks
        .iter()
        .map(|s| s.last())
        .map(Option::unwrap)
        .join("");

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
    let s = Stack::from(ps);
    // println!("parsed stacks {:?}", stack);
    let instructions = ins.split("\n").map(Instruction::from);
    let last = instructions.fold(s, |acc, x| acc.process_multi(x));
    let ff = last
        .stacks
        .iter()
        .map(|s| s.last())
        .map(Option::unwrap)
        .join("");

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
