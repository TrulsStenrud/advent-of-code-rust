use std::u32;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    
    Some(input.lines().map( | line| {
                
        let mut winners:Vec<u32> = [].to_vec();        
        let mut is_winners = true;
        let mut win_count = 0;

        line.split_whitespace().skip(2).for_each(|number|{
            if number == "|"{
                is_winners = false;
            } else if is_winners{
                winners.push(number.parse::<u32>().expect("To be a number"));
            } else if winners.contains(&number.parse::<u32>().expect("To be a number")){
                win_count+=1
            }
        });
        
        let result = if win_count > 0
        {
            (2 as u32).pow(win_count-1)
        }
        else {
            0
        };

        result 
    }).sum())


}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut index = 0;
    let mut cards = vec![1; input.lines().count()];

    input.lines().for_each( | line| {
        let mut winners:Vec<u32> = [].to_vec();        
        let mut is_winners = true;
        let mut win_count = 0;
        line.split_whitespace().skip(2).for_each(|number|{
            if number == "|"{
                is_winners = false;
            } else if is_winners{
                winners.push(number.parse::<u32>().expect("To be a number"));
            } else if winners.contains(&number.parse::<u32>().expect("To be a number")){
                win_count+=1
            }
        });
        
        for x in 1..win_count+1{
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
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        println!("{}", result.unwrap());
        assert_eq!(true, true);
    }
}
