use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

struct Condition{
    c: char,
    operator: char,
    n: u32,
}
struct Check {
    condition: Option<Condition>,
    dest: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    // let colors_re = Regex::new(r"(?<variable>[a-z]+)(?<operator>[<>])").unwrap();
    // let is_invalid = colors_re.captures_iter(line).any( | it | {
    //     let color = &it["color"];
    let regex = Regex::new(r"(?<variable>[a-z]+)(?<operator>[<>])(?<number>[0-9]+):(?<destination>[a-zAR]+)").unwrap();

    let mut workflows: HashMap<String, Vec<Check>> = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty(){
            break
        }

        let mut parts = line.split("{");
        let part_one = parts.next().unwrap();

        let (part_two, _) = parts.next().map(|s| s.split_at(s.len()-1)).unwrap();
        
        let checks = part_two.split(',').map(|check|{
            if let Some(r_match) = regex.captures(check) {
                Check{
                    dest: r_match["destination"].to_string(),
                    condition: Some(Condition{
                        c: r_match["variable"].chars().next().unwrap(),
                        operator: r_match["operator"].chars().next().unwrap(),
                        n: r_match["number"].parse().unwrap()
                    })
                }
            }
            else{
                Check{
                    dest: check.to_string(),
                    condition: None
                }
            }
        }).collect::<Vec<_>>();

        workflows.insert(part_one.to_string(), checks);

        // println!("{}", part_two);
    }
    // println!();
    // println!("Now other lines");
    // println!();
    let regex2 = Regex::new(r"(?<variable>[a-z]+)=(?<number>[0-9]+)").unwrap();
    let variables = lines.map(|line|{
        regex2.captures_iter(line).map(|r_match|{
            (r_match["variable"].chars().next().unwrap(), r_match["number"].parse::<u32>().unwrap())
        }).collect::<HashMap<_, _>>()
    }).collect::<Vec<_>>();


    let mut result: HashMap<char, u32> = HashMap::new();

    for variable in variables{
        // println!("{:?}", variable);
        if pass_through_workflows(&workflows, &"in".to_string(), &variable){
            for (key, value) in variable{
                *result.entry(key).or_insert(0)+=value
            }
        }
        // println!();
    }

    // for (name, checks) in workflows{
    //     println!("{}", name);
    //     for c in checks{
    //         if let Some(co) = &c.condition{
    //             print!("  {}{}{}", co.c, co.operator, co.n);
    //         }
    //         print!(" {} ;", c.dest);
    //     }
    //     println!();
    // }


    Some(result.values().sum())
}

fn pass_through_workflows(workflows: &HashMap<String, Vec<Check>>, curr_workflow: &String, variable: &HashMap<char, u32>)->bool {
    // println!("{} ", curr_workflow);
    if curr_workflow == "R"{
        return false
    }

    if curr_workflow == "A"{
        return true
    }
    
    let workflow = workflows.get(curr_workflow).expect(&format!("What?? {}", curr_workflow));
    
    for c in workflow{
        if let Some(condition) = &c.condition {
            let operator = condition.operator;
            let curr_value = variable.get(&condition.c).unwrap();
            let cond = &condition.n;
            
            if operator == '<'  {
                if curr_value < cond{
                    // println!("  {} was < {}", curr_value,  cond);
                    return pass_through_workflows(workflows, &c.dest, variable);
                }else{
                    // println!("  {} was not < {}", curr_value,  cond);
                }
            }
            else if operator == '>' {
                if curr_value > cond{
                    // println!("  {} was > {}", curr_value,  cond);
                    return pass_through_workflows(workflows, &c.dest, variable);
                }else{
                    // println!("  {} was not > {}", curr_value,  cond);
                }
            }
        }else{
            // println!("Was no condition");
            return pass_through_workflows(workflows, &c.dest, variable); 
        }
    }
    panic!("Ehh...");
}

fn count(variable: &HashMap<char, (u32, u32)>)->u64{
    let mut result:u64 = 1;
    variable.values().for_each(|value|{
        result*=(value.1-value.0+1) as u64
    });
    return result
}

fn core_part_2(workflows: &HashMap<String, Vec<Check>>, curr_workflow: &String, variable: &mut HashMap<char, (u32, u32)>)->u64 {
    if curr_workflow == "R"{
        return 0
    }

    if curr_workflow == "A"{
        return count(variable);
    }

    let workflow = workflows.get(curr_workflow).expect(&format!("What?? {}", curr_workflow));
    let mut sum = 0;
    for check in workflow{
        if let Some(condition) = &check.condition {
            let (curr_from, curr_to) = variable.get(&condition.c).unwrap().to_owned();
            
            if condition.operator == '<'  {
                if curr_from < condition.n{
                    if curr_to < condition.n{
                        return sum + core_part_2(workflows, &check.dest, variable);
                    }else{
                        let mut new_variables = variable.clone();
                        new_variables.insert(condition.c, (curr_from, condition.n-1));
                        variable.insert(condition.c,(condition.n, curr_to));
                        sum += core_part_2(workflows, &check.dest, &mut new_variables);
                    }
                }
            }
            else if condition.operator == '>' {
                if curr_to > condition.n{
                    if curr_from > condition.n {
                        return sum + core_part_2(workflows, &check.dest, variable);
                    }else{
                        let mut new_variables = variable.clone();
                        variable.insert(condition.c, (curr_from, condition.n));
                        new_variables.insert(condition.c,(condition.n+1, curr_to));
                        sum += core_part_2(workflows, &check.dest, &mut new_variables);
                    }
                }
            }
        }else{
            // println!("Was no condition");
            return sum + core_part_2(workflows, &check.dest, variable); 
        }
    }
    panic!("Hva skjedde nå?");
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let regex = Regex::new(r"(?<variable>[a-z]+)(?<operator>[<>])(?<number>[0-9]+):(?<destination>[a-zAR]+)").unwrap();

    let mut workflows: HashMap<String, Vec<Check>> = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty(){
            break
        }

        let mut parts = line.split("{");
        let part_one = parts.next().unwrap();

        let (part_two, _) = parts.next().map(|s| s.split_at(s.len()-1)).unwrap();
        
        let checks = part_two.split(',').map(|check|{
            if let Some(r_match) = regex.captures(check) {
                Check{
                    dest: r_match["destination"].to_string(),
                    condition: Some(Condition{
                        c: r_match["variable"].chars().next().unwrap(),
                        operator: r_match["operator"].chars().next().unwrap(),
                        n: r_match["number"].parse().unwrap()
                    })
                }
            }
            else{
                Check{
                    dest: check.to_string(),
                    condition: None
                }
            }
        }).collect::<Vec<_>>();

        workflows.insert(part_one.to_string(), checks);

        // println!("{}", part_two);
    }
    
    let mut map:HashMap<char, (u32, u32)> = HashMap::new();
    map.insert('x', (1, 4000));
    map.insert('m', (1, 4000));
    map.insert('a', (1, 4000));
    map.insert('s', (1, 4000));

    let result = core_part_2(&workflows, &"in".to_string(), &mut map);

    Some(result)
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn test() {
        let colors_re = Regex::new(r"(?<variable>[a-z]+)(?<operator>[<>])(?<number>[0-9]+):(?<destination>[a-zAR])").unwrap();
        let string = "a>3333:R";

        if let Some(thing) = colors_re.captures(string){


            println!("{}", thing["variable"].to_string());
            println!("{}", thing["operator"].to_string());
            println!("{}", thing["number"].to_string());
            println!("{}", thing["destination"].to_string());

        }
    }
}
