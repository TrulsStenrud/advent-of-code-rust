advent_of_code::solution!(14);

static ROCK:u32=0;
static BOULDER:u32=1;
static SPACE:u32=2;

pub fn part_one(input: &str) -> Option<u32> {


    let mut boulders: Vec<(usize, usize)> = vec![];

    let mut dish:Vec<Vec<u32>> = input.lines().enumerate().map(|(x, line)|{
        // println!("{}", line);
        line.chars().enumerate().map(|(y, c)|{
            match c {
                'O' => {
                    boulders.push((x, y));
                    BOULDER
                },
                '#' => ROCK,
                '.' => SPACE,
                _ => panic!()
            }
        }).collect()
    }).collect();

    (0..boulders.len()).for_each(|i|{
        move_boulder_north(&mut dish, &mut boulders, i);
    });

    // println!();
    // dish.iter().for_each(|v|{
    //     v.iter().for_each(|b|{
    //         match b {
    //             0 => print!("#"),
    //             1 => print!("O"),
    //             2 => print!("."),
    //             _ => panic!(),
    //         }
    //     });
    //     println!();
    // });

    let len = dish.len();
    Some(boulders.iter().map(|(x, _)| (len - x) as u32).sum())
}

fn move_boulder_north(dish:&mut Vec<Vec<u32>>, boulders: &mut Vec<(usize, usize)>, boulder_n: usize ){
    let (start_x, start_y) = boulders[boulder_n];
    if start_x == 0{
        return;
    }
        
    let mut steps = 0;

    while steps < start_x && dish[start_x-(steps+1)][start_y] == SPACE {
        steps+=1
    }
    if steps == 0{
        return;
    }
    dish[start_x][start_y] = SPACE;
    dish[start_x-steps][start_y] = BOULDER;
    boulders[boulder_n] = (start_x-steps, start_y);
}
fn move_boulder_west(dish:&mut Vec<Vec<u32>>, boulders: &mut Vec<(usize, usize)>, boulder_n: usize ){
    let (start_x, start_y) = boulders[boulder_n];
    if start_y == 0{
        return;
    }
        
    let mut steps = 0;

    while steps < start_y && dish[start_x][start_y-(steps+1)] == SPACE {
        steps+=1
    }
    if steps == 0{
        return;
    }
    dish[start_x][start_y] = SPACE;
    dish[start_x][start_y-steps] = BOULDER;
    boulders[boulder_n] = (start_x, start_y-steps);
}

fn move_boulder_south(dish:&mut Vec<Vec<u32>>, boulders: &mut Vec<(usize, usize)>, boulder_n: usize ){
    let (start_x, start_y) = boulders[boulder_n];
    if start_x == dish.len()-1{
        return;
    }
        
    let mut steps = 0;

    while start_x+steps < dish.len()-1 && dish[start_x+(steps+1)][start_y] == SPACE {
        steps+=1
    }
    if steps == 0{
        return;
    }
    dish[start_x][start_y] = SPACE;
    dish[start_x+steps][start_y] = BOULDER;
    boulders[boulder_n] = (start_x+steps, start_y);
}
fn move_boulder_east(dish:&mut Vec<Vec<u32>>, boulders: &mut Vec<(usize, usize)>, boulder_n: usize ){
    let (start_x, start_y) = boulders[boulder_n];
    if start_y == dish[start_x].len()-1{
        return;
    }
        
    let mut steps = 0;

    while start_y+steps < dish[start_x].len()-1 && dish[start_x][start_y+(steps+1)] == SPACE {
        steps+=1
    }
    if steps == 0{
        return;
    }
    dish[start_x][start_y] = SPACE;
    dish[start_x][start_y+steps] = BOULDER;
    boulders[boulder_n] = (start_x, start_y+steps);
}

fn print_dish(dish: &Vec<Vec<u32>>){
    println!();
        dish.iter().for_each(|v|{
            v.iter().for_each(|b|{
                match b {
                    0 => print!("#"),
                    1 => print!("O"),
                    2 => print!("."),
                    _ => panic!(),
                }
            });
            println!();
        });
}

pub fn part_two(input: &str) -> Option<u32> {
    return None;
    let mut boulders: Vec<(usize, usize)> = vec![];

    let mut dish:Vec<Vec<u32>> = input.lines().enumerate().map(|(x, line)|{
        // println!("{}", line);
        line.chars().enumerate().map(|(y, c)|{
            match c {
                'O' => {
                    boulders.push((x, y));
                    BOULDER
                },
                '#' => ROCK,
                '.' => SPACE,
                _ => panic!()
            }
        }).collect()
    }).collect();


    (0..1 as u64).for_each(|i|{
        print_dish(&dish);
        (0..boulders.len()).for_each(|i|{
            move_boulder_north(&mut dish, &mut boulders, i);
        });
        print_dish(&dish);
        (0..boulders.len()).for_each(|i|{
            move_boulder_west(&mut dish, &mut boulders, i);
        });
        print_dish(&dish);
        (0..boulders.len()).for_each(|i|{
            move_boulder_south(&mut dish, &mut boulders, i);
        });
        print_dish(&dish);
        (0..boulders.len()).for_each(|i|{
            move_boulder_east(&mut dish, &mut boulders, i);
        });
        print_dish(&dish);
    });


    let len = dish.len();
    Some(boulders.iter().map(|(x, _)| (len - x) as u32).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
