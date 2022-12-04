pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|n| *n == b'\n')
            .map(|s| s.split_at(s.len() / 2))
            .map(|(a, b)| b.iter().find(|e| a.contains(e)).unwrap())
            .map(|e| {
                if *e >= b'a' {
                    (e - b'a') as i32 + 1
                } else {
                    (e - b'A') as i32 + 27
                }
            })
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .as_bytes()
            .split(|l| *l == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|g| {
                g[0].iter()
                    .find(|e| g[1].contains(e) && g[2].contains(e))
                    .unwrap()
            })
            .map(|e| {
                if *e >= b'a' {
                    (e - b'a') as i32 + 1
                } else {
                    (e - b'A') as i32 + 27
                }
            })
            .sum::<i32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
