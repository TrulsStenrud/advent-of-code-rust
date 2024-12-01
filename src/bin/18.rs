use std::{os::macos::raw::stat, collections::{HashSet, HashMap}};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = input.lines().map(|line|{
        let mut split = line.split_whitespace();

        let direction = split.next().unwrap().chars().next().unwrap();
        let lenght: u32 = split.next().unwrap().parse().unwrap();
        (direction, lenght)
    }).collect::<Vec<_>>();

    let mut sum = 0;
    let a = instructions.iter().filter(|(a, _)| a == &'U' || a == &'D').map(|(a, b)|{
        if a == &'D'{
            sum += b;
        }else{
            sum -= b;
        }
        sum
    }).max().unwrap();
    println!("{}", a);
    let mut map: HashMap<usize, Vec<(usize, char)>> = HashMap::default();
    
    map.entry(0).or_insert(vec![]).push((0, 'F'));
    
    let mut cur_pos:(usize, usize) = (0,0);
    let mut cur_dir = instructions[0].0;
    if instructions[0].0  == 'R'{
        cur_pos.1 += instructions[0].1 as usize;
    }else{
        cur_pos.0 += instructions[0].1 as usize;
    }

    instructions.iter().skip(1).for_each(|(direction, lenght)|{
        let char = match (cur_dir, direction) {
            ('R', 'U') |
            ('D', 'L') => 'J',
            ('D', 'R') |
            ('L', 'U') => 'L',
            ('U', 'L') |
            ('R', 'D') => '7',
            ('L', 'D') |
            ('U', 'R') => 'F',
            _=> panic!("Whaaaat {}, {}", cur_dir, direction)
        };
        
        map.entry(cur_pos.0).or_insert(vec![]).push((cur_pos.1, char));
        match direction {
            'R'=> cur_pos.1 += *lenght as usize,
            'L'=> cur_pos.1 -= *lenght as usize,
            'U'=> cur_pos.0 -= *lenght as usize,
            'D'=> cur_pos.0 += *lenght as usize,
            _=> panic!("Whaaaat {}", direction)
        }
        cur_dir = *direction;
    });

    map.iter_mut().for_each(|(_, it)| it.sort_by_key(|it|it.0));


    let mut state: HashMap<usize, bool> = HashMap::new();


    // println!("Nå starter dette");
    let mut sorted_map = map.iter().collect::<Vec<_>>();
    
    sorted_map.sort_by_key(|(i, _)|*i);

    let mut areal = 0;
    let mut prev_x: usize = 0;
    for (x, row) in sorted_map.iter(){
        println!("Processing line {}", x);
        // println!("{:?}", row);

        let mut _b = state.iter().filter(|it| it.1 == &true).map(|(a, b)| (*a, *b)).collect::<Vec<_>>();

        _b.sort_by_key(|it| it.0);
        let b = (1.._b.len()).step_by(2).map(|i|{
            (_b[i-1].0, _b[i].0)
        }).collect::<Vec<_>>();

        let sum_b = b.iter().map(|(from, to)| {
            to-from + 1
        }).sum::<usize>();
        println!("Adding from old {}", sum_b*(**x - prev_x));
        areal+=sum_b*(**x - prev_x);

        row.iter().for_each(|c|{
            state.insert(c.0, !(state.get(&c.0).or(Some(&false)).unwrap() == &true));
        });
        
        let mut a = state.iter().filter(|it| it.1 == &true).collect::<Vec<_>>();
        a.sort_by_key(|it| it.0);
        let sum_a = (1..a.len()).step_by(2).map(|i|{
            let mut start = *a[i-1].0;
            let mut stop = *a[i].0;
            if let Some(new_start) = b.iter().find(|(from, to)| from < &start && to > &start).map(|it| it.0){
                // println!("Raplasing start {} with {}", start, new_start);
                start = new_start;
            }
            if let Some(new_stop) = b.iter().find(|(from, to)| to > &stop && from < &stop).map(|it| it.1){
                // println!("Raplasing stop {} with {}", stop, new_stop);
                stop = new_stop;
            }
            
            stop - start + 1
            
        }).sum::<usize>();
        
        
        // println!("a {:?}", a);
        // println!("b {:?}", b);
        if sum_a > sum_b{
            println!("Adding {}", sum_a - sum_b);
            areal += sum_a - sum_b;
        }
        // if a.is_empty(){
        //     println!("Was empty, added {}",  b.iter().map(|(from, to)| {
        //         to-from + 1
        //     }).sum::<usize>());
        //     areal+= b.iter().map(|(from, to)| {
        //         to-from + 1
        //     }).sum::<usize>()
        // }
        prev_x = **x;
        // println!();
    }
    
    // instructions.iter().for_each(|(direction, length)|{
        
    // });

    Some(areal as u64)
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
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test() {
        let mut state: HashMap<usize, bool> = HashMap::new();

        state.insert(2, !(state.get(&2).or(Some(&false)).unwrap() == &true));

        println!("{}", state.get(&2).unwrap());
        state.insert(2, !(state.get(&2).or(Some(&false)).unwrap() == &true));
        println!("{}", state.get(&2).unwrap());
        state.insert(2, !(state.get(&2).or(Some(&false)).unwrap() == &true));
        println!("{}", state.get(&2).unwrap());

    }
}
