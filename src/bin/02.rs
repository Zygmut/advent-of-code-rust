pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .map(|e| ((e[0] - b'A') as i32, (e[2] - b'X') as i32))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .map(|e| ((e[0] - b'A') as i32, (e[2] - b'X') as i32))
            .map(|(a, b)| 1 + b * 3 + (a + b + 2) % 3)
            .sum::<i32>(),
    )
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
