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


fn step(pos:&mut Position,  map: &Vec<Vec<char>>){
    // let binding = get_neighbours(pos.curr, map);
    // let test = binding.iter().filter(|neighbour|{
        
    //     is_pointing_at(pos.curr, **neighbour, map) && *neighbour != &pos.prev
    
    // }).collect::<Vec<_>>();
    
    if pos.curr.0 != 0{
        if is_pointing_at(pos.curr, (pos.curr.0-1, pos.curr.1), map) && (pos.curr.0-1, pos.curr.1)!= pos.prev{
            pos.prev = pos.curr;
            pos.curr = (pos.curr.0-1, pos.curr.1);
            pos.steps+=1;
            return;
        }
    }
    if pos.curr.1 != 0{
        if is_pointing_at(pos.curr, (pos.curr.0, pos.curr.1-1), map) && (pos.curr.0, pos.curr.1-1)!= pos.prev{
            pos.prev = pos.curr;
            pos.curr = (pos.curr.0, pos.curr.1-1);
            pos.steps+=1;
            return;
        }
    }
    if pos.curr.0 != map.len()-1{
        if is_pointing_at(pos.curr, (pos.curr.0+1, pos.curr.1), map) && (pos.curr.0+1, pos.curr.1)!= pos.prev{
            pos.prev = pos.curr;
            pos.curr = (pos.curr.0+1, pos.curr.1);
            pos.steps+=1;
            return;
        }
    }
    if pos.curr.1 != map[pos.curr.0].len()-1{
        if is_pointing_at(pos.curr, (pos.curr.0, pos.curr.1+1), map) && (pos.curr.0, pos.curr.1+1)!= pos.prev{
            pos.prev = pos.curr;
            pos.curr = (pos.curr.0, pos.curr.1+1);
            pos.steps+=1;
            return;
        }
    }
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


fn find_value_based_on_neighbours(start: (usize, usize), map: &Vec<Vec<char>>) -> char {
    let binding = get_neighbours(start, &map);
    let neighbours = binding
    .iter()
    .filter(|n| is_pointing_at(**n, start, &map))
    .collect::<Vec<_>>();

    let n1 = neighbours[0];
    let n2 = neighbours[1];

    if n1.0 > start.0{
        if n2.1 > start.1{
            'F'
        }else if n2.1 < start.1{
            '7'
        }else{
            '|'
        }
    }
    else if n1.0 < start.0{
        if n2.1 > start.1{
            'L'
        }else if n2.1 < start.1{
            'J'
        }else{
            '|'
        }
    }else if n1.1 > start.1{
        if n2.0 > start.0{
            'F'
        }else if n2.1 < start.1{
            'L'
        }else{
            '-'
        }
    }
    else if n1.1 < start.1{
        if n2.0 > start.0{
            '7'
        }else if n2.0 < start.0{
            'J'
        }else{
            '-'
        }
    }
    else{
        panic!();
    }


}

#[derive(PartialEq, Eq)]
enum Location{
    INSIDE,
    OUTSIDE,
    ONLINE_IN_IF_UP,
    ONLINE_IN_IF_DOWN,
}

struct Part {
    c: char,
    y: usize
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
    
    while starters[0].curr != start{
        step(&mut starters[0], &map);       
    }
    Some(starters[0].steps/2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start = (usize::MAX, usize::MAX);

    let mut map:Vec<Vec<char>> = input.lines().enumerate().map(|(x, line)|{
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
    
    let mut thing:Vec<Vec<Part>> = (0..map.len()).map(|_| vec![]).collect();

    
    thing[start.0].push(Part{ y: start.1, c: find_value_based_on_neighbours(start, &map)});


    
    while starters[0].curr != start{
        step(&mut starters[0], &map);
        thing[starters[0].prev.0].push(Part{ y: starters[0].prev.1, c: map[starters[0].prev.0][starters[0].prev.1]})
    }

    let mut sum:usize = 0;
    thing.iter_mut().for_each(|line|{
        
        let mut location = Location::OUTSIDE;
        let mut entered_inside = usize::MAX;

        line.sort_by_key(|it|it.y);

        line.iter().for_each(|node|{
            if location == Location::OUTSIDE{
                if node.c == '|'{
                    location = Location::INSIDE;
                    entered_inside = node.y + 1;
                }
                else if node.c == 'F'{
                    location = Location::ONLINE_IN_IF_UP;
                }
                else if node.c == 'L'{
                    location = Location::ONLINE_IN_IF_DOWN;
                }
                else{
                    panic!();
                }
            }
            else if location == Location::INSIDE{
                sum+= node.y - entered_inside;
                entered_inside = usize::MAX;

                if node.c == '|'{
                    location = Location::OUTSIDE;
                }
                else if node.c == 'L' {
                    location = Location::ONLINE_IN_IF_UP;
                }
                else if node.c == 'F' {
                    location = Location::ONLINE_IN_IF_DOWN;
                }
                else {
                    panic!()
                }
            }
            else if location == Location::ONLINE_IN_IF_DOWN{
                if node.c == 'J'{
                    location = Location::OUTSIDE;
                }
                else if node.c == '7' {
                    location = Location::INSIDE;
                    entered_inside = node.y + 1;
                }else if node.c == '-'{
                    // do nothing
                }
                else {
                    println!("Found {}", node.c);
                    panic!();
                }
            }
            else if location == Location::ONLINE_IN_IF_UP{
                if node.c == '7'{
                    location = Location::OUTSIDE;
                }
                else if node.c == 'J' {
                    location = Location::INSIDE;
                    entered_inside = node.y + 1;
                }else if node.c == '-'{
                    // do nothing
                }
                else {
                    println!("Found {}", node.c);
                    panic!();
                }
            }

        });
    });
    

    Some(sum as u32)
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
