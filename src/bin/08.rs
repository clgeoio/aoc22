fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10))
                .map(Option::unwrap)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let tranposed = transpose(matrix.clone());

    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let value = &matrix[y][x];

            let top = if y > 0 {
                Some(&tranposed[x][0..y])
            } else {
                None
            };
            let right = if x < matrix[y].len() {
                Some(&matrix[y][x + 1..])
            } else {
                None
            };
            let bottom = if y < matrix.len() {
                Some(&tranposed[x][y + 1..])
            } else {
                None
            };
            let left = if x > 0 { Some(&matrix[y][0..x]) } else { None };

            if left.is_some() && right.is_some() && top.is_some() && bottom.is_some() {
                if left.unwrap().iter().all(|x| x < value)
                    || right.unwrap().iter().all(|x| x < value)
                    || top.unwrap().iter().all(|x| x < value)
                    || bottom.unwrap().iter().all(|x| x < value)
                {
                    count = count + 1
                }
            } else {
                count = count + 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10))
                .map(Option::unwrap)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let tranposed = transpose(matrix.clone());

    let mut score = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let value = &matrix[y][x];

            let top = if y > 0 {
                let v = &tranposed[x][0..y];
                let t = v.iter().rev().take_while(|&x| x < value).count();
                v.iter().take(t + 1).count()
            } else {
                0
            };

            let right = if x < matrix[y].len() {
                let v = &matrix[y][x + 1..];
                let t = v.iter().take_while(|&x| x < value).count();
                v.iter().take(t + 1).count()
            } else {
                0
            };
            let bottom = if y < matrix.len() {
                let v = &tranposed[x][y + 1..];
                let t = v.iter().take_while(|&x| x < value).count();
                v.iter().take(t + 1).count()
            } else {
                0
            };

            let left = if x > 0 {
                let v = &matrix[y][0..x];
                let t = v.iter().rev().take_while(|&x| x < value).count();
                v.iter().take(t + 1).count()
            } else {
                0
            };

            let s = [top, right, bottom, left].iter().fold(1, |acc, v| acc * v);
            score = score.max(s);
        }
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
