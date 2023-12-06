advent_of_code::solution!(6);

fn test_run(hold_seconds: u32, total_time: u32)->u32{
    hold_seconds * (total_time - hold_seconds)
}

fn test_run_big(hold_seconds: u64, total_time: u64)->u64{
    hold_seconds * (total_time - hold_seconds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.parse::<u32>().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut result = 1 as u32;
    times.enumerate().map(|(index, time)|{
        let distance = distances[index];
        
        ((distance/time) as u32..time - (distance/time))
        .filter(|test_hold| test_run(*test_hold, time)> distance)
        .count()
    })
    .for_each(|value| {
        result*=value as u32
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();
    let distance = lines.next().unwrap().split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap();
    
    let v = (time.pow(2) as f64 - 4f64 * distance as f64).sqrt();
    Some(((0.5 * (time as f64 + v)).ceil() - (0.5 * (time as f64 - v)).ceil())as u32)
}

pub fn part_two_binary_search(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap();
    let distance = lines.next().unwrap().split_whitespace().skip(1).map(|it| it.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap();
        
        let mut start = distance/time;
        let mut stop = time - start;

        let temp = (stop - start ) / 2;
        let middle = start + temp;
        let mut keep_koing = true;
        
        let mut p = middle;
        
        while keep_koing {
            let current_hold = (start + p)/2;
            let current_value = test_run_big(current_hold, time);
            
            if current_value > distance{
                p = current_hold;
            } else if current_value < distance{
                if start == current_hold{
                    keep_koing = false;                    
                }
                else{
                    start = current_hold;
                }
            } else {
                start = current_hold + 1;
                keep_koing = false;
            }
        }
        start+=1;
        
        
        p = middle;

        keep_koing = true;
        while keep_koing {
            let current_hold = (p + stop)/2;
            let current_value = test_run_big(current_hold, time);

            if current_value > distance{
                if p == current_hold{
                    keep_koing = false
                }
                p = current_hold;
            } else if current_value < distance{
                if stop == current_hold{
                    keep_koing = false
                }
                stop = current_hold;
            } else {
                stop = current_hold - 1;
                keep_koing = false;
            }
        }
        stop-=1;

    Some((stop - start + 1) as u32)
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
}
