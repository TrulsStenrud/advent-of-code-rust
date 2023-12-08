use std::collections::HashMap;


advent_of_code::solution!(8);

struct StringDirections {
    this: String,
    right: String,
    left: String,
}
struct IntDirections {
    this: String,
    right: usize,
    left: usize,
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let go_right_sequence: Vec<bool> = lines.next().unwrap().chars().map(|char| char == 'R').collect();
    
    let mut string_directions:Vec<StringDirections> = vec![];
    let mut location_to_index:HashMap<String, usize> = HashMap::new();
    
    let mut index = 0;
    lines.skip(1).for_each(|line|{
        let current = line[0..3].to_string();
        string_directions.push( StringDirections{
            this: current.clone(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string()
        });
        location_to_index.insert(current, index);
        index+=1
    });

    let int_directions = string_directions.iter().map(| thing|{
        IntDirections{
            this: thing.this.clone(),
            left: location_to_index[&thing.left],
            right: location_to_index[&thing.right],
        }
    }).collect::<Vec<IntDirections>>();
    
    let mut counter = 0;
    let mut position = location_to_index["AAA"]; 
    
    while int_directions[position].this != "ZZZ" {
        if go_right_sequence[counter % go_right_sequence.len()]{
            position = int_directions[position].right;
        }else {
            position = int_directions[position].left;
        }
        counter+=1;
    }

    Some(counter as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let go_right_sequence: Vec<bool> = lines.next().unwrap().chars().map(|char| char == 'R').collect();
    let mut start_positions:Vec<String> = vec![];
    
    let mut string_directions:Vec<StringDirections> = vec![];
    let mut location_to_index:HashMap<String, usize> = HashMap::new();
    
    let mut index = 0;
    lines.skip(1).for_each(|line|{
        let current = line[0..3].to_string();
        
        if current.ends_with('A'){
            start_positions.push(current.clone());
        }

        string_directions.push( StringDirections{
            this: current.clone(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string()
        });
        location_to_index.insert(current, index);
        index+=1
    });

    let int_directions = string_directions.iter().map(| thing|{
        IntDirections{
            this: thing.this.clone(),
            left: location_to_index[&thing.left],
            right: location_to_index[&thing.right],
        }
    }).collect::<Vec<IntDirections>>();
    
    let positions:Vec<usize> = start_positions.iter().map(|pos| location_to_index[pos]).collect();

    let lentghs = positions.iter().map(|start|{
        let mut counter = 0;
        let mut position = *start; 
        
        while !int_directions[position].this.ends_with('Z') {
            if go_right_sequence[counter % go_right_sequence.len()]{
                position = int_directions[position].right;
            }else {
                position = int_directions[position].left;
            }
            counter+=1;
        }

        counter as u64
    }).collect::<Vec<u64>>();    

    Some(lcm_of_vector(lentghs))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_vector(values: Vec<u64>) -> u64 {
    if values.is_empty() {
        panic!("Input vector is empty");
    }

    let mut result = values[0];
    
    for &value in values.iter().skip(1) {
        result = lcm(result, value);
    }

    result
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
        assert_eq!(result, Some(6));
    }
}
