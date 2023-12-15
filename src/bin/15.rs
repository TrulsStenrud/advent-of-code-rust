use core::num;

advent_of_code::solution!(15);

fn hash(input: &str)->u64{
    let mut current_value = 0;
    for c in input.chars(){
        current_value += c as u64;
        current_value*=17;
        current_value = current_value % 256;
    }
    current_value
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.trim().split(',').map(hash).sum())
}

#[derive(Clone)]
struct Lens{
    label: String,
    numbers: String,
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes:Vec<Vec<Lens>> = vec![vec![]; 256];
    for str in input.trim().split(','){
        
        let mut operation = '0';
        let mut numbers:String = String::new();
        let mut label:String = String::new();
        for c in str.chars(){
            if c.is_alphabetic(){
                label.push(c)
            }
            else if c.is_ascii_digit(){
                numbers.push(c)
            }
            else {
                operation = c
            }
        }
        let box_i = hash(&label) as usize;
        
        if operation == '='{
            let mut modified = false;
            for i in 0..boxes[box_i].len(){
                if boxes[box_i][i].label == label{
                    modified = true;
                    boxes[box_i][i].numbers = numbers.clone()
                }
            }
            if ! modified{
                boxes[box_i].push(Lens{ label, numbers});
            }
        }
        else if operation == '-'{
            boxes[box_i].retain(|lens| lens.label != label);
        }
        else {
            panic!("Ehh...")
        }
    }
    Some(
        boxes.iter().enumerate().map(|(i, lenses)|{
            lenses.iter().enumerate().map(|(j, lens)|{      
                ((i+1)*(j+1)*lens.numbers.parse::<usize>().unwrap()) as u32
            }).sum::<u32>()
        }).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        let result =hash("HASH");
        assert_eq!(result, 52);
    }
}
