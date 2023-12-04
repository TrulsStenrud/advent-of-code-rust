advent_of_code::solution!(2);
use regex::Regex;
use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let game_re = Regex::new(r"Game (?<game>[0-9]+)").unwrap();
    let colors_re = Regex::new(r"(?<number>[0-9]+) (?<color>green|blue|red)").unwrap();
    Some(input.lines().map( | line| {
        
        let Some(game_caps) = game_re.captures(line) else {
            println!("no game match!");
            return 0
        };
    
        let is_invalid = colors_re.captures_iter(line).any( | it | {
            let color = &it["color"];
            let number = &it["number"].parse::<u32>().expect("To be a number");
            
            match color{ 
                "red"=> {
                    return number > &12
                }, 
                "blue"=>{
                    return number > &14
                }, 
                "green"=>{
                    return number > &13
                }, 
                _=>{
                    return number > &0
                },  
              }
        });

        if is_invalid {
            0
        }else{
            game_caps["game"].parse::<u32>().expect("To be a number")
        }

    }).sum())


}

pub fn part_two(input: &str) -> Option<u32> {
    let colors_re = Regex::new(r"(?<number>[0-9]+) (?<color>green|blue|red)").unwrap();
    Some(input.lines().map( | line| {
        let mut reds= 0;
        let mut blues = 0;
        let mut greens=0;
    
        colors_re.captures_iter(line).for_each( | it | {
            let color = &it["color"];
            let number = it["number"].parse::<u32>().expect("To be a number");
            
            match color{ 
                "red"=> {
                    reds = cmp::max(reds, number);
                }, 
                "blue"=>{
                    blues = cmp::max(blues, number);
                }, 
                "green"=>{
                    greens = cmp::max(greens, number);
                }, 
                _=>{
                    println!("Color [{}] did not match", color)
                },  
              }
        });
        reds*blues*greens
    }).sum())


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
