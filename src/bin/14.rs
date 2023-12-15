use std::collections::{HashSet, HashMap};

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
fn move_it_north(dish:&mut Vec<Vec<u32>>, boulder: (usize, usize) ){
    let (start_x, start_y) = boulder;
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

fn move_it_west(dish:&mut Vec<Vec<u32>>, boulder: (usize, usize)){
    let (start_x, start_y) = boulder;
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
fn move_it_south(dish:&mut Vec<Vec<u32>>, boulder: (usize, usize)){
    let (start_x, start_y) = boulder;
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
fn move_it_east(dish:&mut Vec<Vec<u32>>, boulder: (usize, usize)){
    let (start_x, start_y) = boulder;
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


    let mut prev:Vec<u32> = vec![];

    let mut i = 0;
    let result = loop {
        // print_dish(&dish);
        move_north(&mut boulders, &mut dish);
        // print_dish(&dish);
        move_west(&mut boulders, &mut dish);
        // print_dish(&dish);
        move_south(&mut boulders, &mut dish);
        // print_dish(&dish);
        move_east(&mut boulders, &mut dish);
        // print_dish(&dish);
        let len = dish.len();
        let curr_value = (0..dish.len()).map(|x| (0..dish[x].len()).map(|y| {
            if dish[x][y] == BOULDER{
                len - x
            }
            else {
                0
            }
        }).sum::<usize>()).sum::<usize>() as u32;
        // println!("{}", curr_value);
        prev.push(curr_value);
        if i > 40{
            
            if let Some(a) = find_repeating(&prev){
                let t = 1000000000-a.0-1;
                break  a.1[t % a.1.len()];
            }else{
                // println!("Did not find");
            }
        }

        i+=1;
    };


    Some(result)
    // let len = dish.len();
    // Some(boulders.iter().map(|(x, _)| (len - x) as u32).sum())
}

fn move_east(boulders: &mut Vec<(usize, usize)>, dish: &mut Vec<Vec<u32>>) {
    // boulders.sort_by_key(|it| -(it.1 as i32));
    // (0..boulders.len()).for_each(|i|{
    //     move_boulder_east(dish, boulders, i);
    // });
    for j in (0..dish[0].len()).rev(){
        for i in 0..dish.len(){
            if dish[i][j] == BOULDER{
                move_it_east(dish, (i, j));
            }
        }
    }
}

fn move_south(boulders: &mut Vec<(usize, usize)>, dish: &mut Vec<Vec<u32>>) {
    // boulders.sort_by_key(|it| -(it.0 as i32));
    // (0..boulders.len()).for_each(|i|{
    //     move_boulder_south(dish, boulders, i);
    // });
    for i in (0..dish.len()).rev(){
        for j in 0..dish[i].len(){
            if dish[i][j] == BOULDER{
                move_it_south(dish, (i, j));
            }
        }
    }
}

fn move_west(boulders: &mut Vec<(usize, usize)>, dish: &mut Vec<Vec<u32>>) {
    for j in 0..dish[0].len(){
        for i in 0..dish.len(){
            if dish[i][j] == BOULDER{
                move_it_west(dish, (i, j));
            }
        }
    }
}

fn move_north(boulders: &mut Vec<(usize, usize)>, dish: &mut Vec<Vec<u32>>) {
    for i in 0..dish.len(){
        for j in 0..dish[i].len(){
            if dish[i][j] == BOULDER{
                move_it_north(dish, (i, j));
            }
        }
    }
    // boulders.sort_by_key(|it| it.0);
    // (0..boulders.len()).for_each(|i|{
    //     move_boulder_north(dish, boulders, i);
    // });
}

fn find_repeating(v: &Vec<u32>) -> Option<(usize,Vec<u32>)> {

    let len = v.len();
    let mut n = 10;
    while n < v.len()/2 {
        // println!("Checking length {}", n);
        let mut res:Vec<u32> = vec![];
        for i in (0..n).rev(){
            // println!("Comparing {} and {}", v[len - 1 - i], v[len-i-1-n]);
            if v[len - 1 - i] == v[len-1-i-n]{
                res.push(v[len - 1 - i]);
            }
            
        }
        if res.len() == n{
            return Some((len - n, res));
        }else{
            // println!("Length was {}, not {}", res.len(), n);
        }
        // println!();
        n+=1;
    }

    return None
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

        // let thing = find_repeating(vec![1, 1, 1, 4, 3, 2, 4, 5, 6, 5, 4, 34, 4, 5, 6, 5, 4, 4, 5, 56, 54, 4, 4, 1, 34, 4, 5, 6, 5, 4, 4, 5, 56, 54, 4, 4, 1]).unwrap();
        // println!("Found at {}", thing.0);
        // thing.1.iter().for_each(|it|{
        //     println!("{}", it);
        // });
    }
}
