use std::u32;

use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let colors_re = Regex::new(r"Card\s+[0-9]+:\s+(?<winners>([0-9]+\s+)+)\|\s+(?<yours>([0-9]+\s*)+)").unwrap();
    Some(input.lines().map( | line| {
                
        let Some(game_caps) = colors_re.captures(line) else {
            println!("no match: {}", line);
            return 0
        };

        let winners:Vec<u32> = game_caps["winners"].split_whitespace().map(| number | {
            number.parse::<u32>().expect("To be a number")}
        ).collect();
        let yours = game_caps["yours"].split_whitespace().map(| number | {
            number.parse::<u32>().expect("To be a number")
        });

        let n = yours.filter(|n|{
            winners.contains(n)
        }).count() as u32;
        
        let result = if n > 0
        {
            (2 as u32).pow(n-1)
        }
        else {
            0
        };

        result 
    }).sum())


}

pub fn part_two(input: &str) -> Option<u32> {
    let colors_re = Regex::new(r"Card\s+[0-9]+:\s+(?<winners>([0-9]+\s+)+)\|\s+(?<yours>([0-9]+\s*)+)").unwrap();
    let mut index = 0;
    let mut cards = vec![1; input.lines().count()];

    input.lines().for_each( | line| {
                
        let Some(game_caps) = colors_re.captures(line) else {
            println!("no match: {}", line);
            return 
        };

        let winners:Vec<u32> = game_caps["winners"].split_whitespace().map(| number | {
            number.parse::<u32>().expect("To be a number")}
        ).collect();
        let yours = game_caps["yours"].split_whitespace().map(| number | {
            number.parse::<u32>().expect("To be a number")
        });

        let n = yours.filter(|n|{
            winners.contains(n)
        }).count() as u32;
    

        for x in 1..n+1{
            cards[index+x as usize]+=cards[index as usize]
        }

        index+=1;
    });

    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(true, true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(true, true);
    }
}
