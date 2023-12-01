
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let first = line.chars().find(|char| char.is_numeric()).unwrap();
        let last = line.chars().rev().find(|char| char.is_numeric()).unwrap();
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
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
