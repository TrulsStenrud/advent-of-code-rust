use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {

    let mut maxX = 0;
    let mut maxY = 0;
    let mut galaxies: Vec<[usize; 2]> = vec![];
    input.lines().enumerate().for_each(|(line_nr, line)|{
        line.chars().enumerate().for_each(|(c_nr, c)|{
            if c == '#'{
                galaxies.push([line_nr, c_nr]);
                maxX = usize::max(maxX, line_nr);
                maxY = usize::max(maxY, c_nr);
            }
        });
    });

    let existing_x = galaxies.iter().map(|it| it[0]).collect::<HashSet<_>>();
    let existing_y = galaxies.iter().map(|it| it[1]).collect::<HashSet<_>>();

    let mut sum = 0;

    for i in 0..galaxies.len()-1{
        for j in (i+1)..galaxies.len(){
            let start = galaxies[i];
            let stop = galaxies[j];

            
            let diff_x = start[0].max(stop[0]) - start[0].min(stop[0]);
            let diff_y = start[1].max(stop[1]) - start[1].min(stop[1]);
            let extra_x = ((start[0].min(stop[0])+1)..(start[0].max(stop[0])))
            .filter(|x|!existing_x.contains(x)).count();
        
            let extra_y = ((start[1].min(stop[1])+1)..(start[1].max(stop[1])))
            .filter(|x|!existing_y.contains(x)).count();

        sum+= diff_x + diff_y + extra_x + extra_y
        }
    };

    Some(sum as u32)
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
