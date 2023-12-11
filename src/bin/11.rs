use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {

    let mut galaxies: Vec<[usize; 2]> = vec![];
    let mut extra_x = 0;   
        
    let mut max_y = 0;
    let mut min_y =usize::MAX;
    
    let mut existing_y:HashSet<usize> = HashSet::new();
    input.lines().enumerate().for_each(|(line_nr, line)|{
        let mut is_empty = true;
    
        line.chars().enumerate().for_each(|(c_nr, c)|{
    
            if c == '#'{
                is_empty = false;
                
                max_y = max_y.max(c_nr);
                min_y = min_y.min(c_nr);

                galaxies.push([line_nr + (extra_x), c_nr]); 
                existing_y.insert(c_nr);
            }
        });
        if is_empty{
            extra_x += 1
        }
    });

    let spacers = (min_y..max_y).filter(|it| {
            !existing_y.contains(it)
        }).collect::<Vec<_>>();

    for i in 0..galaxies.len(){
        galaxies[i][1] +=spacers.iter().filter(|x| x < &&galaxies[i][1]).count();
    }

    let mut sum = 0;

    for i in 0..galaxies.len()-1{
        for j in (i+1)..galaxies.len(){
            let start = galaxies[i];
            let stop = galaxies[j];
            
            let diff_x = start[0].max(stop[0]) - start[0].min(stop[0]);
            let diff_y = start[1].max(stop[1]) - start[1].min(stop[1]);

            sum+= diff_x + diff_y
        }
    };

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut galaxies: Vec<[usize; 2]> = vec![];
    let mut extra_x = 0;   
        
    let mut max_y = 0;
    let mut min_y =usize::MAX;
    
    let mut existing_y:HashSet<usize> = HashSet::new();
    input.lines().enumerate().for_each(|(line_nr, line)|{
        let mut is_empty = true;
    
        line.chars().enumerate().for_each(|(c_nr, c)|{
    
            if c == '#'{
                is_empty = false;
                
                max_y = max_y.max(c_nr);
                min_y = min_y.min(c_nr);

                galaxies.push([line_nr + (extra_x), c_nr]); 
                existing_y.insert(c_nr);
            }
        });
        if is_empty{
            extra_x += 1000000-1
        }
    });

    let spacers = (min_y..max_y).filter(|it| {
            !existing_y.contains(it)
        }).collect::<Vec<_>>();

    for i in 0..galaxies.len(){
        galaxies[i][1] += spacers.iter().filter(|x| x < &&galaxies[i][1]).count()*(1000000-1);
    }

    let mut sum = 0;

    for i in 0..galaxies.len()-1{
        for j in (i+1)..galaxies.len(){
            let start = galaxies[i];
            let stop = galaxies[j];
            
            let diff_x = start[0].max(stop[0]) - start[0].min(stop[0]);
            let diff_y = start[1].max(stop[1]) - start[1].min(stop[1]);

            sum+= diff_x + diff_y
        }
    };

    Some(sum as u64)
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
