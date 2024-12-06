use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

struct WordSearch {
    grid: Vec<Vec<u8>>,
    line_len: usize,
}

impl WordSearch {
    fn new(data: &str) -> Option<Self> {
        let grid: Vec<Vec<u8>> = data.lines().map(|l| l.as_bytes().to_vec()).collect();
        let line_len = grid[0].len();
        if grid.iter().any(|l| l.len() != line_len) {
            return None;
        }
        Some(Self { grid, line_len })
    }
}

trait SomeHelpers {
    fn does_have(&self, v: usize, h: usize, c: u8) -> Result<(u8, usize), String>;
    fn search(&self, pat: &str, h_offset: i32, v_offset: i32) -> Vec<Vec<(u8, usize)>>;
    fn j_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn k_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn l_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn h_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn hj_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn hk_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn lk_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
    fn lj_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>>;
}

#[inline]
pub fn offsets(o: i32) -> Box<dyn Fn(usize, usize) -> Option<usize>> {
    if o != 0 {
        if o < 0 {
            {
                Box::new(move |n1: usize, n2: usize| n1.checked_sub(n2))
            }
        } else {
            Box::new(move |n1: usize, n2: usize| Some(n1 + n2))
        }
    } else {
        Box::new(move |n1: usize, _| Some(n1))
    }
}

impl SomeHelpers for WordSearch {
    fn does_have(&self, h: usize, v: usize, c: u8) -> Result<(u8, usize), String> {
        if h < self.line_len && v < self.grid.len() && c == self.grid[v][h] {
            Ok((c, (self.line_len + 1) * v + h))
        } else {
            Err("Nope Nope Nope".to_string())
        }
    }
    fn search(&self, pat: &str, h_offset: i32, v_offset: i32) -> Vec<Vec<(u8, usize)>> {
        let v_offset = offsets(v_offset);
        let h_offset = offsets(h_offset);
        let mut finds = vec![];
        let pat_bytes = pat.as_bytes();
        for h in 0..self.grid.len() {
            for v in 0..self.line_len {
                if let Ok(v) = pat_bytes
                    .iter()
                    .enumerate()
                    .map(|(i, c)| match (h_offset(h, i), v_offset(v, i)) {
                        (Some(ho), Some(vo)) => self.does_have(ho, vo, *c),
                        _ => Err("I don't know rust")?,
                    })
                    .collect()
                {
                    finds.push(v);
                }
            }
        }
        finds
    }
    #[inline]
    fn j_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, 0, 1)
    }
    #[inline]
    fn l_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, 1, 0)
    }
    #[inline]
    fn h_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, -1, 0)
    }
    #[inline]
    fn k_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, 0, -1)
    }
    #[inline]
    fn lj_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, 1, 1)
    }
    #[inline]
    fn hk_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, -1, -1)
    }
    #[inline]
    fn hj_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, -1, 1)
    }
    #[inline]
    fn lk_search(&self, pat: &str) -> Vec<Vec<(u8, usize)>> {
        self.search(pat, 1, -1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = WordSearch::new(input)?;
    Some(
        [
            SomeHelpers::l_search,
            SomeHelpers::j_search,
            SomeHelpers::k_search,
            SomeHelpers::h_search,
            SomeHelpers::lk_search,
            SomeHelpers::lj_search,
            SomeHelpers::hk_search,
            SomeHelpers::hj_search,
        ]
        .iter()
        .map(|f| f(&puzzle, "XMAS").len())
        .sum::<usize>()
        .try_into()
        .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle = WordSearch::new(input)?;
    let mut count = 0;
    let mut expected: HashSet<&usize> = HashSet::new();
    Some(
        [
            SomeHelpers::lk_search,
            SomeHelpers::lj_search,
            SomeHelpers::hk_search,
            SomeHelpers::hj_search,
        ]
        .iter()
        .flat_map(|f| f(&puzzle, "MAS").into_iter())
        .counts_by(|c| c[1].1)
        .into_iter()
        .filter(|(_, c)| *c > 1)
        .count()
        .try_into()
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
