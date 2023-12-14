

use cached::proc_macro::cached;

advent_of_code::solution!(12);

fn check(input: &str, thing: &Vec<u64>)->bool{
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

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input.lines().map(|line|{
            let mut thing = line.split_whitespace();
            
            let part_one = thing.next().unwrap();
            let part_two = thing.next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

            do_this_shit(&part_one.to_string(), &part_two)
        }).sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input.lines().map(|line|{
            let mut thing = line.split_whitespace();
            
            let _part_one = thing.next().unwrap();
            let mut _part_two = thing.next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

            let part_one = format!("{}?{}?{}?{}?{}", _part_one,_part_one,_part_one,_part_one,_part_one);
            let mut  part_two:Vec<u64> = vec![];
            for _ in 0..5{
                _part_two.iter().for_each(|it|{
                    part_two.push(*it);
                });
            }

            // println!("Starting {}", line);
            
            do_this_shit(&part_one, &part_two)
        }).sum()
    )
}



fn do_this_shit(input: &String, thing: &Vec<u64>) -> u64{   
    //  println!("{}, {}", input, thing.iter().map(|it| it.to_string()).collect::<Vec<_>>().join(", "));
     
    //  println!("Starting {}", input);
     let problems = input
     .split(".")
     .filter(|it| !it.is_empty())
     .map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();


     
     not_sure_yet(problems, thing.clone())
}

fn all_is_hash(word: &Vec<char>)->bool{
    word.iter().all(|c| c == &'#')
}

fn count_init_hash(word: &Vec<char>)->u64{
    let mut count = 0;
    while count < word.len() && word[count] == '#' {
        count+=1;
    }
    count as u64
}

fn remove_group(groups: &Vec<u64>)->Vec<u64>{
    let mut result = groups.clone();
        result.remove(0);
        result
}

fn count_q_from_i(problem: &Vec<char>, start_i: usize)->u64{
    let mut q_count = 0;
        let mut i = start_i;
        while i < problem.len() && problem[i] == '?' {
            q_count+=1;
            i+=1
        }
        q_count
}

#[cached]
fn not_sure_yet(problems: Vec<Vec<char>>, groups: Vec<u64>)->u64{  
    if problems.is_empty() && groups.is_empty(){
        return 1;
    }
    
    if groups.is_empty() {
        if problems.iter().all(|it| it.iter().all(|it| it == &'?')){
            return 1
        }else {
            return 0
        }
    }
    
    if problems.is_empty(){
        return 0
    }


    let next_group = groups[0];
    let next_problem = & problems[0];
    if next_problem.is_empty(){
        let mut new_problems = problems.clone();
        new_problems.remove(0);
        return not_sure_yet(new_problems, groups);
    }

    // if all hashtags and same size as group
    if all_is_hash(next_problem){
        if next_group == next_problem.len() as u64 {
            let removed_group = remove_group(&groups);
            let mut removed_problem = problems.clone();
            removed_problem.remove(0);
            return not_sure_yet(removed_problem, removed_group);
        }
        else {
            return 0;
        }
    }

    if next_problem[0] == '#'{
        let h_count = count_init_hash(&next_problem);
        if h_count > next_group{
            return 0
        }
        if h_count == next_group{
            let removed_group = remove_group(&groups);
            // remove hashes plus next, which should be a questionmark
            // basicly making it a "."
            let mut new_problem = problems.clone();
            
            new_problem[0].drain(0..((h_count+1) as usize));
            return not_sure_yet(new_problem, removed_group)
        }
        
        let q_count = count_q_from_i(next_problem, h_count as usize);
        
        if q_count > 0{
            let mut problem_with_new_hash = problems.clone();
            problem_with_new_hash[0][h_count as usize] = '#';
            return not_sure_yet(problem_with_new_hash, groups)
        }
        
        panic!("Hash {:?} {:?}, {}", next_problem, next_group, q_count);
    }

    if next_problem[0] == '?'{
            let mut problem_with_new_dot = problems.clone();
            problem_with_new_dot[0].drain(0..1);
            let mut problem_with_new_hash = problems.clone();
            problem_with_new_hash[0][0] = '#';
            return not_sure_yet(problem_with_new_dot, groups.clone()) 
            + not_sure_yet(problem_with_new_hash, groups)
    }

    panic!("Fuck {:?}", next_problem);
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