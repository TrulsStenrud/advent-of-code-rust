
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| {
        let first = line.chars().find(|char| char.is_numeric()).unwrap();
        let last = line.chars().rev().find(|char| char.is_numeric()).unwrap();
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum())
}

fn find_first(input: &str, map: &[(String, char); 20]) -> char {
    map.iter().min_by_key(|(text, _)|{
            input.match_indices(text).find(|_| true).unwrap_or((1111111, "")).0
        }
        ).unwrap().1
}

pub fn part_two(input: &str) -> Option<u32> {
    let thing = [
            ("zero".to_owned(), '0'),
            ("one".to_owned(), '1'),
            ("two".to_owned(), '2'),
            ("three".to_owned(), '3'),
            ("four".to_owned(), '4'),
            ("five".to_owned(), '5'),
            ("six".to_owned(), '6'),
            ("seven".to_owned(), '7'),
            ("eight".to_owned(), '8'),
            ("nine".to_owned(), '9'),
            ("0".to_owned(), '0'),
            ("1".to_owned(), '1'),
            ("2".to_owned(), '2'),
            ("3".to_owned(), '3'),
            ("4".to_owned(), '4'),
            ("5".to_owned(), '5'),
            ("6".to_owned(), '6'),
            ("7".to_owned(), '7'),
            ("8".to_owned(), '8'),
            ("9".to_owned(), '9')
        ];

    let reversed_thing = thing.clone().map(|(text, value)| (text.chars().rev().collect::<String>(),value));

    Some(input.lines().map(|line| {

        let first = find_first(line, &thing);
        let reversed = line.chars().rev().collect::<String>();
        let last = find_first(&reversed, &reversed_thing);
        
        format!("{}{}", first, last).parse::<u32>().unwrap()
    }).sum())
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        
        assert_eq!(result, None);
    }
}
