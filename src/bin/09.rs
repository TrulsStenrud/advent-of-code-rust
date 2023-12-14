advent_of_code::solution!(9);

fn extrapolate(line: &Vec<i32>) -> (i32, i32){
    let mut all_zero = true;
    let steps = line.iter().enumerate().skip(1).map(|(i, n)|{
        if n != &line[i-1]{
            all_zero = false;
        }
         n - line[i-1]
    }).collect::<Vec<_>>();
    if all_zero{
        (line[0], line[line.len()-1])
    }else{
        let (before, after) =  extrapolate(&steps);
        (line[0] - before, line[line.len()-1] + after) 
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(|line|{
        extrapolate(&line.split_whitespace().map(|it|{
            it.parse().unwrap()
        }).collect::<Vec<_>>()).1
    }).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input.lines().map(|line|{
        extrapolate(&line.split_whitespace().map(|it|{
            it.parse().unwrap()
        }).collect::<Vec<_>>()).0
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
