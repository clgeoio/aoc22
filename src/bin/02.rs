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
        let value = match self {
            Self::Rock => {
                self.value()
                    + match other {
                        PlayerMove::Rock => 3,
                        PlayerMove::Paper => 0,
                        PlayerMove::Scissors => 6,
                    }
            }
            Self::Paper => {
                self.value()
                    + match other {
                        PlayerMove::Rock => 6,
                        PlayerMove::Paper => 3,
                        PlayerMove::Scissors => 0,
                    }
            }
            Self::Scissors => {
                self.value()
                    + match other {
                        PlayerMove::Rock => 0,
                        PlayerMove::Paper => 6,
                        PlayerMove::Scissors => 3,
                    }
            }
        };
        println!("{:?} plays {:?} outcome {}", self, other, value);
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
        let value = match self {
            Self::Lose => {
                self.value()
                    + match other {
                        PlayerMove::Rock => PlayerMove::Scissors.value(),
                        PlayerMove::Paper => PlayerMove::Rock.value(),
                        PlayerMove::Scissors => PlayerMove::Paper.value(),
                    }
            }
            Self::Draw => {
                self.value()
                    + match other {
                        PlayerMove::Rock => PlayerMove::Rock.value(),
                        PlayerMove::Paper => PlayerMove::Paper.value(),
                        PlayerMove::Scissors => PlayerMove::Scissors.value(),
                    }
            }
            Self::Win => {
                self.value()
                    + match other {
                        PlayerMove::Rock => PlayerMove::Paper.value(),
                        PlayerMove::Paper => PlayerMove::Scissors.value(),
                        PlayerMove::Scissors => PlayerMove::Rock.value(),
                    }
            }
        };

        println!("{:?} plays {:?} outcome {}", self, other, value);
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
