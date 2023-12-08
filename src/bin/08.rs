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

pub fn part_two(input: &str) -> Option<u32> {
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
    
    let mut counter = 0;
    let mut positions:Vec<usize> = start_positions.iter().map(|pos| location_to_index[pos]).collect();
    while !positions.iter().all(|pos| int_directions[*pos].this.ends_with('Z')) {
        
        for i in 0..positions.len(){
            if go_right_sequence[counter % go_right_sequence.len()]{
                positions[i] = int_directions[positions[i]].right;
            }else {
                positions[i] = int_directions[positions[i]].left;
            }
        }
        counter+=1;
    }

    Some(counter as u32)
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
