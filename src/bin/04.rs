pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(",").unwrap();
                let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
                (
                    a.parse::<u32>().unwrap(),
                    b.parse::<u32>().unwrap(),
                    c.parse::<u32>().unwrap(),
                    d.parse::<u32>().unwrap(),
                )
            })
            .filter(|(a, b, c, d)| (a <= c && b >= d) || (a >= c && b <= d))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(",").unwrap();
                let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
                (
                    a.parse::<u32>().unwrap(),
                    b.parse::<u32>().unwrap(),
                    c.parse::<u32>().unwrap(),
                    d.parse::<u32>().unwrap(),
                )
            })
            .filter(|(a, b, c, d)| (a <= c && b >= d) || (a >= c && b <= d) || (a <= c && b >= c) || (a <= d && b >= d))
            .count() as u32,
    )
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
