use std::{collections::HashMap, fmt::Display, time::Instant};

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


struct Hand{
    cards: String,
    bid: u32,
    value: u32
}

fn calc_value(cards: &str )->u32{
    cards.chars().enumerate().map(|(i, c)|{
        ((cards.len()-i) as u32)*13 * get_number(&c) as u32
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    // let mut start = Instant::now();
    
    let mut a:[Vec<Hand>;7] = Default::default();
    
    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap().parse::<u32>().unwrap();
        let value_of_hand = find_hand(hand);
        a[value_of_hand as usize].push(Hand{
            cards: hand.to_string(), bid: bid, value: calc_value(hand)
        })
    });
    // let mut duration = start.elapsed();

    // println!("Time elapsed in finding hands is: {:?}", duration);
    // start = Instant::now();

    (0..a.len()).for_each(|i|{
    a[i].sort_by(| hand_a, hand_b|{
                if hand_a.value > hand_b.value {
                    println!("{} is greater", hand_a.cards);
                    println!("{}", hand_b.cards);
                    return std::cmp::Ordering::Greater;
                }
                else if hand_a.value < hand_b.value{
                    println!("{} is less", hand_a.cards);
                    println!("{}", hand_b.cards);
                    return std::cmp::Ordering::Less;
                }
            std::cmp::Ordering::Less
    });
    });
   
    // duration = start.elapsed();

    // println!("Time elapsed in sorting: {:?}", duration);
    let mut i = 0;

    Some(a.iter().map(|list|{
        list.iter().map(|thing|{
            i+=1;
            thing.bid * (i+1) as u32
        }).sum::<u32>()
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
                let a = card_map[&chars_a[i]];
                let b = card_map[&chars_b[i]];
                if a > b {
                    return std::cmp::Ordering::Greater;
                }
                else if b > a{
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
//         J9999 is greater
// Q8888
        let result1 = calc_value("J9999");
        let resul2t = calc_value("Q8888");
        println!("{}", result1);
        println!("{}", resul2t);
    }
}
