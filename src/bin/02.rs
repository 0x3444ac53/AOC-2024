advent_of_code::solution!(2);
use itertools::Itertools;

fn is_safe(levels: Vec<u32>) -> bool {
    levels.windows(2).all(|w| (w[0] > w[1] && w[0] <= w[1] + 3))
        || levels.windows(2).all(|w| (w[1] > w[0] && w[1] <= w[0] + 3))
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().try_fold(0, |acc, line| {
        let nums: Vec<u32> = line
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_safe(nums) {
            Some(acc + 1)
        } else {
            Some(acc)
        }
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
