advent_of_code::solution!(2);

trait IsSafe {
    fn is_safe(&self, damped: bool) -> bool;
}

impl IsSafe for Vec<u32> {
    fn is_safe(&self, damped: bool) -> bool {
        let mut changes: Vec<i32> = vec![];
        self.windows(2)
            .for_each(|w| changes.push(w[1] as i32 - w[0] as i32));
        let _safe = changes.clone().into_iter().filter(|w| *w <= 3 && *w >= -3);
        if (changes.iter().all(|w| *w > 0) || changes.iter().all(|w| *w < 0))
            && _safe.clone().count() == changes.len()
        {
            true
        } else {
            dbg!(self);
            dbg!(changes.clone());
            dbg!(
                (changes.iter().all(|w| *w > 0) || changes.iter().all(|w| *w < 0))
                    && _safe.count() == changes.len()
            );
            false
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().try_fold(0, |acc, line| {
        let nums: Vec<u32> = line
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if nums.is_safe(false) {
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
