advent_of_code::solution!(13);



fn is_perfect_reflection_row(box_thing: &Vec<Vec<bool>>, row: usize)-> bool{
    let steps = (row).min(box_thing.len()-row);

    // println!("Doint {} steps for row {}", steps, row);
    (0..steps).all(|i|{
        box_thing[row-1-i] == box_thing[row+i]
    })
}

fn column_equals(box_thing: &Vec<Vec<bool>>, column1: usize, column2: usize)-> bool{
    (0..box_thing.len()).all(|row|{
        box_thing[row][column1] == box_thing[row][column2]
    })
}


fn is_perfect_reflection_column(box_thing: &Vec<Vec<bool>>, column: usize)-> bool{
    let steps = (column).min(box_thing[0].len()-column);

    (0..steps).all(|i|{
        column_equals(box_thing, column-1-i, column+i)
    })
}

fn calculate_box(box_thing: &Vec<Vec<bool>>)-> u32{
    for i in 1..box_thing.len(){
        if is_perfect_reflection_row(box_thing, i){
            
            return 100 * i as u32;
        }
    }
    for i in 1..box_thing[0].len(){
        if is_perfect_reflection_column(box_thing, i){
            
            return i as u32;
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut thing:Vec<Vec<bool>> = vec![];
    let mut sum:u32 = 0;

    for line in input.lines(){
        if line.is_empty(){
            sum += calculate_box(&thing);
            thing.clear()
        }else{
            thing.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    if !thing.is_empty(){
        sum += calculate_box(&thing);
    }
            
    Some(sum)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn test_t() {
        let box_thing = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let row = 2;
        let steps = (row).min(box_thing.len()-row);

        println!("{}", steps);
    }
}
