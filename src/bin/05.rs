advent_of_code::solution!(5);
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

type OrderingRules = HashMap<u32, HashSet<u32>>;

#[derive(Clone, Debug)]
struct InstructionManual {
    pages_raw: Vec<u32>,
    pages_hash: HashMap<u32, usize>,
}

trait ToIndexMap<T> {
    fn to_index_map(&self) -> HashMap<T, usize>;
}

trait Ordered {
    fn validate(&self, _: &OrderingRules) -> bool;
}

impl<T: std::hash::Hash + Eq + Copy> ToIndexMap<T> for Vec<T> {
    fn to_index_map(&self) -> HashMap<T, usize> {
        self.iter()
            .enumerate()
            .map(|(i, &item)| (item, i))
            .collect()
    }
}

impl Ordered for InstructionManual {
    fn validate(&self, rules: &OrderingRules) -> bool {
        for (&item, afters) in rules {
            if let Some(&item_pos) = self.pages_hash.get(&item) {
                for num_after in afters {
                    if let Some(&after_pos) = self.pages_hash.get(num_after) {
                        if after_pos <= item_pos {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl InstructionManual {
    fn new(items: Vec<u32>) -> Option<Self> {
        Some(Self {
            pages_hash: items.to_index_map(),
            pages_raw: items,
        })
    }
    fn new_checked(items: Vec<u32>, rules: &OrderingRules) -> Option<Self> {
        let tmp = Self::new(items)?;
        if tmp.validate(rules) {
            Some(tmp)
        } else {
            None
        }
    }
    fn from_str_checked(s: &str, rules: &OrderingRules) -> Option<Self> {
        Self::new_checked(
            s.split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect(),
            rules,
        )
    }
}

impl FromStr for InstructionManual {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect(),
        )
        .unwrap())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_str, books) = input.split_once("\n\n")?;
    let mut rules: OrderingRules = OrderingRules::new();
    for rule in rule_str.lines() {
        let (_before, _after) = rule.split_once('|')?;
        let after = _after.parse().unwrap();
        let before = _before.parse().unwrap();
        rules.entry(before).or_default().insert(after);
        rules.entry(after).or_default();
    }
    Some(
        books
            .lines()
            .filter_map(|book| InstructionManual::from_str_checked(book, &rules))
            .map(|book| book.pages_raw[book.pages_raw.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rule_str, books) = input.split_once("\n\n")?;
    let mut rules: OrderingRules = OrderingRules::new();
    for rule in rule_str.lines() {
        let (_before, _after) = rule.split_once('|')?;
        let after = _after.parse().unwrap();
        let before = _before.parse().unwrap();
        rules.entry(before).or_default().insert(after);
        rules.entry(after).or_default();
    }
    dbg!(books
        .lines()
        .filter_map(|book_str| {
            InstructionManual::from_str(book_str)
                .ok()
                .filter(|book| !book.validate(&rules))
        })
        .collect::<Vec<InstructionManual>>());
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
