advent_of_code::solution!(5);
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Deref;
use std::ops::DerefMut;

use itertools::WhileSome;

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
    fn validate(&self) -> bool;
    fn sorted(&self) -> Result<Self, &str>
    where
        Self: Sized;
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
    fn validate(&self) -> bool {
        for (item, afters) in &self.subgraph.rules {
            if let Some(&item_pos) = self.pages_hash.get(item) {
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
    fn sorted(&self) -> Result<Self, &str> {
        let mut in_degrees = self.subgraph.in_degrees.clone();
        let mut queue: VecDeque<u32> = in_degrees
            .iter()
            .filter_map(|(&node, &in_degree)| if in_degree == 0 { Some(node) } else { None })
            .collect();

        let mut sorted = vec![];

        while let Some(node) = queue.pop_front() {
            sorted.push(node);
            self.subgraph
                .get(&node)
                .into_iter()
                .flat_map(|neighbours| neighbours.iter())
                .for_each(|&neighbour| {
                    if let Some(degree) = in_degrees.get_mut(&neighbour) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(neighbour);
                        }
                    }
                })
        }
        if sorted.len() == self.subgraph.len() {
            Ok(Self {
                pages_hash: sorted.to_index_map(),
                pages_raw: sorted,
                subgraph: self.subgraph.clone(),
            })
        } else {
            Err("Cyclic")
        }
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_str, books) = input.split_once("\n\n")?;
    let rule_super_set: HashSet<(u32, u32)> = rule_str
        .lines()
        .map(|rule| {
            let (b, a) = rule.split_once('|').unwrap();
            (b.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())
        })
        .collect();
    Some(books.lines().fold(0, |acc, book| {
        match InstructionManual::from_str(book, rule_super_set.clone()) {
            Ok(b) => {
                if b.validate() {
                    acc + b.pages_raw[b.pages_raw.len() / 2]
                } else {
                    acc
                }
            }
            _ => acc,
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rule_str, books) = input.split_once("\n\n")?;
    let rule_super_set: HashSet<(u32, u32)> = rule_str
        .lines()
        .map(|rule| {
            let (b, a) = rule.split_once('|').unwrap();
            (b.parse::<u32>().unwrap(), a.parse::<u32>().unwrap())
        })
        .collect();
    Some(books.lines().fold(0, |acc, book| {
        match InstructionManual::from_str(book, rule_super_set.clone()) {
            Ok(b) => {
                if b.validate() {
                    acc
                } else if let Ok(sorted) = b.sorted() {
                    acc + sorted.pages_raw[sorted.pages_raw.len() / 2]
                } else {
                    acc
                }
            }
            _ => acc,
        }
    }))
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
        assert_eq!(result, Some(123));
    }
}
