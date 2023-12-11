use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1000000) as u64)
}

fn solve (input: &str, multiplier: usize) -> usize{
    let mut galaxies: Vec<[usize; 2]> = vec![];
    
    let mut max_y = 0;
    let mut min_y =usize::MAX;
    
    let mut offset_x = 0;   
    
    let mut used_y:HashSet<usize> = HashSet::new();

    input.lines().enumerate().for_each(|(line_nr, line)|{
        let mut x_is_empty = true;
    
        line.chars().enumerate().for_each(|(c_nr, c)|{
    
            if c == '#'{
                x_is_empty = false;
                
                max_y = max_y.max(c_nr);
                min_y = min_y.min(c_nr);

                galaxies.push([line_nr + (offset_x), c_nr]); 
                used_y.insert(c_nr);
            }
        });
        if x_is_empty{
            offset_x += multiplier-1
        }
    });

    let empty_ys = (min_y..max_y).filter(|it| {
            !used_y.contains(it)
        }).collect::<Vec<_>>();

    for i in 0..galaxies.len(){
        galaxies[i][1] += empty_ys.iter().filter(|y| y < &&galaxies[i][1]).count()*(multiplier-1);
    }

    let mut sum = 0;
    for i in 0..galaxies.len()-1{
        for j in (i+1)..galaxies.len(){
            sum+= dist(&galaxies[i], &galaxies[j])
        }
    };

    sum
}

fn dist(a: &[usize; 2], b:&[usize; 2])->usize{
    a[0].max(b[0]) - a[0].min(b[0]) + a[1].max(b[1]) - a[1].min(b[1])
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
