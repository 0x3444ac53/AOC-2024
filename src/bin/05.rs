advent_of_code::solution!(5);
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Clone, Debug)]
struct OrderingRules {
    rules: HashMap<u32, HashSet<u32>>,
    in_degrees: HashMap<u32, usize>,
}

impl DerefMut for OrderingRules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rules
    }
}

impl Deref for OrderingRules {
    type Target = HashMap<u32, HashSet<u32>>;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl OrderingRules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
            in_degrees: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct InstructionManual {
    pages_raw: Vec<u32>,
    pages_hash: HashMap<u32, usize>,
    subgraph: OrderingRules,
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
        for (&item, afters) in &rules.rules {
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

impl FromIterator<(u32, u32)> for OrderingRules {
    fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
        let mut rules = OrderingRules::new();
        for (before, after) in iter {
            rules.entry(before).or_default().insert(after);
            rules.entry(after).or_default();
            *rules.in_degrees.entry(after).or_insert(0) += 1;
            rules.in_degrees.entry(before).or_insert(0);
        }
        rules
    }
}

impl InstructionManual {
    fn new<T>(items: Vec<u32>, rule_set: T) -> Option<Self>
    where
        T: IntoIterator<Item = (u32, u32)>,
    {
        let pages_hash = items.to_index_map();
        let pages_raw = items;
        let subgraph = rule_set
            .into_iter()
            .filter(|(b, a)| pages_hash.contains_key(b) && pages_hash.contains_key(a))
            .collect();
        Some(Self {
            pages_hash,
            pages_raw,
            subgraph,
        })
    }
    fn from_str<T>(s: &str, rule_set: T) -> Result<Self, String>
    where
        T: IntoIterator<Item = (u32, u32)>,
    {
        Ok(Self::new(
            s.split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect(),
            rule_set,
        )
        .unwrap())
    }

    fn new_force_order(items: Vec<u32>, rules: &OrderingRules) {
        // Find Sub graph with the nodes we care about
        // and then recalculate our in_degrees
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_str, books) = input.split_once("\n\n")?;
    let rule_super_set: OrderingRules = rule_str
        .lines()
        .map(|rule| {
            let (b, a) = rule.split_once('|').unwrap();
            (b.parse().unwrap(), a.parse().unwrap())
        })
        .collect();
}

pub fn part_two(input: &str) -> Option<u32> {
    // let (rule_str, books) = input.split_once("\n\n")?;
    // let mut rules: OrderingRules = OrderingRules::new();
    // for rule in rule_str.lines() {
    //     let (_before, _after) = rule.split_once('|')?;
    //     let after = _after.parse().unwrap();
    //     let before = _before.parse().unwrap();
    //     rules.entry(before).or_default().insert(after);
    //     rules.entry(after).or_default();
    //     *rules.in_degrees.entry(after).or_insert(0) += 1;
    //     rules.in_degrees.entry(before).or_insert(0);
    // }
    // dbg!(books
    //     .lines()
    //     .filter_map(|book_str| {
    //         InstructionManual::from_str(book_str)
    //             .ok()
    //             .filter(|book| !book.validate(&rules))
    //     })
    //     .collect::<Vec<InstructionManual>>());
    // dbg!(rules);
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
