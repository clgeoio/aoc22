// A Rock
// B Papger
// C Sc

// X Rock

#[derive(Debug)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum PlayerOutcome {
    Lose,
    Draw,
    Win,
}

impl PlayerMove {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => PlayerMove::Rock,
            "B" | "Y" => PlayerMove::Paper,
            "C" | "Z" => PlayerMove::Scissors,
            _ => panic!("Unexpected char {}", s),
        }
    }

    fn value(&self) -> i32 {
        return match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
    }

    fn outcome(&self, other: PlayerMove) -> i32 {
        let value = self.value()
            + match self {
                Self::Rock => match other {
                    PlayerMove::Rock => PlayerOutcome::Draw,
                    PlayerMove::Paper => PlayerOutcome::Lose,
                    PlayerMove::Scissors => PlayerOutcome::Win,
                },
                Self::Paper => match other {
                    PlayerMove::Rock => PlayerOutcome::Win,
                    PlayerMove::Paper => PlayerOutcome::Draw,
                    PlayerMove::Scissors => PlayerOutcome::Lose,
                },
                Self::Scissors => match other {
                    PlayerMove::Rock => PlayerOutcome::Lose,
                    PlayerMove::Paper => PlayerOutcome::Win,
                    PlayerMove::Scissors => PlayerOutcome::Draw,
                },
            }
            .value();

        //println!("{:?} plays {:?} outcome {}", self, other, value);
        value
    }
}

impl PlayerOutcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => PlayerOutcome::Lose,
            "Y" => PlayerOutcome::Draw,
            "Z" => PlayerOutcome::Win,
            _ => panic!("Unexpected char {}", s),
        }
    }

    fn value(&self) -> i32 {
        return match self {
            Self::Lose => 0,
            Self::Win => 6,
            Self::Draw => 3,
        };
    }

    fn outcome(&self, other: PlayerMove) -> i32 {
        let value = self.value()
            + match self {
                Self::Lose => match other {
                    PlayerMove::Rock => PlayerMove::Scissors,
                    PlayerMove::Paper => PlayerMove::Rock,
                    PlayerMove::Scissors => PlayerMove::Paper,
                },
                Self::Draw => match other {
                    PlayerMove::Rock => PlayerMove::Rock,
                    PlayerMove::Paper => PlayerMove::Paper,
                    PlayerMove::Scissors => PlayerMove::Scissors,
                },
                Self::Win => match other {
                    PlayerMove::Rock => PlayerMove::Paper,
                    PlayerMove::Paper => PlayerMove::Scissors,
                    PlayerMove::Scissors => PlayerMove::Rock,
                },
            }
            .value();

        //println!("{:?} plays {:?} outcome {}", self, other, value);
        value
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let round: i32 = input
        .lines()
        .map(|x| {
            let plays = x.split(" ").collect::<Vec<&str>>();

            let op = PlayerMove::from(plays[0]);
            let me = PlayerMove::from(plays[1]);
            return me.outcome(op);
        })
        .sum();

    Some(round)
}

pub fn part_two(input: &str) -> Option<i32> {
    let round: i32 = input
        .lines()
        .map(|x| {
            let plays = x.split(" ").collect::<Vec<&str>>();

            let op = PlayerMove::from(plays[0]);
            let me = PlayerOutcome::from(plays[1]);
            return me.outcome(op);
        })
        .sum();

    Some(round)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
