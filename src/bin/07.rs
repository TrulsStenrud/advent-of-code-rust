use std::{collections::HashMap, fmt::Display, time::Instant};

advent_of_code::solution!(7);

    static FIVE:u32 = 6;
    static FOUR:u32 = 5;
    static HOUSE:u32 = 4;
    static THREE:u32 = 3;
    static TWOPAIRS:u32=2;
    static PAIR:u32=1;
    static NOTHING:u32=0;

    fn number_to_hand(c: &u32)->String{
        match c {
            6 => "FIVE".to_string(),
            5 => "FOUR".to_string(),
            4 => "HOUSE".to_string(),
            3 => "THREE".to_string(),
            2 => "TWOPAIRS".to_string(),
            1 => "PAIR".to_string(),
            0 => "NOTHING".to_string(),
                _=> panic!(),
        }
    }
    
    fn get_number(c: &char)->usize{
        match c {
                'A'=> 12,
                'K'=> 11,
                'Q'=> 10,
                'J'=> 9,
                'T'=> 8,
                '9'=> 7,
                '8'=> 6,
                '7'=> 5,
                '6'=> 4,
                '5'=> 3,
                '4'=> 2,
                '3'=> 1,
                '2'=> 0,
                _=> panic!(),
        }
    }
    fn get_number_joker(c: &char)->usize{
        match c {
                'A'=> 12,
                'K'=> 11,
                'Q'=> 10,
                'T'=> 9,
                '9'=> 8,
                '8'=> 7,
                '7'=> 6,
                '6'=> 5,
                '5'=> 4,
                '4'=> 3,
                '3'=> 2,
                '2'=> 1,
                'J'=> 0,
                _=> panic!(),
        }
    }

fn find_hand(input: &str)->u32{

    let mut counts = [0; 13];
    
    input.chars().for_each(|c| counts[get_number(&c)]+=1);
    if counts.contains(&3) {
        if counts.contains(&(2)){
            return HOUSE;
        }else {
            return THREE;
        }
    }
    if counts.contains(&(2)){
        if counts.iter().filter(|n| *n == &(2)).count() == 2{
            return TWOPAIRS
        }else{
            return PAIR
        }
    }
    if counts.contains(&(4)){
        return FOUR;
    }
    if counts.contains(&(5)){
        return FIVE;
    }
    
    NOTHING
}

fn find_hand_joker(input: &str)->u32{

    let mut counts = [0; 13];

    input.chars().for_each(|c| counts[get_number_joker(&c)]+=1);

    let jokers = counts[get_number_joker(&'J')];

    if jokers == 4{
        return FIVE;
    }

    if jokers == 3{
        if counts.contains(&2){
            return FIVE;
        }
        else {
            return FOUR
        }
    }

    if jokers == 2{
        if counts.contains(&3){
            return FIVE;
        }
        else if counts.iter().filter(|n| *n == &(2)).count() == 2 {
            return FOUR
        }
        else {
            return THREE
        }
    }

    if jokers == 1{
        if counts.contains(&4){
            return FIVE;
        }
        else if counts.contains(&3){
            return FOUR;
        }
        else if counts.contains(&2){
            if counts.iter().filter(|n| *n == &(2)).count() == 2 {
                return HOUSE
            }
            else {
                return THREE
            }
        }
        else {
            return PAIR
        }
    }


    if counts.contains(&3) {
        if counts.contains(&(2)){
            return HOUSE;
        }else {
            return THREE;
        }
    }
    if counts.contains(&(2)){
        if counts.iter().filter(|n| *n == &(2)).count() == 2{
            return TWOPAIRS
        }else{
            return PAIR
        }
    }
    if counts.contains(&(4)){
        return FOUR;
    }
    if counts.contains(&(5)){
        return FIVE;
    }
    
    NOTHING
}


struct Hand{
    // cards: String,
    bid: u32,
    value: u32
}

fn calc_value_for_highcard_comparison(cards: &str )->u32{
    cards.chars().enumerate().map(|(i, c)|{
        (13 as u32).pow((cards.len()-i-1) as u32) * (get_number(&c)+1) as u32
    }).sum()
}

fn calc_value_for_highcard_comparison_joker(cards: &str )->u32{
    cards.chars().enumerate().map(|(i, c)|{
        (13 as u32).pow((cards.len()-i-1) as u32) * (get_number_joker(&c)+1) as u32
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {    
    let mut hands:[Vec<Hand>;7] = Default::default();
    
    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap().parse::<u32>().unwrap();
        let value_of_hand = find_hand(hand);
        hands[value_of_hand as usize].push(Hand{
            // cards:hand.to_string(), 
            bid: bid, 
            value: calc_value_for_highcard_comparison(hand)
        })
    });
    
    (0..hands.len()).for_each(|i|{
    hands[i].sort_by(| hand_a, hand_b|{
                if hand_a.value > hand_b.value {
                    return std::cmp::Ordering::Greater;
                }
                else if hand_a.value < hand_b.value{
                    return std::cmp::Ordering::Less;
                }
            std::cmp::Ordering::Less
    });
    });
   
    let mut i = 0;

    Some(hands.iter().map(|list|{
        list.iter().map(|thing|{
            i+=1;
            thing.bid * (i) as u32
        }).sum::<u32>()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut hands:[Vec<Hand>;7] = Default::default();
    
    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap().parse::<u32>().unwrap();
        let value_of_hand = find_hand_joker(hand);
        hands[value_of_hand as usize].push(Hand{
            // cards: hand.to_string(), 
            bid: bid, 
            value: calc_value_for_highcard_comparison_joker(hand)
        })
    });
    
    (0..hands.len()).for_each(|i|{
    hands[i].sort_by(| hand_a, hand_b|{
                if hand_a.value > hand_b.value {
                    return std::cmp::Ordering::Greater;
                }
                else if hand_a.value < hand_b.value{
                    return std::cmp::Ordering::Less;
                }
            std::cmp::Ordering::Less
    });
    });
   
    let mut i = 0;

    Some(hands.iter().map(|list|{
        list.iter().map(|thing|{
            i+=1;
            thing.bid * (i) as u32
        }).sum::<u32>()
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
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part() {
        
        [
            "77977",
            "JT7T3",
            "QQ58J",
            "J3762",
            "6KTAJ",
            "24J29",
            "6689J",
            "2J222",
            "JJJJJ",
            "653JK",
            "4K5JA",
            "5593J",
            "5A95J",
            "KKJJK",
            ].iter().for_each( |it|{   
            println!("{} {}", it, number_to_hand(&find_hand_joker(it)));
        });

    }
}
