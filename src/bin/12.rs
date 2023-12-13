use std::collections::binary_heap::Iter;

advent_of_code::solution!(12);

fn check(input: &str, thing: &Vec<u32>)->bool{
    // if input.contains('?'){
    //     panic!();
    // }

    let a = input.split('.').filter(|x| !x.is_empty()).collect::<Vec<_>>();
    
    if a.len() != thing.len(){
        return false;
    }

    for i in 0..a.len(){
        if a[i].len() != thing[i] as usize{
            return false
        }
    }

    true
}

fn get_iterations(input: &str) -> Vec<String> {
    let n_q = input.chars().filter(|c| c == &'?').count();

    let permutations = boolean_permutations(n_q);

    permutations.iter().map(|it|{
        let mut result:String = String::new();
        let mut index = 0;

        input.chars().for_each(|c|{
            if c == '?'{
                if it[index] {
                    result.push('#');
                }else{
                    result.push('.');
                }
                index+=1
            } else{
                result.push(c);
            }
        });

        result
    }).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|line|{
            let mut thing = line.split_whitespace();
            
            let part_one = thing.next().unwrap();
            let part_two = thing.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

            do_this_shit(&part_one.to_string(), &part_two)
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|line|{
            let mut thing = line.split_whitespace();
            
            let _part_one = thing.next().unwrap();
            let mut _part_two = thing.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

            let part_one = format!("{}?{}?{}?{}?{}", _part_one,_part_one,_part_one,_part_one,_part_one);
            let mut  part_two:Vec<u32> = vec![];
            for _ in 0..5{
                _part_two.iter().for_each(|it|{
                    part_two.push(*it);
                });
            }

            println!("Starting {}", line);
            
            do_this_shit(&part_one, &part_two)
        }).sum()
    )
}

fn do_this_shit(input: &String, thing: &Vec<u32>) -> u32{
     get_iterations(&input).iter().filter(|it|{
                check(it, &thing)
            }).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }


    #[test]
    fn test_part_checker() {
        assert_eq!(check("###.....##.###", &vec![3, 2, 3]), true);
        assert_eq!(check("###..#..##.###", &vec![3, 2, 3]), false);
        assert_eq!(check("....###", &vec![3, ]), true);
        assert_eq!(check("###...###..##.###", &vec![3, 3, 2, 3]), true);
    }


    #[test]
    fn test_something() {
        let mut  a = vec![1, 2, 3, 4, 5];
        let mut  b = vec![1, 2, 3, 4, 5];

        println!("{}, {}", a.len(), b.len());
        b.append(&mut a);
        println!("{}, {}", a.len(), b.len());
    }
}
fn boolean_permutations_helper(n: usize, prefix: &mut Vec<bool>, result: &mut Vec<Vec<bool>>) {
    if n == 0 {
        result.push(prefix.clone());
    } else {
        prefix.push(false);
        boolean_permutations_helper(n - 1, prefix, result);
        prefix.pop();

        prefix.push(true);
        boolean_permutations_helper(n - 1, prefix, result);
        prefix.pop();
    }
}

fn boolean_permutations(n: usize) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    let mut prefix = Vec::new();
    boolean_permutations_helper(n, &mut prefix, &mut result);
    result
}