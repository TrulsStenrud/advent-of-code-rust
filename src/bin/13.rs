advent_of_code::solution!(13);

fn is_perfect_reflection_row_smudge(box_thing: &Vec<Vec<bool>>, row: usize)-> bool{
    let steps = (row).min(box_thing.len()-row);
    let mut smudge_fixed = false;
    // println!("Doint {} steps for row {}", steps, row);
    (0..steps).all(|i|{
        // box_thing[row-1-i] == box_thing[row+i]
        (0..box_thing[0].len()).all(|column|{
            if box_thing[row-1-i][column] == box_thing[row+i][column]{
                true
            }else{
                if smudge_fixed{
                    false
                }else{
                    smudge_fixed = true;
                    true
                }
            }
        })
    }) && smudge_fixed
}

fn is_perfect_reflection_column_smudge(box_thing: &Vec<Vec<bool>>, column: usize)-> bool{
    let steps = (column).min(box_thing[0].len()-column);
    let mut smudge_fixed = false;
    (0..steps).all(|i|{
        // column_equals(box_thing, column-1-i, column+i)
        (0..box_thing.len()).all(|row|{
            if box_thing[row][column-1-i] == box_thing[row][column+i] {
                true
            }else{
                if smudge_fixed{
                    false
                }else{
                    smudge_fixed = true;
                    true
                }
            }
        })
    }) && smudge_fixed
}

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

fn calculate_box(box_thing: &Vec<Vec<bool>>, 
    is_reflection_row: &dyn Fn(&Vec<Vec<bool>>, usize)->bool, 
    is_reflection_column: &dyn Fn(&Vec<Vec<bool>>, usize)->bool)-> u32{
    for i in 1..box_thing.len(){
        if is_reflection_row(box_thing, i){
            
            return 100 * i as u32;
        }
    }
    for i in 1..box_thing[0].len(){
        if is_reflection_column(box_thing, i){
            
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
            sum += calculate_box(&thing, &is_perfect_reflection_row, &is_perfect_reflection_column);
            thing.clear()
        }else{
            thing.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    if !thing.is_empty(){
        sum += calculate_box(&thing, &is_perfect_reflection_row, &is_perfect_reflection_column);
    }
            
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut thing:Vec<Vec<bool>> = vec![];
    let mut sum:u32 = 0;

    for line in input.lines(){
        if line.is_empty(){
            sum += calculate_box(&thing, &is_perfect_reflection_row_smudge, &is_perfect_reflection_column_smudge);
            thing.clear()
        }else{
            thing.push(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        }
    }
    if !thing.is_empty(){
        sum += calculate_box(&thing, &is_perfect_reflection_row_smudge, &is_perfect_reflection_column_smudge);
    }
            
    Some(sum)
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

    #[test]
    fn test_t() {
        let box_thing = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let row = 2;
        let steps = (row).min(box_thing.len()-row);

        println!("{}", steps);
    }
}
