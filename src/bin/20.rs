use std::{collections::HashMap, os::unix::raw::off_t, any::{Any, TypeId}};

use queues::{Queue, IsQueue};
use regex::Regex;

advent_of_code::solution!(20);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Pulse {
    HIGH,
    LOW
}

trait Module: AToAny{
    fn push(&mut self, pulse: Pulse, source: String);
    fn get_outputs(&self)->&Vec<String>;
    fn register_connections(&mut self, source: &String);
    fn process(&mut self)->Vec<(String, Pulse)>;
    fn prepare_for_process(&mut self);
    fn has_input(&self) -> bool;
}
pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct FlipFlip{
    name: String,
    outputs: Vec<String>,
    inputs: Queue<(Pulse, String)>,
    to_be_processed: Queue<(Pulse, String)>,
    on:bool,
}
struct Conjuction{
    name: String,
    outputs: Vec<String>,
    connections: HashMap<String, Pulse>,
    inputs: Queue<(Pulse, String)>,
    to_be_processed: Queue<(Pulse, String)>,
}
struct Broadcaster{
    name: String,
    outputs: Vec<String>,
    inputs: Queue<(Pulse, String)>,
    to_be_processed: Queue<(Pulse, String)>,
}
struct Output{
    highs: u32,
    lows: u32,
    inputs: Queue<(Pulse, String)>,
    fake_outputs: Vec<String>,
    to_be_processed: Queue<(Pulse, String)>,
}

impl Module for FlipFlip{
    fn push(&mut self, pulse: Pulse, source: String) {
        let _ = self.inputs.add((pulse, source));
    }

    fn process(&mut self)->Vec<(String, Pulse)> {
        let mut result: Vec<(String, Pulse)> = vec![];
        while let Ok((pulse, _)) = self.to_be_processed.remove(){
            if pulse == Pulse::LOW{
                self.on = !self.on;

                self.outputs.iter_mut().for_each(|out|{
                    result.push((out.to_string(), if self.on {Pulse::HIGH} else {Pulse::LOW}));                    
                })
            }
        }
        result
    }

    fn prepare_for_process(&mut self) {
        while let Ok(thing) = self.inputs.remove(){
            let _ = self.to_be_processed.add(thing);
        }
    }

    fn get_outputs(&self)->&Vec<String> {
        &self.outputs
    }

    fn register_connections(&mut self, source: &String) {
        
    }

    fn has_input(&self)-> bool {
        self.inputs.size() > 0
    }
}

impl Module for Conjuction{
    fn push(&mut self, pulse: Pulse, source: String) {
        let _ = self.inputs.add((pulse, source));
    }

    fn process(&mut self)-> Vec<(String, Pulse)>{
        let mut result : Vec<(String, Pulse)> = vec![];
        while let Ok((pulse, source)) = self.to_be_processed.remove(){
            self.connections.insert(source, pulse);
            let new_pulse = if self.connections.values().all(|pulse| pulse == &Pulse::HIGH) {Pulse::LOW} else {Pulse::HIGH};

            self.outputs.iter().for_each(|out|{
                result.push((out.to_string(), new_pulse));
            })
        }
        result
    }

    fn get_outputs(&self)->&Vec<String> {
        &self.outputs
    }

    fn register_connections(&mut self, source: &String) {
        self.connections.insert(source.to_string(), Pulse::LOW);
    }

    fn prepare_for_process(&mut self) {
        while let Ok(thing) = self.inputs.remove(){
            let _ = self.to_be_processed.add(thing);
        }
    }
    fn has_input(&self)-> bool {
        self.inputs.size() > 0
    }
}

impl Module for Broadcaster{
    fn push(&mut self, pulse: Pulse, source: String) {
        let _ = self.inputs.add((pulse, source));
    }

    fn process(&mut self)->Vec<(String, Pulse)> {
        let mut result: Vec<(String, Pulse)> = vec![];
        while let Ok((pulse, _)) = self.to_be_processed.remove(){
            self.outputs.iter().for_each(|out|{
                result.push((out.to_string(), pulse));
            })
        }
        result
    }
    fn get_outputs(&self)->&Vec<String> {
        &self.outputs
    }

    fn register_connections(&mut self, source: &String) {
        
    }

    fn prepare_for_process(&mut self) {
        while let Ok(thing) = self.inputs.remove(){
            let _ = self.to_be_processed.add(thing);
        }
    }
    fn has_input(&self)-> bool {
        self.inputs.size() > 0
    }
}

impl Module for Output{
    fn push(&mut self, pulse: Pulse, source: String) {
        let _ = self.inputs.add((pulse, source));
    }

    fn process(&mut self)-> Vec<(String, Pulse)> {
        while let Ok((pulse, _)) = self.to_be_processed.remove(){
            match pulse {
                Pulse::HIGH => self.highs += 1,
                Pulse::LOW => self.lows += 1,
            }
        }
        vec![]
    }

    fn get_outputs(&self)->&Vec<String> {
        &self.fake_outputs
    }

    fn register_connections(&mut self, source: &String) {
        
    }

    fn prepare_for_process(&mut self) {
        while let Ok(thing) = self.inputs.remove(){
            let _ = self.to_be_processed.add(thing);
        }
    }
    fn has_input(&self)-> bool {
        self.inputs.size() > 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let re:Regex = Regex::new(r"(?<type>[%&]?)(?<source>[a-z]+) -> (?<destination>[a-z,\s]+)").unwrap();

    // % low pulse -> flip state and send low or high based on state
    // % high pulse -> ignore

    // & sends low if it remembers high for all inputs, otherwise high
    // & default remembers low

    // broadcaster, sends out to everyone if it receives one pulse
    

    let mut map:HashMap<String, Box<dyn Module>> = HashMap::new();
    
    for line in input.lines(){
        if let Some(re_match) = re.captures(line) {
            let t = &re_match["type"];
            let source = &re_match["source"];
            let destination = re_match["destination"].split(",").map(|it| it.trim().to_string()).collect::<Vec<_>>();

            if source == "broadcaster"{
                // println!("Was Broadcaster");
                map.insert(source.to_string(), Box::new(Broadcaster{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new()
                }));
            }
            else if t == "&"{
                // println!("Was Conjuction");
                map.insert(source.to_string(), Box::new(Conjuction{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new(),
                    connections: HashMap::new(),
                }));
            }
            else if t == "%"{
                // println!("Was FlipFlip");
                map.insert(source.to_string(), Box::new(FlipFlip{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new(),
                    on: false,
                }));
            }
            else {
                panic!("What about: {}, {}", t, source)
            }
        }
        else {
            panic!("Didnt match: {}", line);
        }
    }
    
    
    for name in map.keys().map(|it|it.to_string()).collect::<Vec<_>>(){
        for dest in map.get(&name).unwrap().get_outputs().to_owned().iter(){
            if let Some(dest) = map.get_mut(dest){
                dest.register_connections(&name);
            }else{
                // println!("{} is the output", dest);
            }
        }
    }

    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000{
        // println!("New round");
        map.get_mut("broadcaster").unwrap().push(Pulse::LOW, "button".to_string());
        lows+=1;

        while map.values().any(|it| it.has_input()) {
            map.values_mut().for_each(|it| it.prepare_for_process());
            
             map.iter_mut().map(|(name, it)| {
                 (name.to_string(), it.process())
            }).collect::<Vec<_>>().iter().for_each(|(source, output)|{
                output.iter().for_each(|(dest, pulse)|{
                    
                    match pulse {
                        Pulse::HIGH => highs+=1,
                        Pulse::LOW => lows+=1,
                    }
                    if let Some(next) = map.get_mut(dest){
                        next.push(*pulse, source.to_string());
                    }
                });
            });
        }
        // println!();
    }

    // println!("{} highs", highs);
    // println!("{} lows", lows);
    Some(highs * lows)
}

pub fn part_two(input: &str) -> Option<u32> {
    

    let re:Regex = Regex::new(r"(?<type>[%&]?)(?<source>[a-z]+) -> (?<destination>[a-z,\s]+)").unwrap();

    // % low pulse -> flip state and send low or high based on state
    // % high pulse -> ignore

    // & sends low if it remembers high for all inputs, otherwise high
    // & default remembers low

    // broadcaster, sends out to everyone if it receives one pulse
    

    let mut map:HashMap<String, Box<dyn Module>> = HashMap::new();
    
    for line in input.lines(){
        if let Some(re_match) = re.captures(line) {
            let t = &re_match["type"];
            let source = &re_match["source"];
            let destination = re_match["destination"].split(",").map(|it| it.trim().to_string()).collect::<Vec<_>>();

            if source == "broadcaster"{
                // println!("Was Broadcaster");
                map.insert(source.to_string(), Box::new(Broadcaster{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new()
                }));
            }
            else if t == "&"{
                // println!("Was Conjuction");
                map.insert(source.to_string(), Box::new(Conjuction{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new(),
                    connections: HashMap::new(),
                }));
            }
            else if t == "%"{
                // println!("Was FlipFlip");
                map.insert(source.to_string(), Box::new(FlipFlip{
                    name: source.to_string(),
                    outputs: destination,
                    inputs: Queue::new(),
                    to_be_processed: Queue::new(),
                    on: false,
                }));
            }
            else {
                panic!("What about: {}, {}", t, source)
            }
        }
        else {
            panic!("Didnt match: {}", line);
        }
    }
    
    
    for name in map.keys().map(|it|it.to_string()).collect::<Vec<_>>(){
        for dest in map.get(&name).unwrap().get_outputs().to_owned().iter(){
            if let Some(dest) = map.get_mut(dest){
                dest.register_connections(&name);
            }else{
                // println!("{} is the output", dest);
            }
        }
    }

    let mut count = 0;
    let mut finished = false;
    while !finished {
        // println!("New round");
        map.get_mut("broadcaster").unwrap().push(Pulse::LOW, "button".to_string());
        count+=1;

        while map.values().any(|it| it.has_input()) {
            map.values_mut().for_each(|it| it.prepare_for_process());
            
             map.iter_mut().map(|(name, it)| {
                 (name.to_string(), it.process())
            }).collect::<Vec<_>>().iter().for_each(|(source, output)|{
                output.iter().for_each(|(dest, pulse)|{
                    
                    if dest == "rx"{
                        println!("Hit rx {:?}", pulse);
                    }
                    if dest == "rx" && pulse == &Pulse::LOW{
                        finished = true
                    }

                    if let Some(next) = map.get_mut(dest){
                        next.push(*pulse, source.to_string());
                    }
                });
            });
        }
        // println!();
    }
    Some(count)

    // println!("{} highs", highs);
    // println!("{} lows", lows);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_regex() {
        
        let re:Regex = Regex::new(r"(?<type>[%&]?)(?<source>[a-z]+) -> (?<destination>[a-z,\s]+)").unwrap();

        let a = "broadcaster -> a, b, c";

        let res = re.captures(a).unwrap();

        println!("{}", res["type"].to_string());
        println!("{}", res["source"].to_string());
        println!("{}", res["destination"].to_string());

    }
}
