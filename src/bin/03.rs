advent_of_code::solution!(3);

fn getChar(x: usize, y: usize, matrix: &Vec<Vec<char>>)-> char{
    if x < 0 || y < 0 {
        '.'
    }else if x >= matrix.len() || y >= matrix[x].len(){
        '.'
    }else{
        matrix[x][y]
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let matrix:Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut x = 0;
    let mut result:u32 = 0;
    while x < matrix.len() {
        let mut y = 0;
        while y < matrix[x].len() {
            let c = matrix[x][y];
            if c.is_numeric(){
                let mut number:String = c.to_string();
                while y+1 < matrix[x].len() && matrix[x][y+1].is_numeric() {
                    y+=1;
                    number.push(matrix[x][y]);
                }
                let mut a = vec![
                    getChar(x-usize::min(1, x), y-usize::min(number.len(), y), &matrix),
                    getChar(x, y-usize::min(number.len(), y), &matrix),
                    getChar(x+1, y-usize::min(number.len(), y), &matrix),
                    getChar(x-usize::min(1, x), y+1, &matrix),
                    getChar(x, y+1, &matrix),
                    getChar(x+1, y+1, &matrix),
                ];

                
                a.append(
                    &mut (y-usize::min(number.len(), y)..y+1).map(|new_y| getChar(x-usize::min(1, x), new_y, &matrix)).collect()
                );
                a.append(
                    &mut (y-usize::min(number.len(), y)..y+1).map(|new_y| getChar(x+1, new_y, &matrix)).collect()
                );
                // println!("Number is {}", number);
                // print!("Surrounding characters are");
                // a.iter().for_each(|f| print!("{}, ", f));
                // println!("");
                if a.iter().any(|char| is_character(*char))
                 {
                    // println!("Yes\n");
                    result += number.parse::<u32>().unwrap();
                 }
                 else {
                    // println!("No\n");
                 }
            }
            y+=1;
        }   
        x+=1;
    }

    Some(result)
}

fn is_character(character: char) -> bool{
    !(character == '.' || character.is_numeric())
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
