use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut register_x: Vec<i32> = Vec::from([1]);
    input.lines().for_each(|line| {
        let current_value = register_x[register_x.len() - 1];

        register_x.push(current_value);

        match line {
            "noop" => (),
            value => match value.split(" ").collect_tuple::<(_, _)>() {
                None => {}
                Some((_, b)) => {
                    let value = str::parse::<i32>(b).unwrap();
                    register_x.push(current_value + value);
                }
            },
        }
    });

    let v = Vec::from([20, 60, 100, 140, 180, 220])
        .iter()
        .fold(0, |acc, v| acc + (register_x[*v - 1] * *v as i32));
    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut register_x: Vec<i32> = Vec::from([1]);
    input.lines().for_each(|line| {
        let current_value = register_x[register_x.len() - 1];

        register_x.push(current_value);

        match line {
            "noop" => (),
            value => match value.split(" ").collect_tuple::<(_, _)>() {
                None => {}
                Some((_, b)) => {
                    let value = str::parse::<i32>(b).unwrap();
                    register_x.push(current_value + value);
                }
            },
        }
    });

    let mut crt = vec!['.'; register_x.len() - 1];
    for i in 0..crt.len() {
        let x = register_x[i];
        let j = i as i32 % 40;
        if x - 1 == j as i32 || x == j as i32 || x + 1 == j as i32 {
            crt[i] = '#'
        }
    }

    crt.into_iter()
        .chunks(40)
        .into_iter()
        .for_each(|x| println!("{:?}", x.into_iter().join("")));

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
