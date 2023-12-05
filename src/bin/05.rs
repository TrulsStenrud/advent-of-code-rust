advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut seeds: Vec<u64>= vec![];

    let mut seed_to_soil: Vec<[u64;3]>= vec![];
    let mut soil_to_fertilizer: Vec<[u64;3]>= vec![];
    let mut fertilizer_to_water: Vec<[u64;3]>= vec![];
    let mut water_to_light: Vec<[u64;3]>= vec![];
    let mut light_to_temperature: Vec<[u64;3]>= vec![];
    let mut temperature_to_humidity: Vec<[u64;3]>= vec![];
    let mut humidity_to_location: Vec<[u64;3]>= vec![];
    
    let mut current = "";
    
    let mut word = None;
    input.lines().for_each(| line |{


        let mut words = line.split_whitespace();
        
        word = words.next();
        match word{
            Some("seeds:")=> {
                words.for_each(| word |{
                    // println!("Look at number: {}", word);
                    seeds.push(word.parse().expect("To be a seed number"));
                });
            }
            Some("seed-to-soil")|
            Some("soil-to-fertilizer")|
            Some("fertilizer-to-water")|
            Some("water-to-light")|
            Some("light-to-temperature")|
            Some("temperature-to-humidity")|
            Some("humidity-to-location")=> {
                current = word.unwrap();
            }
            None => {
                // Assuming empty line
            }
            _ => {
                // println!("Current line: {}", line);
                match current {
                    "seed-to-soil"=> {
                        seed_to_soil.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "soil-to-fertilizer"=> {
                        soil_to_fertilizer.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "fertilizer-to-water"=> {
                        fertilizer_to_water.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "water-to-light"=> {
                        water_to_light.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "light-to-temperature"=> {
                        light_to_temperature.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "temperature-to-humidity"=> {
                        temperature_to_humidity.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    "humidity-to-location"=> {
                        humidity_to_location.push(
                            [   word.unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap(), 
                                words.next().unwrap().parse().unwrap()]
                        )
                    }
                    _=> {
                        // println!("This was not supposed to happen {}", current)
                    }
                }
            }
        };

    });

    
    let mut next: Vec<u64>= vec![];
    [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ].iter().for_each(|map|{
        let mut used:Vec<&u64> =vec![]; 
        map.iter().for_each(|it|{
            let target = it[0];
            let source = it[1];
            let range = it[2];
            
            seeds.iter().for_each(|seed|{
                if seed >= &source && seed < &(source+range){
                    next.push(target + (seed - source));
                    used.push(seed);
                }
                
            })
        });

        seeds.iter().for_each(|seed|{
            if ! used.contains(&seed){
                next.push(*seed);
            }
        });
    
        seeds = next.clone();
        next = vec![];
    });
    
    
    Some(*seeds.iter().min().unwrap() as u32)
}


pub fn part_two(input: &str) -> Option<u32> {
    
    let mut seeds: Vec<[u64;2]>= vec![];
    let mut maps: Vec<Vec<[u64;3]>> = vec![];

    let mut lines = input.lines().enumerate();
    let mut first_line = lines.next().unwrap().1.split_whitespace().skip(1);
    
    while let (Some(from), Some(length)) = (first_line.next(), first_line.next())   {
        let start = from.parse().unwrap();
        seeds.push([start, start + length.parse::<u64>().unwrap()]);
    }

    lines.for_each(| (_, line) |{
        if let Some(first_char) = line.chars().next() {
            if first_char.is_alphabetic(){
            maps.push(vec![]);
        } else if first_char.is_numeric(){
            let mut thing = line.split_whitespace();
            maps.last_mut().unwrap().push([thing.next().unwrap().parse().unwrap(), 
            thing.next().unwrap().parse().unwrap(), 
            thing.next().unwrap().parse().unwrap()]);
        }}
    });


    let mut next: Vec<[u64;2]>= vec![];
    
    maps.iter().for_each(|map|{
        seeds.iter().for_each(|seed|{
            next.append(&mut map_to_next(seed, map));
        });

        seeds = next.clone();
        next = vec![];
    });
    
    Some(seeds.iter().map(|seed| seed[0]).min().unwrap() as u32)
}

fn map_to_next(input: &[u64;2], maps: &Vec<[u64;3]>) -> Vec<[u64;2]>{
    let mut filtered:Vec<[u64;2]> = vec![];
        
    let start = input[0];
    let stop = input[1];
    maps.iter().for_each(|it|{
        let target = it[0];
        let source = it[1];
        let range = it[2];
            if start >= source && stop <= source + range{
                // hele er inne i map 
                filtered.push([target + (start - source), target + stop - source]);
            }else if start >= source && start < source + range && stop > source + range{
                // starter inne i map, slutter utenfor
                filtered.push([target + (start-source), target+range]);
                filtered.append(&mut map_to_next(&[source+range, stop], maps));
                
            } else if start < source && stop > source && stop < source+range {
                // starter før, slutter inne i map
                filtered.append(&mut map_to_next(&[start, source], maps));
                filtered.push([target, target + (stop - source)]);                        
            }
            else if start < source && stop > source+range {
                // starter før, slutter etter
                filtered.append(&mut map_to_next(&[start, source], maps));
                filtered.append(&mut map_to_next(&[source+range, stop], maps));
                filtered.push([target, target+range]);
            }
    });
    
    if filtered.is_empty() {filtered.push(*input)}
    
    filtered
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
