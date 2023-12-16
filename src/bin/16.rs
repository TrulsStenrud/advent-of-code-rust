use std::time::{Instant, Duration};

advent_of_code::solution!(16);

#[derive(Clone, Copy)]
struct Tile{
    c: char,
    enirgized: [bool;4]
}

struct Beam{
    x: usize,
    y: usize,
    direction: Direction 
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

fn draw_map(map: &Vec<Vec<Tile>>) {
    map.iter().enumerate().for_each(|(i, row)|{
        let light = row.iter().map(|tile|{
            if tile.enirgized.iter().any(|it|*it){
                '#'
            }
            else {'.'}
        }).collect::<String>();
        let mirrors = map[i].iter().map(|tile| tile.c).collect::<String>();
        println!("{}, {}", light, mirrors);
    });
}

fn count_energized(map: &Vec<Vec<Tile>>) -> u32 {
    map.iter().map(|row|{
        row
        .iter()
        .filter(|tile| tile.enirgized.iter().any(|it|*it)).count() as u32
    }).sum::<u32>()
}

fn energize(beams: &mut Vec<Beam>, map: &mut Vec<Vec<Tile>>) {
    let mut curr_tile: &mut Tile;
    let mut beams_len:usize;

    while beams.len()>0 {
        beams_len = beams.len();
    
        for i in (0..beams_len).rev(){
        
            curr_tile = &mut map[beams[i].x][beams[i].y];
        
            if curr_tile.enirgized[beams[i].direction as usize]{
                beams.remove(i);
            }else{
                curr_tile.enirgized[beams[i].direction as usize] = true; 

                match curr_tile.c{
                    '.'=>{
                        move_in_direction(i, beams, map);
                    },
                    '/'=>{
                        match beams[i].direction {
                            Direction::UP => beams[i].direction = Direction::RIGHT,
                            Direction::RIGHT => beams[i].direction = Direction::UP,
                            Direction::DOWN => beams[i].direction = Direction::LEFT,
                            Direction::LEFT => beams[i].direction = Direction::DOWN,
                        }
                        move_in_direction(i,  beams, map);
                    },
                    '\\'=>{
                        match beams[i].direction {
                            Direction::UP => beams[i].direction = Direction::LEFT,
                            Direction::RIGHT => beams[i].direction = Direction::DOWN,
                            Direction::DOWN => beams[i].direction = Direction::RIGHT,
                            Direction::LEFT => beams[i].direction = Direction::UP,
                        }
                        move_in_direction(i,  beams, map);
                    },
                    '|'=>{
                        match beams[i].direction {
                            Direction::RIGHT |
                            Direction::LEFT => {
                                beams[i].direction = Direction::UP;
                                beams.push(Beam { x: beams[i].x, y: beams[i].y, direction: Direction::DOWN });
                                move_in_direction(beams.len() - 1,  beams, map);
                            },
                            _=>{}
                        }
                        move_in_direction(i,  beams, map)
                    },
                    '-'=>{
                        match beams[i].direction {
                            Direction::DOWN |
                            Direction::UP => {
                                beams[i].direction = Direction::RIGHT;
                                beams.push(Beam { x: beams[i].x, y: beams[i].y, direction: Direction::LEFT });
                                move_in_direction(beams.len()-1,  beams, map);
                            },
                            _=>{}
                        }
                        move_in_direction(i, beams, map)
                    },
                    _=>{panic!();
                    }
                }
            }
        }
    }
}

fn move_in_direction(i: usize, beams: &mut Vec<Beam>, map: &Vec<Vec<Tile>>) {
    match beams[i].direction {
        Direction::UP => {
            if beams[i].x == 0{
                beams.remove(i);
            }
            else{
                beams[i].x-=1;
            }
        },
        Direction::RIGHT => {
            if beams[i].y == map[beams[i].x].len()-1{
                beams.remove(i);
            }
            else{
                beams[i].y+=1;
            }
        },
        Direction::DOWN => {
            if beams[i].x == map.len()-1{
                beams.remove(i);
            }
            else{
                beams[i].x+=1;
            }
        },
        Direction::LEFT => {
            if beams[i].y == 0{
                beams.remove(i);
            }
            else{
                beams[i].y-=1;
            }
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input.lines().map(|line|{
        line.chars().map(|c|{
            Tile{c, enirgized: [false;4]}
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut beams =vec![Beam{ x: 0, y: 0, direction: Direction::RIGHT}];

    energize(&mut beams, &mut map);
    Some(count_energized(&map))
    
}



pub fn part_two(input: &str) -> Option<u32> {
    let map = input.lines().map(|line|{
        line.chars().map(|c|{
            Tile{c, enirgized: [false;4]}
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

        
    Some((0..map[0].len()).map(|i|{
    
        let mut this_map = map.clone();
        
        energize(&mut vec![Beam{
            x: 0,
            y: i,
            direction: Direction::DOWN
        }], &mut this_map);

        count_energized(&this_map)
    }).max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
        
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
