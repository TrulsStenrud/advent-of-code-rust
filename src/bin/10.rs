advent_of_code::solution!(10);


fn get_neighbours(pos: (usize, usize), map: &Vec<Vec<char>>)->Vec<(usize, usize)>{
    let mut binding:Vec<(usize, usize)> = vec![];
    if pos.0 != 0{
        binding.push((pos.0-1, pos.1))
    }
    if pos.1 != 0{
        binding.push((pos.0, pos.1-1))
    }
    if pos.0 != map.len()-1{
        binding.push((pos.0+1, pos.1))
    }
    if pos.1 != map[pos.0].len()-1{
        binding.push((pos.0, pos.1+1))
    }
    return binding
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start = (usize::MAX, usize::MAX);

    let map:Vec<Vec<char>> = input.lines().enumerate().map(|(x, line)|{
        line.chars().enumerate().map(|(y, c)|{
            if c == 'S'{
                start = (x, y)
            }
            c
        }).collect()
    }).collect();

    if start == (usize::MAX, usize::MAX){
        panic!("Did not find start");
    }
   
    let mut starters = get_neighbours(start, &map).iter()
    .filter(|it| {
        is_pointing_at(**it, start, &map)
    })
    .map(|pos|{
        Position{
            curr: *pos,
            steps: 1,
            prev: start
        }
    })
    .collect::<Vec<_>>();

    if starters.len() != 2{
        panic!();
    }
    
    while !has_crashed(&starters[0], &starters[1]){
        starters.iter_mut().for_each(|pos|{
            step(pos, &map);
        })
    }

    if starters[0].curr == starters[1].curr{
        Some(starters[0].steps)
    }
    else{
        Some(starters[0].steps-1)
    }


}


fn has_crashed(a: &Position, b:&Position)-> bool{
     a.curr == b.curr || a.curr == b.prev
}

fn step(pos:&mut Position,  map: &Vec<Vec<char>>){
    let binding = get_neighbours(pos.curr, map);
    let test = binding.iter().filter(|neighbour|{
        is_pointing_at(pos.curr, **neighbour, map) && *neighbour != &pos.prev
    }).collect::<Vec<_>>();
    if test.len() != 1{
        panic!();
    }
    pos.prev = pos.curr;
    pos.curr = *test[0];
    pos.steps+=1
}

struct Position{
    steps: u32,
    curr:(usize, usize),
    prev:(usize, usize)
}

fn is_pointing_at(pos: (usize, usize), target_pos: (usize, usize), map: &Vec<Vec<char>>)-> bool{  
    if pos.0 == target_pos.0{
        if pos.1 < target_pos.1{
            // target_pos is east
            return ['-', 'F', 'L'].contains(&map[pos.0][pos.1]);
        }
        else{
            // target_pos is west
            return ['-', '7', 'J'].contains(&map[pos.0][pos.1]);
        }
    }
    else if pos.1 == target_pos.1{
        if pos.0 < target_pos.0{
            // target_pos is south
            return ['|', 'F', '7'].contains(&map[pos.0][pos.1]);
        }
        else{
            // target_pos is north
            return ['|', 'L', 'J'].contains(&map[pos.0][pos.1]);
        }
    }

    false
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
