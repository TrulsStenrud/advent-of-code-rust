advent_of_code::solution!(6);

fn test_run(hold_seconds: u32, total_time: u32)->u32{
    hold_seconds * (total_time - hold_seconds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.parse::<u32>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut result = 1 as u32;
    times.enumerate().map(|(index, time)|{
        let distance = distances[index];
        
        (1..time)
        .filter(|test_hold| test_run(*test_hold, time)> distance)
        .count()
    })
    .for_each(|value| {
        result*=value as u32
    });

    Some(result)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
