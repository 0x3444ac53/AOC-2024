advent_of_code::solution!(1);
use std::collections::HashMap;

fn get_lists(input: &str) -> Vec<Vec<u32>> {
    let mut lists = vec![vec![], vec![]];
    input.lines().for_each({
        |line| {
            let nums: Vec<&str> = line.split_whitespace().collect();
            let n1: u32 = String::from(nums[0]).parse().unwrap();
            let n2: u32 = String::from(nums[1]).parse().unwrap();
            lists[0].push(n1);
            lists[1].push(n2);
        }
    });
    lists[0].sort();
    lists[1].sort();
    lists
}

fn get_occurances(list: Vec<u32>) -> HashMap<u32, u32> {
    list.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let lists = get_lists(input);
    Some(
        lists[0]
            .clone()
            .into_iter()
            .zip(lists[1].clone())
            .fold(0, |acc, (n1, n2)| acc + n1.abs_diff(n2)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = get_lists(input);
    let occs = get_occurances(lists[1].clone());
    lists[0]
        .clone()
        .into_iter()
        .map(|n| n * occs.get(&n).unwrap_or(&0))
        .reduce(|acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
