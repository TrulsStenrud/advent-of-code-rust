use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(7);


fn five_of_a_kind(input: &str)->Option<char>{
    let mut chars = input.chars();
    let first = chars.next().unwrap();
    for i in chars{
        if i != first{
            return None;
        }
    }
    Some(first)
}

// struct Five{
//     value:char
// }

// struct Four{
//     value:char
// }
// struct Three{
//     value:char
// }

// struct Pair{
//     value:char
// }
// struct TwoPair{
//     value1:char,
//     value2:char
// }

    static FIVE:u32 = 6;
    static FOUR:u32 = 5;
    static HOUSE:u32 = 4;
    static THREE:u32 = 3;
    static TWOPAIRS:u32=2;
    static PAIR:u32=1;
    static NOTHING:u32=0;

    
fn find_hand(input: &str)->u32{
    let mut map: HashMap<char, u32> = HashMap::new();

    for c in input.chars(){
        *map.entry(c).or_insert(0) += 1;
    }
    let values:Vec<u32> = map.values().map(|it|*it).collect();
    if values.contains(&(5 as u32)){
        return FIVE;
    }
    if values.contains(&(4 as u32)){
        return FOUR;
    }
    if values.contains(&(3 as u32)) {
        if values.contains(&(2 as u32)){
            return HOUSE;
        }else {
            return THREE;
        }
    }
    if values.contains(&(2 as u32)){
        if values.iter().filter(|n| *n == &(2 as u32)).count() == 2{
            return TWOPAIRS
        }else{
            return PAIR
        }
    }
    
    NOTHING
}


pub fn part_one(input: &str) -> Option<u32> {

    let card_map: HashMap<char, u32> = HashMap::from([
        ('A', 13),
         ('K', 12),
         ('Q', 11),
         ('J', 10),
         ('T', 9),
         ('9', 8),
         ('8', 7),
         ('7', 6),
         ('6', 5),
         ('5', 4),
         ('4', 3),
         ('3', 2),
         ('2', 1),
    ]);

    let mut hands_and_values:Vec<(String, u32, u32)> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap().parse::<u32>().unwrap();
        let valueOfHand = find_hand(hand);
        (hand.to_string(), valueOfHand, bid)
    }).collect();

    hands_and_values.sort_by(| (handA, valueA, bidA), (handB, valueB, bidB)|{
        if valueA > valueB {
            std::cmp::Ordering::Greater
        }
        else if valueB > valueA{
            std::cmp::Ordering::Less
        }
        else {
            let chars_a:Vec<char> = handA.chars().collect();
            let chars_b:Vec<char> = handB.chars().collect();
            for i in 0..5{
                let A = card_map[&chars_a[i]];
                let B = card_map[&chars_b[i]];
                if A > B {
                    return std::cmp::Ordering::Greater;
                }
                else if B > A{
                    return std::cmp::Ordering::Less;
                }
            }

            std::cmp::Ordering::Less
        }
        
    });

    Some(hands_and_values.iter().enumerate().map(|(i, thing)|{
        // println!("{}", thing.0);
        thing.2 * (i+1) as u32
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let card_map: HashMap<char, u32> = HashMap::from([
        ('A', 13),
         ('K', 12),
         ('Q', 11),
         ('T', 9),
         ('9', 8),
         ('8', 7),
         ('7', 6),
         ('6', 5),
         ('5', 4),
         ('4', 3),
         ('3', 2),
         ('2', 1),
         ('J', 0),
    ]);

    let mut hands_and_values:Vec<(String, u32, u32)> = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap().parse::<u32>().unwrap();
        let valueOfHand = find_hand(hand);
        (hand.to_string(), valueOfHand, bid)
    }).collect();

    hands_and_values.sort_by(| (handA, valueA, bidA), (handB, valueB, bidB)|{
        if valueA > valueB {
            std::cmp::Ordering::Greater
        }
        else if valueB > valueA{
            std::cmp::Ordering::Less
        }
        else {
            let chars_a:Vec<char> = handA.chars().collect();
            let chars_b:Vec<char> = handB.chars().collect();
            for i in 0..5{
                let A = card_map[&chars_a[i]];
                let B = card_map[&chars_b[i]];
                if A > B {
                    return std::cmp::Ordering::Greater;
                }
                else if B > A{
                    return std::cmp::Ordering::Less;
                }
            }

            std::cmp::Ordering::Less
        }
        
    });

    Some(hands_and_values.iter().enumerate().map(|(i, thing)|{
        // println!("{}", thing.0);
        thing.2 * (i+1) as u32
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part() {
        let result = find_hand("BBSU");
        assert_eq!(result, PAIR);
    }
}
